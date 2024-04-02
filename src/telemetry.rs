use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    trace::{self, Tracer},
    Resource,
};

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
        .strip_suffix('/')
        .unwrap_or(collector_url)
        .strip_suffix("/v1/traces")
        .unwrap_or(collector_url);

    // Backport https://github.com/open-telemetry/opentelemetry-rust/pull/1553
    let collector_url = collector_url.strip_suffix('/').unwrap_or(collector_url);

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
