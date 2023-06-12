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
pub fn builder(
    service: &str,
) -> SubscriberConfigBuilder<NopEventFormatter, WithoutCountry, WithoutEnvironment> {
    SubscriberConfigBuilder::<NopEventFormatter, WithoutCountry, WithoutEnvironment>::new(service)
}

#[cfg(feature = "json-logger")]
/// Create a [`SubscriberConfigBuilder`]
pub fn builder(
    service: &str,
) -> SubscriberConfigBuilder<DefaultEventFormatter, WithoutCountry, WithoutEnvironment> {
    SubscriberConfigBuilder::<DefaultEventFormatter, WithoutCountry, WithoutEnvironment>::new(
        service,
    )
}

pub struct TelemetryConfig {
    pub collector_url: String,
    pub service_name: String,
}

pub struct WithoutCountry;
pub struct WithCountry(Country);

pub struct WithoutEnvironment;
pub struct WithEnvironment(Environment);

pub struct SubscriberConfigBuilder<F, C, E> {
    country: C,
    env: E,
    telemetry: Option<TelemetryConfig>,
    service: String,
    version: Option<String>,
    formatter: F,
}

impl<F, C, E> SubscriberConfigBuilder<F, C, E> {
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
    pub fn with_custom_json_formatter<G>(self, formatter: G) -> SubscriberConfigBuilder<G, C, E> {
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

impl<F> SubscriberConfigBuilder<F, WithoutCountry, WithoutEnvironment> {
    #[cfg(not(feature = "json-logger"))]
    /// Create a [`SubscriberConfigBuilder`]
    pub fn new(
        service: &str,
    ) -> SubscriberConfigBuilder<NopEventFormatter, WithoutCountry, WithoutEnvironment> {
        Self::_new(service, NopEventFormatter)
    }

    #[cfg(feature = "json-logger")]
    /// Create a [`SubscriberConfigBuilder`]
    pub fn new(
        service: &str,
    ) -> SubscriberConfigBuilder<DefaultEventFormatter, WithoutCountry, WithoutEnvironment> {
        Self::_new(service, DefaultEventFormatter)
    }

    fn _new<G>(
        service: &str,
        formatter: G,
    ) -> SubscriberConfigBuilder<G, WithoutCountry, WithoutEnvironment> {
        SubscriberConfigBuilder {
            service: service.to_owned(),
            country: WithoutCountry,
            env: WithoutEnvironment,
            telemetry: None,
            version: None,
            formatter,
        }
    }
}

impl<F, E> SubscriberConfigBuilder<F, WithoutCountry, E> {
    /// Set the country in which the application is running.
    pub fn with_country(self, country: Country) -> SubscriberConfigBuilder<F, WithCountry, E> {
        SubscriberConfigBuilder {
            country: WithCountry(country),
            env: self.env,
            telemetry: self.telemetry,
            service: self.service,
            version: self.version,
            formatter: self.formatter,
        }
    }
}

impl<F, C> SubscriberConfigBuilder<F, C, WithoutEnvironment> {
    /// Set the country in which the application is running.
    pub fn with_env(self, env: Environment) -> SubscriberConfigBuilder<F, C, WithEnvironment> {
        SubscriberConfigBuilder {
            country: self.country,
            env: WithEnvironment(env),
            telemetry: self.telemetry,
            service: self.service,
            version: self.version,
            formatter: self.formatter,
        }
    }
}

impl<F> SubscriberConfigBuilder<F, WithCountry, WithEnvironment> {
    /// Build a [`SubscriberConfig`]
    pub fn build(self) -> SubscriberConfig<F> {
        SubscriberConfig {
            country: self.country.0,
            env: self.env.0,
            telemetry: self.telemetry,
            service: self.service,
            version: self.version,
            json_formatter: self.formatter,
        }
    }
}
