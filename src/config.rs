#[cfg(feature = "prima-logger-json")]
use crate::json::formatter::DefaultEventFormatter;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[cfg(not(feature = "prima-logger-json"))]
use crate::subscriber::NopEventFormatter;
/// `SubscriberConfig` configuration built via [`SubscriberConfigBuilder`]
/// It contains
/// - Application env
/// - Telemetry config
/// - JSON formatter
pub struct SubscriberConfig<T> {
    pub env: Environment,
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
            env: Environment::Dev,
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

/// All the possible environments in which the application can run.
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Environment {
    Dev,
    Qa,
    Staging,
    Production,
}

impl FromStr for Environment {
    type Err = EnvironmentParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" => Ok(Self::Dev),
            "qa" => Ok(Self::Qa),
            "staging" => Ok(Self::Staging),
            "production" => Ok(Self::Production),
            _ => Err(EnvironmentParseError(s.to_string())),
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Dev => "dev",
            Self::Qa => "qa",
            Self::Staging => "staging",
            Self::Production => "production",
        };
        f.write_str(str)
    }
}

#[derive(Debug)]
pub struct EnvironmentParseError(String);

impl Display for EnvironmentParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} is not a valid environment string. Allowed strings are 'dev', 'qa', 'staging' and 'production'.",
            &self.0
        ))
    }
}

impl Error for EnvironmentParseError {}

pub struct SubscriberConfigBuilder<T>(SubscriberConfig<T>);

impl<T> SubscriberConfigBuilder<T> {
    /// Build a [`SubscriberConfig`]
    pub fn build(self) -> SubscriberConfig<T> {
        self.0
    }

    /// Set the environment in which the application is running.
    /// If you do not specify it, it defaults to `Environment::Dev`.
    pub fn with_env(mut self, env: Environment) -> Self {
        self.0.env = env;
        self
    }

    /// Set the application version.
    pub fn with_version(mut self, version: String) -> Self {
        self.0.version = Some(version);
        self
    }

    /// Set the telemetry collector URL and the service name included in telemetry traces.
    pub fn with_telemetry(mut self, collector_url: String, service_name: String) -> Self {
        self.0.telemetry = Some(TelemetryConfig {
            collector_url,
            service_name,
        });

        self
    }

    /// Set the custom JSON formatter to be used when the feature `prima-logger-json` is activated.
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
