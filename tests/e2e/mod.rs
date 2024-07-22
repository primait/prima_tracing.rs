use std::fmt::{Debug, Display, Formatter};

use opentelemetry_jaeger::testing::jaeger_api_v2::Span;
use opentelemetry_jaeger::testing::jaeger_client::JaegerTestClient;

use prima_tracing::{builder, configure_subscriber, init_subscriber, Country, Environment};

async fn get_spans(f: impl FnOnce(), collector_url: &str) -> Option<Vec<Span>> {
    std::env::set_var("RUST_LOG", "info");

    // Unique id for this test run
    let seed = uuid::Uuid::new_v4();
    let service_name = format!("e2e-test-{seed}");

    let query_api_url = "http://jaeger:16685/";

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
    let log_message = "hello traces_are_sent_to_datadog";

    let spans = get_spans(
        || {
            let span = tracing::info_span!("my span");
            span.in_scope(|| {
                tracing::info!("{log_message}");
            });
        },
        "http://jaeger:55681",
    )
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
    let spans = get_spans(
        || {
            let span = tracing::info_span!("my span");
            span.in_scope(|| {
                tracing::info!(hello = "meta!", "meta?");
            });
        },
        "http://jaeger:55681",
    )
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

#[cfg(feature = "traces")]
#[tokio::test(flavor = "multi_thread")]
async fn strips_trailing_slash() {
    get_spans(
        || {
            let span = tracing::info_span!("my span");
            span.in_scope(|| {
                tracing::info!(hello = "meta!", "meta?");
            });
        },
        "http://jaeger:55681/",
    )
    .await
    .expect("Failed to fetch traces from jaeger");
}

#[cfg(feature = "traces")]
#[tokio::test(flavor = "multi_thread")]
async fn strips_trailing_endpoint() {
    get_spans(
        || {
            let span = tracing::info_span!("my span");
            span.in_scope(|| {
                tracing::info!(hello = "meta!", "meta?");
            });
        },
        "http://jaeger:55681/v1/traces/",
    )
    .await
    .expect("Failed to fetch traces from jaeger");
}

#[cfg(feature = "traces")]
#[tokio::test(flavor = "multi_thread")]
async fn error_layer_enrich_errored_spans() {
    std::env::set_var("RUST_LOG", "info");

    // Unique id for this test run
    let seed = uuid::Uuid::new_v4();
    let service_name = format!("e2e-test-{seed}");

    let query_api_url = "http://jaeger:16685/";

    let subscriber = configure_subscriber(
        builder(&service_name)
            .with_country(Country::Common)
            .with_env(Environment::Dev)
            .with_telemetry("http://jaeger:55681".to_string(), service_name.clone())
            .build(),
    );

    {
        let _guard = init_subscriber(subscriber);
        let error = Error {
            message: "error message".to_string(),
        };
        let span = tracing::error_span!("An error occurred");
        span.in_scope(|| {
            tracing::error!(
                error = &error as &dyn std::error::Error,
                "An error occurred"
            );
        });
    };

    let mut client = JaegerTestClient::new(query_api_url);

    let spans = if !client.contain_service(&service_name).await {
        None
    } else {
        let spans = client.find_traces_from_services(&service_name).await;
        Some(spans)
    }
    .unwrap();

    let err_msg = spans[0].tags.iter().find(|f| f.key == "error.message");
    let err_kind = spans[0].tags.iter().find(|f| f.key == "error.type");
    let err_stack = spans[0].tags.iter().find(|f| f.key == "error.stack");

    assert_eq!("Error: error message", err_msg.unwrap().v_str);
    assert_eq!("Error", err_kind.unwrap().v_str);
    assert_eq!("Error: error message", err_stack.unwrap().v_str);

    assert_eq!(spans[0].logs.len(), 1);

    let err_msg = spans[0].logs[0]
        .fields
        .iter()
        .find(|f| f.key == "error.message");
    let err_kind = spans[0].logs[0]
        .fields
        .iter()
        .find(|f| f.key == "error.type");
    let err_stack = spans[0].logs[0]
        .fields
        .iter()
        .find(|f| f.key == "error.stack");

    assert_eq!("Error: error message", err_msg.unwrap().v_str);
    assert_eq!("Error", err_kind.unwrap().v_str);
    assert_eq!("Error: error message", err_stack.unwrap().v_str);
}

#[derive(Debug)]
struct Error {
    message: String,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Error: {}", &self.message))
    }
}
