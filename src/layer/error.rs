use opentelemetry::trace::Status;
use opentelemetry::KeyValue;
use opentelemetry_semantic_conventions as semconv;

use tracing::field::{Field, Visit};
use tracing::{Event, Level, Span, Subscriber};
use tracing_opentelemetry::OpenTelemetrySpanExt as _;
use tracing_subscriber::{registry::LookupSpan, Layer};

pub struct ErrorLayer;

impl<S> Layer<S> for ErrorLayer
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    fn on_event(&self, event: &Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        // This is not an error
        if !event.metadata().is_event() || event.metadata().level() != &Level::ERROR {
            return;
        }

        // No current span, nothing to do
        if ctx.lookup_current().is_none() {
            return;
        }

        let mut visitor = ErrorVisitor::default();
        event.record(&mut visitor);

        if !visitor.is_error {
            return;
        }

        let span: Span = Span::current();

        // Create new exception event, as specified here
        // https://opentelemetry.io/docs/specs/semconv/exceptions/exceptions-spans/
        span.add_event(
            "exception",
            vec![
                // semconv
                KeyValue::new(semconv::trace::EXCEPTION_TYPE, visitor.kind.clone()),
                KeyValue::new(semconv::trace::EXCEPTION_MESSAGE, visitor.message.clone()),
                KeyValue::new(semconv::trace::EXCEPTION_STACKTRACE, visitor.stack.clone()),
                // legacy
                KeyValue::new("error.type", visitor.kind.clone()),
                KeyValue::new("error.message", visitor.message.clone()),
            ],
        );

        // setta lo status span a errore
        span.set_status(Status::error(visitor.message));
    }
}

#[derive(Default)]
struct ErrorVisitor {
    message: String,
    kind: String,
    stack: String,
    is_error: bool,
}

impl Visit for ErrorVisitor {
    fn record_error(&mut self, _field: &Field, value: &(dyn std::error::Error + 'static)) {
        let mut source: String = value.to_string();
        let mut next_err = value.source();

        while let Some(err) = next_err {
            source.push_str(&format!("\n{}", err));
            next_err = err.source();
        }

        let error_msg = value.to_string();

        self.message = error_msg;
        self.kind = "Error".to_string();
        self.stack = source;
        self.is_error = true;
    }

    fn record_debug(&mut self, _field: &Field, _value: &dyn std::fmt::Debug) {}
}
