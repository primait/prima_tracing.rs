use prima_tracing::{builder, configure_subscriber, init_subscriber, Country, Environment};
use tracing::{info, info_span};

#[tokio::main]
async fn main() {
    let service_name = "datadog-logger".to_string();
    let subscriber = configure_subscriber(
        builder(&service_name)
            .with_env(Environment::Dev)
            .with_country(Country::Common)
            .with_version("1.0".to_string())
            // We need a tracer if we want trace and span IDs to be created and propagated, otherwise logs won't contain these correlation IDs
            // You can also setup custom tracer and custom subscriber if you don't wanna use the `traces` feature
            .with_telemetry("http://localhost:55681/v1/traces".to_string(), service_name)
            .build(),
    );

    let _guard = init_subscriber(subscriber);

    // If inside a span the JSON logs will contain `"dd":{"span_id": u64,"trace_id": u64}` at root level and Datadog will use it to correlate logs and traces.
    // If you don't use Datadog for logging you can search for the span_id on the APM/Traces dashboard.
    let main_span = info_span!("MainSpan");
    main_span.in_scope(|| {
        // this log will be correlated to the MainSpan span_id
        info!("Starting my awesome app in the MainSpan");
        let hello_span = info_span!("HelloSpan");
        // all the logs inside this scope will be correlated to the HelloSpan span_id
        hello_span.in_scope(|| {
            hello();
        });
    });
}

fn hello() {
    info!("Hello!");
    hola();
}

fn hola() {
    info!("Hola!");
    halo();
}

fn halo() {
    info!("Halo!");
}
