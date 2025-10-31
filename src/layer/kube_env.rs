use opentelemetry::KeyValue;
use tracing::{span, Subscriber};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{layer::Context, Layer};

// Extracts KUBE_APP_PART_OF, KUBE_APP_MANAGED_BY, KUBE_APP_VERSION and KUBE_APP_INSTANCE from the environment
// and adds them to span attributes
pub struct KubeEnvLayer {
    pub kube_app_part_of: Option<String>,
    pub kube_app_managed_by: Option<String>,
    pub kube_app_version: Option<String>,
    pub kube_app_instance: Option<String>,
}

impl Default for KubeEnvLayer {
    fn default() -> Self {
        Self {
            kube_app_part_of: std::env::var("KUBE_APP_PART_OF").ok(),
            kube_app_managed_by: std::env::var("KUBE_APP_MANAGED_BY").ok(),
            kube_app_version: std::env::var("KUBE_APP_VERSION").ok(),
            kube_app_instance: std::env::var("KUBE_APP_INSTANCE").ok(),
        }
    }
}

impl<S> Layer<S> for KubeEnvLayer
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_new_span(&self, _attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut exts = span.extensions_mut();

        let mut kvs: Vec<KeyValue> = Vec::new();
        if let Some(ref part_of) = self.kube_app_part_of {
            kvs.push(KeyValue::new("kube_app_part_of", part_of.clone()));
        }
        if let Some(ref managed_by) = self.kube_app_managed_by {
            kvs.push(KeyValue::new("kube_app_managed_by", managed_by.clone()));
        }
        if let Some(ref version) = self.kube_app_version {
            kvs.push(KeyValue::new("kube_app_version", version.clone()));
        }
        if let Some(ref instance) = self.kube_app_instance {
            kvs.push(KeyValue::new("kube_app_instance", instance.clone()));
        }

        // Store attributes to apply on enter
        exts.insert(KubeAttrs(kvs));
    }

    fn on_enter(&self, id: &span::Id, ctx: Context<'_, S>) {
        if let Some(span_ref) = ctx.span(id) {
            if let Some(attrs) = span_ref.extensions().get::<KubeAttrs>() {
                let current = tracing::Span::current();
                for kv in &attrs.0 {
                    current.set_attribute(kv.key.clone(), kv.value.clone());
                }
            }
        }
    }
}
