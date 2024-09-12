use once_cell::sync::Lazy;
use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    trace::{self, Tracer},
    Resource,
};
use std::mem;
use std::sync::Mutex;

use crate::SubscriberConfig;

fn normalize_collector_url(collector_url: &str) -> String {
    // OTLP before version 0.15 didn't append a /v1/traces suffix, but started doing so there.
    // For backwards compatibility we strip it from configurations that do have it.
    let collector_url = collector_url
        // In case of a trailing slash strip it
        .strip_suffix('/')
        .unwrap_or(collector_url)
        .strip_suffix("/v1/traces")
        .unwrap_or(collector_url);

    // Backport https://github.com/open-telemetry/opentelemetry-rust/pull/1553
    let collector_url = collector_url.strip_suffix('/').unwrap_or(collector_url);

    // And now starting from version 0.23 opentelemetry randomly stopped appending
    // the url suffix, so we need to do it ourselves.
    // This was not announced in the changelogs 🙃
    collector_url.to_string() + "/v1/traces"
}

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

    let collector_url = normalize_collector_url(&telemetry.collector_url);

    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_endpoint(collector_url);

    let resource = Resource::new(vec![
        KeyValue::new("environment", config.env.to_string()),
        KeyValue::new("country", config.country.to_string()),
        KeyValue::new("service.name", telemetry.service_name.clone()),
    ]);

    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .with_trace_config(trace::Config::default().with_resource(resource))
        .install_batch(runtime)
        .expect("Failed to configure the OpenTelemetry tracer provider");

    set_tracer_provider(tracer_provider.clone());

    tracer_provider
        .tracer_builder("prima-tracing")
        .with_version(env!("CARGO_PKG_VERSION"))
        .build()
}

// Consider to remove this wrapper when https://github.com/open-telemetry/opentelemetry-rust/issues/1961 is resolved
static TRACER_PROVIDER: Lazy<Mutex<Option<trace::TracerProvider>>> = Lazy::new(Default::default);

fn set_tracer_provider(new_provider: trace::TracerProvider) {
    global::set_tracer_provider(new_provider.clone());

    let mut tracer_provider = TRACER_PROVIDER
        .lock()
        .expect("OpenTelemetry tracer provider mutex poisoned");
    _ = mem::replace(&mut *tracer_provider, Some(new_provider));
}

pub(crate) fn shutdown_tracer_provider() {
    global::shutdown_tracer_provider();

    let tracer_provider = TRACER_PROVIDER
        .lock()
        .expect("OpenTelemetry tracer provider mutex poisoned")
        .take()
        .expect("OpenTelemetry tracer provider is missing, cannot shutdown");

    if let Err(err) = tracer_provider.shutdown() {
        eprintln!("Failed to shutdown the OpenTelemetry tracer provider: {err:?}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normalize_collector_url_test() {
        let base = "http://localhost:8080";
        let expected = "http://localhost:8080/v1/traces";

        assert_eq!(normalize_collector_url(base), expected);

        let with_trailing_slash = format!("{}/", base);
        assert_eq!(
            normalize_collector_url(with_trailing_slash.as_str()),
            expected
        );

        let complete = format!("{}/v1/traces", base);
        assert_eq!(normalize_collector_url(complete.as_str()), expected);
    }
}
