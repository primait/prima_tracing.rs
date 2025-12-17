#[cfg(test)]
mod tests {
    use tracing_capture::{CaptureLayer, CapturedEvent, SharedStorage};
    use tracing_subscriber::layer::SubscriberExt;

    #[cfg(feature = "anyhow")]
    use {anyhow::anyhow, prima_tracing::trace_anyhow_error};

    use prima_tracing::trace_error;

    #[test]
    fn produce_trace_error() {
        let error = "not a number".parse::<usize>().unwrap_err();

        with_test_tracing(
            || {
                trace_error!(error, "Parsing error!", something = "something");
            },
            |events| {
                let event = events.first().unwrap();

                assert_eq!(event["error.message"], "invalid digit found in string");
                assert_eq!(event["error.kind"], "core::num::error::ParseIntError");
                assert_eq!(
                    event["error.stack"].as_debug_str(),
                    Some("0: ParseIntError { kind: InvalidDigit }")
                );
                assert!(event["error.trace"].as_debug_str().is_some());
                assert!(event["error.trace"]
                    .as_debug_str()
                    .unwrap()
                    .contains("macros::tests::produce_trace_error"));
                assert_eq!(event["something"], "something");
                assert_eq!(
                    event["message"].as_debug_str(),
                    Some("Parsing error! invalid digit found in string")
                );
            },
        )
    }

    #[cfg(feature = "anyhow")]
    #[test]
    fn produce_trace_anyhow_error() {
        let error = anyhow!("an error");

        with_test_tracing(
            || {
                trace_anyhow_error!(error, "Throw error!");
            },
            |events| {
                let event = events.first().unwrap();

                assert_eq!(event["error.message"], "an error");
                assert_eq!(event["error.kind"], "&dyn core::error::Error");
                assert_eq!(event["error.stack"].as_debug_str(), Some("0: \"an error\""));
                assert_eq!(
                    event["error.trace"].as_debug_str(),
                    Some(format!("{}", error.backtrace()).as_str())
                );
                assert_eq!(
                    event["message"].as_debug_str(),
                    Some(format!("Throw error! {error:?}").as_str())
                );
            },
        )
    }

    fn with_test_tracing<F, T>(tracing: F, tests: T)
    where
        F: FnOnce(),
        T: FnOnce(Vec<CapturedEvent>),
    {
        let subscriber = tracing_subscriber::fmt().pretty().finish();
        let storage: SharedStorage = SharedStorage::default();
        let subscriber = subscriber.with(CaptureLayer::new(&storage));

        tracing::subscriber::with_default(subscriber, || {
            tracing::info_span!("test-span").in_scope(tracing);
        });

        let storage = storage.lock();
        let span = storage.all_spans().next().unwrap();

        tests(span.events().collect());
    }
}
