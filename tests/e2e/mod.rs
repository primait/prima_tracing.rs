use opentelemetry_jaeger::testing::jaeger_client::JaegerTestClient;
use prima_tracing::{builder, configure_subscriber, init_subscriber, Country, Environment};

#[cfg(feature = "traces")]
#[tokio::test(flavor = "multi_thread")]
async fn it_sends_traces_to_jaeger() {
    let service_name = "it_sends_traces_to_jaeger";

    let collector_url = "http://jaeger:14268/api/traces";
    let query_api_url = "http://jaeger:16685";

    let mut client = JaegerTestClient::new(query_api_url);
    let service_name = format!("{}-{}", service_name, "agent");

    let subscriber = configure_subscriber(
        builder(&service_name)
            .with_country(Country::Common)
            .with_env(Environment::Dev)
            .with_telemetry(collector_url.to_string(), service_name.clone())
            .build(),
    );
    let _guard = init_subscriber(subscriber);

    assert!(
        client.contain_service(&service_name).await,
        "jaeger cannot find service with name {}",
        service_name
    );

    // let spans = client.find_traces_from_services(&service_name).await;
    // assert_eq!(spans.len(), 5);

    assert!(true);
}
