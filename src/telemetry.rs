use opentelemetry::{
    sdk::{
        trace::{self, Tracer},
        Resource,
    },
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use tracing::{span, Subscriber};
use tracing_opentelemetry::OtelData;
use tracing_subscriber::{layer::Context, Layer};

use crate::SubscriberConfig;

pub fn configure<T>(config: &SubscriberConfig<T>) -> Tracer {
    let telemetry = config
        .telemetry
        .as_ref()
        .expect("Telemetry config must be provided when the `traces` feature is enabled.");

    let runtime = {
        #[cfg(feature = "rt-tokio-current-thread")]
        {
            opentelemetry::runtime::TokioCurrentThread
        }
        #[cfg(not(feature = "rt-tokio-current-thread"))]
        {
            opentelemetry::runtime::Tokio
        }
    };

    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_endpoint(telemetry.collector_url.as_str());
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .with_trace_config(
            trace::config()
                .with_resource(Resource::new(vec![KeyValue::new(
                    "environment",
                    config.env.to_string(),
                )]))
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    telemetry.service_name.clone(),
                )])),
        )
        .install_batch(runtime)
        .expect("Failed to configure the OpenTelemetry tracer")
}

pub struct VersionLayer {
    pub version: Option<String>,
}

impl<S> Layer<S> for VersionLayer
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_new_span(&self, _attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();

        if let (Some(otel_data), Some(version)) = (extensions.get_mut::<OtelData>(), &self.version)
        {
            otel_data
                .builder
                .attributes
                .get_or_insert_with(Default::default)
                .extend([
                    KeyValue::new("version", version.clone()),
                    KeyValue::new("service.version", version.clone()),
                ]);
        }
    }
}
