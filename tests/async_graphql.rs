#![cfg(feature = "async-graphql")]

#[cfg(test)]
mod tests {
    use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
    use std::fmt::Debug;
    use std::sync::{Arc, Mutex};
    use tracing::Level;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::Layer;

    use prima_tracing::async_graphql::TracingRootFieldsExtension;

    #[derive(Debug, Eq, PartialEq)]
    struct CapturedSpan {
        name: String,
        fields: Vec<Field>,
    }

    #[derive(Debug, Eq, PartialEq)]
    struct Field {
        name: String,
        value: String,
    }

    impl tracing::field::Visit for CapturedSpan {
        fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
            self.fields.push(Field {
                name: field.name().to_string(),
                value: value.to_string(),
            })
        }

        fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn Debug) {
            self.fields.push(Field {
                name: field.name().to_string(),
                value: format!("{:?}", value),
            })
        }
    }

    #[derive(Debug)]
    struct CapturedEvent {
        level: tracing::Level,
        fields: Vec<Field>,
    }

    impl tracing::field::Visit for CapturedEvent {
        fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
            self.fields.push(Field {
                name: field.name().to_string(),
                value: value.to_string(),
            })
        }

        fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn Debug) {
            self.fields.push(Field {
                name: field.name().to_string(),
                value: format!("{:?}", value),
            })
        }
    }

    struct CapturingLayer {
        spans: Arc<Mutex<Vec<CapturedSpan>>>,
        events: Arc<Mutex<Vec<CapturedEvent>>>,
    }

    impl<S: tracing::Subscriber> Layer<S> for CapturingLayer {
        fn on_new_span(
            &self,
            attrs: &tracing::span::Attributes<'_>,
            _id: &tracing::span::Id,
            _ctx: tracing_subscriber::layer::Context<'_, S>,
        ) {
            let mut span = CapturedSpan {
                name: attrs.metadata().name().to_string(),
                fields: vec![],
            };
            attrs.record(&mut span);
            self.spans.lock().unwrap().push(span);
        }

        fn on_event(
            &self,
            event: &tracing::Event<'_>,
            _ctx: tracing_subscriber::layer::Context<'_, S>,
        ) {
            let mut captured = CapturedEvent {
                level: *event.metadata().level(),
                fields: vec![],
            };
            event.record(&mut captured);
            self.events.lock().unwrap().push(captured);
        }
    }

    struct QueryRoot;

    #[Object]
    impl QueryRoot {
        async fn users(&self) -> Vec<String> {
            tracing::info!("something happened in users resolver");
            vec!["user_1".to_string()]
        }

        async fn organizations(&self) -> Vec<String> {
            tracing::info!("something happened in organization resolver");
            vec!["org_1".to_string()]
        }
    }

    fn span_has_fields(span: &CapturedSpan, expected: &[(&str, &str)]) -> bool {
        expected.iter().all(|(name, value)| {
            span.fields
                .iter()
                .any(|field| field.name == *name && field.value == *value)
        })
    }

    struct MutRoot;

    #[Object]
    impl MutRoot {
        async fn create_user(&self) -> String {
            "user_new".to_string()
        }
    }

    type CapturedSpans = Arc<Mutex<Vec<CapturedSpan>>>;
    type CapturedEvents = Arc<Mutex<Vec<CapturedEvent>>>;

    struct QueryWithRequiredArg;

    #[Object]
    impl QueryWithRequiredArg {
        async fn user_by_id(&self, id: String) -> String {
            id
        }
    }

    fn setup_subscriber_with_captures() -> (CapturedSpans, CapturedEvents, impl tracing::Subscriber)
    {
        let spans: CapturedSpans = Arc::new(Mutex::new(vec![]));
        let events: CapturedEvents = Arc::new(Mutex::new(vec![]));
        let layer = CapturingLayer {
            spans: Arc::clone(&spans),
            events: Arc::clone(&events),
        };
        let subscriber = tracing_subscriber::registry().with(layer).with(
            tracing_subscriber::filter::LevelFilter::from_level(Level::DEBUG),
        );
        (spans, events, subscriber)
    }

    #[test_log::test(tokio::test(flavor = "current_thread"))]
    async fn field_name_appears_in_tracing_span() {
        let (spans, _events, subscriber) = setup_subscriber_with_captures();
        let _guard = tracing::subscriber::set_default(subscriber);

        let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
            .extension(TracingRootFieldsExtension::new("test_schema"))
            .finish();

        schema.execute("query { users organizations }").await;

        let captured = spans.lock().unwrap();

        let request_spans: Vec<_> = captured
            .iter()
            .filter(|span| span.name == "graphql_request")
            .collect();
        assert_eq!(
            request_spans.len(),
            1,
            "expected exactly one graphql_request span"
        );
        assert!(
            span_has_fields(request_spans[0], &[("schema", "test_schema")]),
            "expected graphql_request span to have schema tag",
        );

        let root_spans: Vec<_> = captured
            .iter()
            .filter(|span| span.name == "graphql_root_field")
            .collect();
        assert_eq!(root_spans.len(), 2);

        assert!(
            root_spans.iter().any(|span| span_has_fields(
                span,
                &[
                    ("name", "users"),
                    ("operation_type", "query"),
                    ("parent_type", "QueryRoot"),
                    ("return_type", "[String!]!"),
                ]
            )),
            "expected a span for the `users` root field",
        );

        assert!(
            root_spans.iter().any(|span| span_has_fields(
                span,
                &[
                    ("name", "organizations"),
                    ("operation_type", "query"),
                    ("parent_type", "QueryRoot"),
                    ("return_type", "[String!]!"),
                ]
            )),
            "expected a span for the `organizations` root field",
        );
    }

    #[test_log::test(tokio::test(flavor = "current_thread"))]
    async fn mutation_operation_type_appears_in_tracing_span() {
        let (spans, _events, subscriber) = setup_subscriber_with_captures();
        let _guard = tracing::subscriber::set_default(subscriber);

        let schema = Schema::build(QueryRoot, MutRoot, EmptySubscription)
            .extension(TracingRootFieldsExtension::new("test_schema"))
            .finish();

        schema.execute("mutation { createUser }").await;

        let captured = spans.lock().unwrap();

        let request_spans: Vec<_> = captured
            .iter()
            .filter(|span| span.name == "graphql_request")
            .collect();
        assert_eq!(
            request_spans.len(),
            1,
            "expected exactly one graphql_request span"
        );
        assert!(
            span_has_fields(request_spans[0], &[("schema", "test_schema")]),
            "expected graphql_request span to have schema tag",
        );

        let root_spans: Vec<_> = captured
            .iter()
            .filter(|span| span.name == "graphql_root_field")
            .collect();
        assert_eq!(root_spans.len(), 1);

        assert!(
            root_spans.iter().any(|span| span_has_fields(
                span,
                &[
                    ("name", "createUser"),
                    ("operation_type", "mutation"),
                    ("parent_type", "MutRoot"),
                    ("return_type", "String!"),
                ]
            )),
            "expected a span for the `createUser` mutation root field",
        );
    }

    #[test_log::test(tokio::test(flavor = "current_thread"))]
    async fn unknown_field_logs_validation_error() {
        let (spans, events, subscriber) = setup_subscriber_with_captures();
        let _guard = tracing::subscriber::set_default(subscriber);

        let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
            .extension(TracingRootFieldsExtension::new("test_schema"))
            .finish();

        // `nonExistentField` does not exist on QueryRoot — this is a schema violation.
        let response = schema.execute("query { nonExistentField }").await;

        assert!(
            !response.errors.is_empty(),
            "expected validation errors for unknown field"
        );

        // No root-field span should be emitted because execution never started.
        let captured_spans = spans.lock().unwrap();

        let request_spans: Vec<_> = captured_spans
            .iter()
            .filter(|span| span.name == "graphql_request")
            .collect();
        assert_eq!(
            request_spans.len(),
            1,
            "expected exactly one graphql_request span even when validation fails"
        );
        assert!(
            span_has_fields(request_spans[0], &[("schema", "test_schema")]),
            "expected graphql_request span to have schema tag",
        );

        let root_spans: Vec<_> = captured_spans
            .iter()
            .filter(|span| span.name == "graphql_root_field")
            .collect();
        assert_eq!(
            root_spans.len(),
            0,
            "no graphql_root_field span expected when validation fails"
        );

        let captured_events = events.lock().unwrap();
        let error_events: Vec<_> = captured_events
            .iter()
            .filter(|e| e.level == tracing::Level::ERROR)
            .collect();
        assert!(
            !error_events.is_empty(),
            "expected at least one ERROR log event for the unknown field validation error"
        );
        assert!(
            error_events.iter().any(|e| {
                e.fields
                    .iter()
                    .any(|f| f.name == "message" && f.value.contains("graphql validation error"))
            }),
            "expected the ERROR event to contain 'graphql validation error'"
        );
    }

    #[test_log::test(tokio::test(flavor = "current_thread"))]
    async fn parse_error_does_not_produce_root_field_span() {
        let (spans, events, subscriber) = setup_subscriber_with_captures();
        let _guard = tracing::subscriber::set_default(subscriber);

        let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
            .extension(TracingRootFieldsExtension::new("test_schema"))
            .finish();

        // Deliberately malformed GraphQL: unclosed brace → parse error.
        let response = schema.execute("query { users {").await;

        assert!(
            !response.errors.is_empty(),
            "expected a parse error for malformed query"
        );

        let captured_spans = spans.lock().unwrap();

        let request_spans: Vec<_> = captured_spans
            .iter()
            .filter(|span| span.name == "graphql_request")
            .collect();
        assert_eq!(
            request_spans.len(),
            1,
            "expected exactly one graphql_request span even when parsing fails"
        );
        assert!(
            span_has_fields(request_spans[0], &[("schema", "test_schema")]),
            "expected graphql_request span to have schema tag",
        );

        let root_spans: Vec<_> = captured_spans
            .iter()
            .filter(|span| span.name == "graphql_root_field")
            .collect();
        assert_eq!(
            root_spans.len(),
            0,
            "no graphql_root_field span expected when parsing fails"
        );

        let captured_events = events.lock().unwrap();
        let error_events: Vec<_> = captured_events
            .iter()
            .filter(|e| e.level == tracing::Level::ERROR)
            .collect();
        assert!(
            !error_events.is_empty(),
            "expected at least one ERROR log event for the parse error"
        );
        assert!(
            error_events.iter().any(|e| {
                e.fields
                    .iter()
                    .any(|f| f.name == "message" && f.value.contains("graphql query parse error"))
            }),
            "expected the ERROR event to contain 'graphql query parse error'"
        );
    }

    #[test_log::test(tokio::test(flavor = "current_thread"))]
    async fn missing_mandatory_argument_logs_error() {
        let (spans, events, subscriber) = setup_subscriber_with_captures();
        let _guard = tracing::subscriber::set_default(subscriber);

        let schema = Schema::build(QueryWithRequiredArg, EmptyMutation, EmptySubscription)
            .extension(TracingRootFieldsExtension::new("test_schema"))
            .finish();

        // `id` is a required argument — omitting it triggers a schema validation error.
        let response = schema.execute("query { userById }").await;

        assert!(
            !response.errors.is_empty(),
            "expected validation errors for missing required argument"
        );

        let captured_spans = spans.lock().unwrap();
        let request_spans: Vec<_> = captured_spans
            .iter()
            .filter(|span| span.name == "graphql_request")
            .collect();
        assert_eq!(
            request_spans.len(),
            1,
            "expected exactly one graphql_request span even when validation fails"
        );
        assert!(
            span_has_fields(request_spans[0], &[("schema", "test_schema")]),
            "expected graphql_request span to have schema tag",
        );
        drop(captured_spans);

        let captured = events.lock().unwrap();
        let error_events: Vec<_> = captured
            .iter()
            .filter(|e| e.level == tracing::Level::ERROR)
            .collect();

        assert!(
            !error_events.is_empty(),
            "expected at least one ERROR log event for the missing required argument"
        );

        assert!(
            error_events.iter().any(|e| {
                e.fields
                    .iter()
                    .any(|f| f.name == "message" && f.value.contains("graphql validation error"))
            }),
            "expected the ERROR event to contain 'graphql validation error'"
        );
    }
}
