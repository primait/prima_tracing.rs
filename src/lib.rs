//! `prima_tracing` provide an handy way for configuring the [`tracing`] crate with
//! support for JSON output formatter and integration with opentelemetry
//! # Usage
//!
//! ```rust
//!
//! use prima_tracing::{builder, configure_subscriber, Environment, init_subscriber};
//!
//! let subscriber = configure_subscriber(
//!   builder("ping")
//!     .with_env(Environment::Dev)
//!     .build()
//! );
//!
//! let _guard = init_subscriber(subscriber);
//! ```

mod config;

mod subscriber;

#[cfg(feature = "prima-logger-json")]
pub mod json;
#[cfg(feature = "prima-telemetry")]
pub mod telemetry;

pub use crate::config::{
    builder, Environment, EnvironmentParseError, SubscriberConfig, SubscriberConfigBuilder,
};
pub use crate::subscriber::{
    configure_subscriber, init_subscriber, ContextInfo, EventFormatter, Tracing, Uninstall,
};
