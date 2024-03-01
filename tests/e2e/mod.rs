use opentelemetry_jaeger::testing::jaeger_api_v2::Span;
use opentelemetry_jaeger::testing::jaeger_client::JaegerTestClient;
use prima_tracing::{builder, configure_subscriber, init_subscriber, Country, Environment};

async fn get_spans(f: impl FnOnce()) -> Option<Vec<Span>> {
    std::env::set_var("RUST_LOG", "info");

    // Unique id for this test run
    let seed = std::fs::read_to_string("/proc/sys/kernel/random/uuid").unwrap();
    let service_name = format!("e2e-test-{seed}");

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
        f()
    }

    let mut client = JaegerTestClient::new(query_api_url);

    if !client.contain_service(&service_name).await {
        None
    } else {
        let spans = client.find_traces_from_services(&service_name).await;
        Some(spans)
    }
}

#[cfg(feature = "traces")]
#[tokio::test(flavor = "multi_thread")]
async fn traces_are_sent_to_datadog() {
    let log_message = "hello it_sends_traces_to_jaeger";

    let spans = get_spans(|| {
        let span = tracing::info_span!("my span");
        span.in_scope(|| {
            tracing::info!("{log_message}");
        });
    })
    .await
    .expect("Failed to fetch traces from jaeger");

    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].logs.len(), 1);

    let msg = spans[0].logs[0].fields[0].v_str.as_str();
    assert_eq!(log_message, msg);
}

#[cfg(feature = "traces")]
#[tokio::test(flavor = "multi_thread")]
async fn events_contain_metadata() {
    let spans = get_spans(|| {
        let span = tracing::info_span!("my span");
        span.in_scope(|| {
            tracing::info!(hello = "meta!", "meta?");
        });
    })
    .await
    .expect("Failed to fetch traces from jaeger");

    assert_eq!(
        "meta!",
        &spans[0].logs[0]
            .fields
            .iter()
            .find(|f| f.key == "hello")
            .unwrap()
            .v_str
    );
}
