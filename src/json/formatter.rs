use std::io::Stdout;
use std::io::Write;

use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;
use tracing::{Event, Metadata, Subscriber};
use tracing_subscriber::{
    fmt::MakeWriter,
    layer::Context,
    registry::{LookupSpan, SpanRef},
    Layer,
};

use crate::json::storage::PrimaJsonVisitor;
use crate::subscriber::{ContextInfo, EventFormatter};

pub struct PrimaFormattingLayer<'writer, W: MakeWriter<'writer>, F: EventFormatter> {
    make_writer: &'writer W,
    app_name: String,
    country: String,
    environment: String,
    formatter: F,
}

/// Build a [`PrimaFormattingLayer`] layer with [`DefaultEventFormatter`] as format
/// and [`std::io::Stdout`] as output
pub fn layer<'writer>(
    app_name: String,
    country: String,
    environment: String,
) -> PrimaFormattingLayer<'writer, impl Fn() -> Stdout, DefaultEventFormatter> {
    PrimaFormattingLayer::new(
        app_name,
        country,
        environment,
        &std::io::stdout,
        DefaultEventFormatter,
    )
}

impl<'writer, W: MakeWriter<'writer>, F: EventFormatter> PrimaFormattingLayer<'writer, W, F> {
    pub(crate) fn new(
        app_name: String,
        country: String,
        environment: String,
        make_writer: &'writer W,
        formatter: F,
    ) -> Self {
        Self {
            make_writer,
            app_name,
            country,
            environment,
            formatter,
        }
    }

    pub fn with_formatter<A: EventFormatter>(
        self,
        formatter: A,
    ) -> PrimaFormattingLayer<'writer, W, A> {
        PrimaFormattingLayer::new(
            self.app_name,
            self.country,
            self.environment,
            self.make_writer,
            formatter,
        )
    }

    fn emit(&self, mut buffer: Vec<u8>) -> Result<(), std::io::Error> {
        buffer.write_all(b"\n")?;
        self.make_writer.make_writer().write_all(&buffer)
    }

    fn format_event<S>(
        &self,
        event: &Event<'_>,
        ctx: Context<'_, S>,
    ) -> Result<Vec<u8>, std::io::Error>
    where
        S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    {
        self.formatter.format_event(
            event,
            ctx,
            ContextInfo {
                app_name: self.app_name.as_str(),
                country: self.country.as_str(),
                environment: self.environment.as_str(),
            },
        )
    }
}

impl<S, W, F: 'static> Layer<S> for PrimaFormattingLayer<'static, W, F>
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    W: MakeWriter<'static>,
    F: EventFormatter,
{
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        if let Ok(serialized) = self.format_event(event, ctx) {
            let _ = self.emit(serialized);
        }
    }
}

pub struct DefaultEventFormatter;

impl EventFormatter for DefaultEventFormatter {
    fn format_event<S>(
        &self,
        event: &Event<'_>,
        ctx: Context<'_, S>,
        info: ContextInfo<'_>,
    ) -> Result<Vec<u8>, std::io::Error>
    where
        S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    {
        let metadata = event.metadata();
        let mut buffer = Vec::new();
        let mut serializer = serde_json::Serializer::new(&mut buffer);
        let mut map_serializer = serializer.serialize_map(None)?;

        map_serializer.serialize_entry("timestamp", &chrono::Utc::now())?;
        map_serializer.serialize_entry(
            "level",
            metadata.level().to_string().to_lowercase().as_str(),
        )?;
        map_serializer.serialize_entry("country", info.country())?;
        map_serializer.serialize_entry("environment", info.environment())?;
        map_serializer.serialize_entry("type", info.app_name())?;

        let mut visitor = PrimaJsonVisitor::default();
        event.record(&mut visitor);

        map_serializer.serialize_entry("message", &visitor.fields().get("message"))?;

        map_serializer.serialize_entry(
            "metadata",
            &MetadataSerializer {
                ctx: &ctx,
                metadata,
                visitor: &visitor,
                environment: info.environment(),
            },
        )?;

        // Adds support for correlating logs and traces on datadog
        // In order for Datadog to be able to correlate the logs with the traces we need to insert `dd.trace_id` and `dd.span_id` at root level
        // https://docs.datadoghq.com/tracing/connect_logs_and_traces/opentelemetry/
        #[cfg(feature = "datadog")]
        {
            use opentelemetry::trace::TraceContextExt;
            use std::collections::HashMap;
            use tracing_opentelemetry::OtelData;

            if let Some(current_span) = ctx.current_span().id().and_then(|id| ctx.span(id)) {
                let ext = current_span.extensions();
                if let Some(otel_data) = ext.get::<OtelData>() {
                    let builder = &otel_data.builder;

                    let parent_span = &otel_data.parent_cx.span();
                    let parent_span_ctx = parent_span.span_context();

                    let span_id = builder.span_id.unwrap_or_else(|| parent_span_ctx.span_id());
                    let trace_id = builder
                        .trace_id
                        .unwrap_or_else(|| parent_span_ctx.trace_id());

                    // Datadog trace and span IDs need to be 64-bit unsigned integers
                    let trace_id = u128::from_be_bytes(trace_id.to_bytes()) as u64;
                    let span_id = u64::from_be_bytes(span_id.to_bytes());

                    let mut dd = HashMap::new();
                    dd.insert("trace_id", trace_id);
                    dd.insert("span_id", span_id);

                    map_serializer.serialize_entry("dd", &dd)?;
                }
            }
        }

        map_serializer.end()?;

        Ok(buffer)
    }
}

pub struct MetadataSerializer<'a, S>
where
    S: Subscriber + tracing_subscriber::registry::LookupSpan<'a>,
{
    ctx: &'a Context<'a, S>,
    metadata: &'a Metadata<'a>,
    visitor: &'a PrimaJsonVisitor<'a>,
    environment: &'a str,
}

impl<'a, Sub> Serialize for MetadataSerializer<'a, Sub>
where
    Sub: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map_serializer = serializer.serialize_map(None)?;

        map_serializer.serialize_entry("environment", self.environment)?;
        map_serializer.serialize_entry(
            "target",
            self.visitor
                .get("log.target")
                .unwrap_or_else(|| self.metadata.target()),
        )?;
        map_serializer.serialize_entry(
            "file",
            self.metadata
                .file()
                .or_else(|| self.visitor.get("log.file"))
                .unwrap_or("-"),
        )?;
        map_serializer.serialize_entry(
            "line",
            &self
                .metadata
                .line()
                .or_else(|| self.visitor.get("log.line"))
                .unwrap_or(0),
        )?;

        for (key, value) in self
            .visitor
            .fields()
            .iter()
            .filter(|(&key, _)| key != "message" && !key.starts_with("log."))
        {
            map_serializer.serialize_entry(key, value)?;
        }

        if let Some(current_span) = self
            .ctx
            .current_span()
            .id()
            .and_then(|id| self.ctx.span(id))
        {
            map_serializer.serialize_entry("current_span", &SpanSerializer(&current_span))?;
        }

        map_serializer.serialize_entry("spans", &SpanListSerializer(self.ctx))?;

        map_serializer.end()
    }
}

struct SpanSerializer<'a, 'b, Span>(&'b SpanRef<'a, Span>)
where
    Span: for<'lookup> LookupSpan<'lookup>;

impl<'a, 'b, Span> Serialize for SpanSerializer<'a, 'b, Span>
where
    Span: for<'lookup> LookupSpan<'lookup>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serializer = serializer.serialize_map(None)?;

        serializer.serialize_entry("name", self.0.metadata().name())?;
        serializer.serialize_entry("line", &self.0.metadata().line())?;
        serializer.serialize_entry("target", &self.0.metadata().target())?;
        serializer.serialize_entry("file", &self.0.metadata().file())?;

        if let Some(visitor) = self.0.extensions().get::<PrimaJsonVisitor>() {
            for (key, value) in visitor.fields().iter() {
                serializer.serialize_entry(key, value)?;
            }
        }

        serializer.end()
    }
}

struct SpanListSerializer<'a, 'b, S>(&'b Context<'a, S>)
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>;

impl<'a, 'b, Sub> Serialize for SpanListSerializer<'a, 'b, Sub>
where
    Sub: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serializer = serializer.serialize_seq(None)?;

        if let Some(span_root) = self
            .0
            .current_span()
            .id()
            .and_then(|id| self.0.span_scope(id).map(|iter| iter.from_root()))
        {
            for span in span_root {
                serde::ser::SerializeSeq::serialize_element(
                    &mut serializer,
                    &SpanSerializer(&span),
                )?;
            }
        }

        serde::ser::SerializeSeq::end(serializer)
    }
}
