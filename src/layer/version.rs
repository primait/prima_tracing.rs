use opentelemetry::KeyValue;
use tracing::span;
use tracing_core::Subscriber;
use tracing_opentelemetry::OtelData;
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;

pub struct VersionLayer {
    pub version: Option<String>,
}

impl<S> Layer<S> for VersionLayer
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_new_span(&self, _attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();

        if let (Some(otel_data), Some(version)) = (extensions.get_mut::<OtelData>(), &self.version)
        {
            otel_data
                .builder
                .attributes
                .get_or_insert_with(Default::default)
                .extend([
                    KeyValue::new("version", version.clone()),
                    KeyValue::new("service.version", version.clone()),
                ]);
        }
    }
}
