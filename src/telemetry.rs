use opentelemetry::trace::SpanBuilder;
use opentelemetry::{
    sdk::{
        trace::{self, Tracer},
        Resource,
    },
    KeyValue,
};
use tracing::{span, Subscriber};
use tracing_subscriber::{layer::Context, Layer};

use crate::SubscriberConfig;

pub fn configure<T>(config: &SubscriberConfig<T>) -> Tracer {
    let telemetry = config
        .telemetry
        .as_ref()
        .expect("Tracing config should be provided when the feature `prima-tracing` is enabled");

    opentelemetry_zipkin::new_pipeline()
        .with_collector_endpoint(telemetry.collector_url.as_str())
        .with_service_name(telemetry.service_name.as_str())
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                "environment",
                config.env.clone(),
            )])),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Failed to create the zipkin pipeline")
}

pub struct VersionLayer {
    pub version: Option<String>,
}

impl<S> Layer<S> for VersionLayer
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn new_span(&self, _attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();
        match (extensions.get_mut::<SpanBuilder>(), &self.version) {
            (Some(builder), Some(version)) => {
                let root_version = KeyValue::new("version", version.clone());
                let service_version = KeyValue::new("service.version", version.clone());
                if let Some(ref mut attributes) = builder.attributes {
                    attributes.push(root_version);
                    attributes.push(service_version);
                } else {
                    builder.attributes = Some(vec![root_version, service_version]);
                }
            }
            _ => {}
        }
    }

    fn on_record(&self, _id: &span::Id, _values: &span::Record<'_>, _ctx: Context<'_, S>) {}
}
