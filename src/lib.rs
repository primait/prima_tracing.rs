//! `prima_tracing` provide an handy way for configuring the [`tracing`] crate with
//! support for JSON output formatter and integration with opentelemetry
//! # Usage
//!
//! ```rust
//!
//! use prima_tracing::{builder, configure_subscriber, Country, Environment, init_subscriber};
//! # #[cfg(not(feature = "traces"))]
//! # {
//! let subscriber = configure_subscriber(
//!   builder("ping")
//!     .with_country(Country::Common)
//!     .with_env(Environment::Dev)
//!     .build()
//! );
//!
//! let _guard = init_subscriber(subscriber);
//! # }
//! ```

#[cfg(feature = "traces")]
#[macro_use]
pub mod macros;

mod config;
mod subscriber;

#[cfg(feature = "json-logger")]
pub mod json;
#[cfg(feature = "traces")]
pub mod layer;
#[cfg(feature = "traces")]
pub mod resources;
#[cfg(feature = "traces")]
pub mod telemetry;

pub use crate::config::{
    builder, Country, Environment, EnvironmentParseError, SubscriberConfig, SubscriberConfigBuilder,
};
pub use crate::subscriber::{
    configure_subscriber, init_subscriber, ContextInfo, EventFormatter, Tracing, Uninstall,
};
pub use tracing;

/// Create a tracing error event, casting the error to &dyn [std::error::Error] for [layer::ErrorLayer],
/// and adding the type name as error.kind.
///
/// Usage:
/// ```
/// use prima_tracing::report_error;
///
/// let error = "not a number".parse::<usize>().unwrap_err();
/// report_error!(error, "Parsing error!");
/// ```
///
/// You can also add use add attributes and do things, just like with a regular [tracing::error]
/// macro call
/// ```
/// use prima_tracing::report_error;
/// # let input = "not a number";
/// # let uid = "1223";
///
/// let error = input.parse::<usize>().unwrap_err();
/// report_error!(error, input, user=uid, "Parsing error: {}", error);
/// ```
#[macro_export]
macro_rules! report_error {
    ($error:expr, $($args:tt)*) => {
        {
          let kind = ::std::any::type_name_of_val(&$error);
          $crate::tracing::error!(error.kind = kind, error = &$error as &dyn ::std::error::Error, $($args)+)
        }
    };
}
