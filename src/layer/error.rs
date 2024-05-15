use opentelemetry::KeyValue;
use tracing::field::Field;
use tracing::field::Visit;
use tracing::Event;
use tracing::Level;
use tracing::Subscriber;
use tracing_opentelemetry::OtelData;
use tracing_subscriber::{registry::LookupSpan, Layer};

pub struct ErrorLayer;

impl<S> Layer<S> for ErrorLayer
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    fn on_event(&self, event: &Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        if event.metadata().is_event() && event.metadata().level() == &Level::ERROR {
            let mut visitor = ErrorVisitor::default();
            event.record(&mut visitor);

            if let Some(span) = ctx.lookup_current() {
                if let Some(data) = span.extensions_mut().get_mut::<OtelData>() {
                    let builder_attrs =
                        data.builder.attributes.get_or_insert(Vec::with_capacity(4));
                    builder_attrs.extend([
                        KeyValue::new("error.message", visitor.message),
                        KeyValue::new("error.type", visitor.kind.clone()),
                        KeyValue::new("error.kind", visitor.kind),
                        KeyValue::new("error.stack", visitor.stack),
                    ]);
                }
            };
        }
    }
}

#[derive(Default)]
struct ErrorVisitor {
    message: String,
    kind: String,
    stack: String,
}

impl Visit for ErrorVisitor {
    fn record_error(&mut self, _field: &Field, value: &(dyn std::error::Error + 'static)) {
        let mut source: String = format!("Stack:\n{}", value);
        let mut next_err = value.source();

        while let Some(err) = next_err {
            source.push_str(&format!("\n{}", err));
            next_err = err.source();
        }

        let error_msg = value.to_string();

        self.message = error_msg;
        self.kind = "Error".to_string();
        self.stack = source;
    }
    fn record_debug(&mut self, _field: &Field, _value: &dyn std::fmt::Debug) {}
}
