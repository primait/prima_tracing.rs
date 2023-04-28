use tracing::{Event, Subscriber};
use tracing_log::LogTracer;
use tracing_subscriber::{
    layer::{Context, SubscriberExt},
    EnvFilter,
};

use crate::config::SubscriberConfig;

pub struct Tracing;

impl Tracing {}

/// Configure a subscriber using [`SubscriberConfig`].
/// The configuration is behind feature flags
/// - `default`: uses the [`tracing_subscriber::fmt::layer()`]
/// - `json-logger`: activate the json logger
/// - `traces`: activate spans export via `opentelemetry-otlp`
pub fn configure_subscriber<T: EventFormatter + Send + Sync + 'static>(
    _config: SubscriberConfig<T>,
) -> impl Subscriber + Send + Sync {
    let subscriber = tracing_subscriber::Registry::default();
    let subscriber = subscriber.with(EnvFilter::from_default_env());

    #[cfg(feature = "traces")]
    let subscriber = {
        let tracer = crate::telemetry::configure(&_config);
        subscriber
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .with(crate::telemetry::VersionLayer {
                version: _config.version.clone(),
            })
    };

    #[cfg(not(feature = "json-logger"))]
    let subscriber = subscriber.with(tracing_subscriber::fmt::layer());
    #[cfg(feature = "json-logger")]
    let subscriber = {
        use crate::json::formatter::PrimaFormattingLayer;
        use crate::json::storage::PrimaJsonStorage;
        subscriber
            .with(PrimaJsonStorage::default())
            .with(PrimaFormattingLayer::new(
                _config.service.clone(),
                _config.country.map(|country| country.to_string()),
                _config.env.to_string(),
                &std::io::stdout,
                _config.json_formatter,
            ))
    };

    subscriber
}
/// Initialize the subscriber and return the [`Uninstall`] guard
pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) -> Uninstall {
    LogTracer::init().expect("Failed to set logger");
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    #[cfg(feature = "traces")]
    {
        use opentelemetry::{global, sdk::propagation::TraceContextPropagator};
        global::set_text_map_propagator(TraceContextPropagator::new());
    };
    Uninstall
}
/// `EventFormatter` allows you to customise the format of [`tracing::Event`] if the `json-logger` feature is active
pub trait EventFormatter {
    fn format_event<S>(
        &self,
        event: &Event<'_>,
        ctx: Context<'_, S>,
        info: ContextInfo<'_>,
    ) -> Result<Vec<u8>, std::io::Error>
    where
        S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>;
}
/// Uninstall guard for doing works in shutdown
pub struct Uninstall;

impl Drop for Uninstall {
    fn drop(&mut self) {
        #[cfg(feature = "traces")]
        opentelemetry::global::shutdown_tracer_provider();
    }
}
/// Information about the current app context like name or environment
pub struct ContextInfo<'a> {
    pub(crate) app_name: &'a str,
    pub(crate) country: &'a Option<&'a str>,
    pub(crate) environment: &'a str,
}

impl<'a> ContextInfo<'a> {
    pub fn app_name(&self) -> &'a str {
        self.app_name
    }

    pub fn country(&self) -> &'a Option<&'a str> {
        self.country
    }

    pub fn environment(&self) -> &'a str {
        self.environment
    }
}

pub struct NopEventFormatter {}

impl EventFormatter for NopEventFormatter {
    fn format_event<S>(
        &self,
        _event: &Event<'_>,
        _ctx: Context<'_, S>,
        _info: ContextInfo<'_>,
    ) -> Result<Vec<u8>, std::io::Error>
    where
        S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    {
        Ok(vec![])
    }
}
