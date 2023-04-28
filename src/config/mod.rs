use std::str::FromStr;

pub use self::{
    country::{Country, CountryParseError},
    environment::{Environment, EnvironmentParseError},
};
#[cfg(feature = "json-logger")]
use crate::json::formatter::DefaultEventFormatter;

mod country;
mod environment;

#[cfg(not(feature = "json-logger"))]
use crate::subscriber::NopEventFormatter;

/// `SubscriberConfig` configuration built via [`SubscriberConfigBuilder`]
/// It contains
/// - Application env
/// - Telemetry config
/// - JSON formatter
pub struct SubscriberConfig<T> {
    pub country: Country,
    pub env: Environment,
    pub telemetry: Option<TelemetryConfig>,
    pub service: String,
    pub version: Option<String>,
    pub json_formatter: T,
}

#[cfg(not(feature = "json-logger"))]
/// Create a [`SubscriberConfigBuilder`]
pub fn builder(service: &str) -> SubscriberConfigBuilder<NopEventFormatter> {
    SubscriberConfigBuilder {
        service: service.to_owned(),
        country: Country::Unknown,
        env: Environment::Dev,
        telemetry: None,
        version: None,
        formatter: NopEventFormatter,
    }
}

#[cfg(feature = "json-logger")]
/// Create a [`SubscriberConfigBuilder`]
pub fn builder(service: &str) -> SubscriberConfigBuilder<DefaultEventFormatter> {
    SubscriberConfigBuilder {
        service: service.to_owned(),
        country: Country::Unknown,
        env: Environment::Dev,
        telemetry: None,
        version: None,
        formatter: DefaultEventFormatter,
    }
}

pub struct TelemetryConfig {
    pub collector_url: String,
    pub service_name: String,
}

pub struct SubscriberConfigBuilder<T> {
    country: Country,
    env: Environment,
    telemetry: Option<TelemetryConfig>,
    service: String,
    version: Option<String>,
    formatter: T,
}

impl<T> SubscriberConfigBuilder<T> {
    #[cfg(not(feature = "json-logger"))]
    /// Create a [`SubscriberConfigBuilder`]
    pub fn new(service: &str) -> SubscriberConfigBuilder<NopEventFormatter> {
        Self::_new(service, NopEventFormatter)
    }

    #[cfg(feature = "json-logger")]
    /// Create a [`SubscriberConfigBuilder`]
    pub fn new(service: &str) -> SubscriberConfigBuilder<DefaultEventFormatter> {
        Self::_new(service, DefaultEventFormatter)
    }

    fn _new<F>(service: &str, formatter: F) -> SubscriberConfigBuilder<F> {
        SubscriberConfigBuilder {
            service: service.to_owned(),
            country: Country::Unknown,
            env: Environment::Dev,
            telemetry: None,
            version: None,
            formatter,
        }
    }

    /// Build a [`SubscriberConfig`]
    pub fn build(self) -> SubscriberConfig<T> {
        SubscriberConfig {
            country: self.country,
            env: self.env,
            telemetry: self.telemetry,
            service: self.service,
            version: self.version,
            json_formatter: self.formatter,
        }
    }

    /// Set the country in which the application is running.
    pub fn with_country(mut self, country: Country) -> Self {
        self.country = country;
        self
    }

    /// Try to load `country` from the `COUNTRY` environment variable
    pub fn load_country(mut self) -> Self {
        let unparsed_country =
            std::env::var("COUNTRY").expect("COUNTRY variable must be defined to be loaded");
        self.country = match Country::from_str(&unparsed_country) {
            Ok(parsed_country) => parsed_country,
            Err(parse_error) => panic!("{}", parse_error),
        };
        self
    }

    /// Set the environment in which the application is running.
    pub fn with_env(mut self, env: Environment) -> Self {
        self.env = env;
        self
    }

    /// Try to load `env` from the `ENV` environment variable
    pub fn load_env(mut self) -> Self {
        let unparsed_env = std::env::var("ENV").expect("ENV variable must be defined to be loaded");
        self.env = match Environment::from_str(&unparsed_env) {
            Ok(parsed_env) => parsed_env,
            Err(parse_error) => panic!("{}", parse_error),
        };
        self
    }

    /// Set the application version.
    pub fn with_version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    /// Set the telemetry collector URL and the service name included in telemetry traces.
    pub fn with_telemetry(mut self, collector_url: String, service_name: String) -> Self {
        self.telemetry = Some(TelemetryConfig {
            collector_url,
            service_name,
        });

        self
    }

    /// Set the custom JSON formatter to be used when the feature `json-logger` is activated.
    pub fn with_custom_json_formatter<F>(self, formatter: F) -> SubscriberConfigBuilder<F> {
        SubscriberConfigBuilder {
            formatter,
            country: self.country,
            env: self.env,
            service: self.service,
            version: self.version,
            telemetry: self.telemetry,
        }
    }
}
