#[cfg(feature = "prima-logger-json")]
use crate::json::formatter::DefaultEventFormatter;

#[cfg(not(feature = "prima-logger-json"))]
use crate::subscriber::NopEventFormatter;
/// `SubscriberConfig` configuration built via [`SubscriberConfigBuilder`]
/// It contains
/// - Application env
/// - Telemetry config
/// - JSON formatter
pub struct SubscriberConfig<T> {
    pub env: String,
    pub telemetry: Option<TelemetryConfig>,
    pub service: String,
    pub version: Option<String>,
    pub json_formatter: T,
}

impl<T> SubscriberConfig<T> {
    pub(crate) fn new(
        service: String,
        version: Option<String>,
        json_formatter: T,
    ) -> SubscriberConfig<T> {
        SubscriberConfig {
            env: String::from("dev"),
            telemetry: None,
            service,
            version,
            json_formatter,
        }
    }
}
#[cfg(not(feature = "prima-logger-json"))]
/// Create a [`SubscriberConfigBuilder`]
pub fn builder(service: &str) -> SubscriberConfigBuilder<NopEventFormatter> {
    SubscriberConfigBuilder(SubscriberConfig::new(
        service.to_owned(),
        None,
        NopEventFormatter {},
    ))
}

#[cfg(feature = "prima-logger-json")]
/// Create a [`SubscriberConfigBuilder`]
pub fn builder(service: &str) -> SubscriberConfigBuilder<DefaultEventFormatter> {
    SubscriberConfigBuilder(SubscriberConfig::new(
        service.to_owned(),
        None,
        DefaultEventFormatter {},
    ))
}

pub struct TelemetryConfig {
    pub collector_url: String,
    pub service_name: String,
}

pub struct SubscriberConfigBuilder<T>(SubscriberConfig<T>);

impl<T> SubscriberConfigBuilder<T> {
    /// Build a [`SubscriberConfig`]
    pub fn build(self) -> SubscriberConfig<T> {
        self.0
    }
    /// Set the environment. By `dev` default
    pub fn with_env(mut self, env: String) -> Self {
        self.0.env = env;
        self
    }
    /// Set the service version.
    pub fn with_version(mut self, version: String) -> Self {
        self.0.version = Some(version);
        self
    }

    /// Set telemetry config like `collector_url` and `service_name`
    pub fn with_telemetry(mut self, collector_url: String, service_name: String) -> Self {
        self.0.telemetry = Some(TelemetryConfig {
            collector_url,
            service_name,
        });

        self
    }

    /// Set custom json formatter if the feature `prima-logger-json` is activated
    pub fn with_custom_json_formatter<F>(self, formatter: F) -> SubscriberConfigBuilder<F> {
        SubscriberConfigBuilder(SubscriberConfig {
            json_formatter: formatter,
            env: self.0.env,
            service: self.0.service,
            version: self.0.version,
            telemetry: self.0.telemetry,
        })
    }
}
