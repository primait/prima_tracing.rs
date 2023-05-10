//! `prima_tracing` provide an handy way for configuring the [`tracing`] crate with
//! support for JSON output formatter and integration with opentelemetry
//! # Usage
//!
//! ```rust
//!
//! use prima_tracing::{builder, configure_subscriber, Country, Environment, init_subscriber};
//!
//! let subscriber = configure_subscriber(
//!   builder("ping")
//!     .with_country(Country::Es)
//!     .with_env(Environment::Dev)
//!     .build()
//! );
//!
//! let _guard = init_subscriber(subscriber);
//! ```

mod config;

mod subscriber;

#[cfg(feature = "json-logger")]
pub mod json;
#[cfg(feature = "traces")]
pub mod telemetry;

pub use crate::config::{
    builder, Country, Environment, EnvironmentParseError, SubscriberConfig, SubscriberConfigBuilder,
};
pub use crate::subscriber::{
    configure_subscriber, init_subscriber, ContextInfo, EventFormatter, Tracing, Uninstall,
};
