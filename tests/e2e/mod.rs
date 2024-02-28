use opentelemetry_jaeger::testing::jaeger_client::JaegerTestClient;
use prima_tracing::{builder, configure_subscriber, init_subscriber, Country, Environment};
use std::time::SystemTime;

#[cfg(feature = "traces")]
#[tokio::test(flavor = "multi_thread")]
async fn it_sends_traces_to_jaeger() {
    std::env::set_var("RUST_LOG", "info");

    let service_name = "it_sends_traces_to_jaeger";
    // Unique id for this test run
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let service_name = format!("{service_name}-{seed}");
    let log_message = format!("hello {service_name}");


    let collector_url = "http://jaeger:55681";
    let query_api_url = "http://jaeger:16685";

    let subscriber = configure_subscriber(
        builder(&service_name)
            .with_country(Country::Common)
            .with_env(Environment::Dev)
            .with_telemetry(collector_url.to_string(), service_name.clone())
            .build(),
    );

    {
        let _guard = init_subscriber(subscriber);

        let span = tracing::info_span!("my span");
        span.in_scope(|| {
            tracing::info!("{log_message}");
        });
    }

    let mut client = JaegerTestClient::new(query_api_url);

    assert!(
        client.contain_service(&service_name).await,
        "jaeger cannot find service with name {}",
        service_name
    );

    let spans = client.find_traces_from_services(&service_name).await;
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].logs.len(), 1);

    let msg = spans[0].logs[0].fields[0].v_str.as_str();
    assert_eq!(log_message, msg);

    assert!(true);
}
