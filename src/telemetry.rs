use opentelemetry::{
    sdk::{
        trace::{self, Tracer},
        Resource,
    },
    KeyValue,
};

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
