use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    trace::{self, Tracer},
    Resource,
};
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
            opentelemetry_sdk::runtime::TokioCurrentThread
        }
        #[cfg(not(feature = "rt-tokio-current-thread"))]
        {
            opentelemetry_sdk::runtime::Tokio
        }
    };

    let collector_url = telemetry.collector_url.as_str();
    // OTLP before version 0.15 didn't append a /v1/traces suffix, but started doing so there.
    // For backwards compatibility we strip it from configurations that do have it
    let collector_url = collector_url
        // In case of a trailing slash strip it
        .strip_suffix("/")
        .unwrap_or(collector_url)
        .strip_suffix("/v1/traces")
        .unwrap_or(collector_url);

    // Backport https://github.com/open-telemetry/opentelemetry-rust/pull/1553
    let collector_url = collector_url.strip_suffix("/").unwrap_or(collector_url);

    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_endpoint(collector_url.to_string());

    let resource = Resource::new(vec![
        KeyValue::new("environment", config.env.to_string()),
        KeyValue::new("country", config.country.to_string()),
        KeyValue::new("service.name", telemetry.service_name.clone()),
    ]);

    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .with_trace_config(trace::config().with_resource(resource))
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
