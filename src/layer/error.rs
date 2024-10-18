use opentelemetry::KeyValue;
use tracing::field::Field;
use tracing::field::Visit;
use tracing::Event;
use tracing::Level;
use tracing::Subscriber;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{registry::LookupSpan, Layer};

pub struct ErrorLayer;

impl<S> Layer<S> for ErrorLayer
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    fn on_event(&self, event: &Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        if event.metadata().is_event() && event.metadata().level() == &Level::ERROR {
            if let Some(span) = ctx.lookup_current() {
                let mut visitor = ErrorVisitor::default();
                event.record(&mut visitor);

                if visitor.is_error {
                    span.set_attribute("error.message", visitor.message);
                    span.set_attribute("error.type", visitor.kind.clone());
                    span.set_attribute("error.kind", visitor.kind);
                    span.set_attribute("error.stack", visitor.stack);
                }
            }
        }
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
