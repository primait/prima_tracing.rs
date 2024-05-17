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
            if let Some(span) = ctx.lookup_current() {
                let mut visitor = ErrorVisitor::default();
                event.record(&mut visitor);

                let otel_data = span.extensions_mut().remove::<OtelData>();

                if let Some(mut otel_data) = otel_data {
                    let builder = &mut otel_data.builder;
                    let builder_attrs = builder.attributes.get_or_insert(vec![]);

                    // Adding fields to existing trace events (logs)
                    if let Some(ref mut events) = builder.events {
                        for event in events.iter_mut() {
                            event
                                .attributes
                                .push(KeyValue::new("error.message", visitor.message.clone()));
                            event
                                .attributes
                                .push(KeyValue::new("error.type", visitor.kind.clone()));
                            event
                                .attributes
                                .push(KeyValue::new("error.kind", visitor.kind.clone()));
                            event
                                .attributes
                                .push(KeyValue::new("error.stack", visitor.stack.clone()));
                        }
                    }

                    // Adding fields to existing trace tags
                    builder_attrs.push(KeyValue::new("error.message", visitor.message));
                    builder_attrs.push(KeyValue::new("error.type", visitor.kind.clone()));
                    builder_attrs.push(KeyValue::new("error.kind", visitor.kind));
                    builder_attrs.push(KeyValue::new("error.stack", visitor.stack));

                    span.extensions_mut().replace(otel_data);
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
    }

    fn record_debug(&mut self, _field: &Field, _value: &dyn std::fmt::Debug) {}
}
