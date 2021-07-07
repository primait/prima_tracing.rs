//! `prima_tracing` provide an handy way for configuring the [`tracing`] crate with
//! support for JSON output formatter and integration with opentelemetry
//! # Usage
//!
//!
//! ```rust
//!
//! use prima_tracing::{builder, configure_subscriber, init_subscriber};
//!
//! fn main() {
//!    let subscriber = configure_subscriber(
//!        builder("ping")
//!        .with_env("dev".to_string())
//!        .build()
//!    );
//!
//!    let _guard = init_subscriber(subscriber);
//! }
//! ```

mod config;

mod subscriber;

#[cfg(feature = "prima-logger-json")]
pub mod json;
#[cfg(feature = "prima-telemetry")]
pub mod telemetry;

pub use crate::config::{builder, SubscriberConfig, SubscriberConfigBuilder};
pub use crate::subscriber::{
    configure_subscriber, init_subscriber, ContextInfo, EventFormatter, Tracing,
};
