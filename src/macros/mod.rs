pub mod error_report;

/// Emit a tracing error event for an error with rich, structured context.
/// It captures the error using the experimental Rust [std::error::Report](https://doc.rust-lang.org/stable/std/error/struct.Report.html)
/// and adding the type name as error.kind, the backtrace as error.trace and the error stack as error.message
///
/// # Examples
///
/// ```rust
/// use prima_tracing::trace_error;
/// # fn main() {
///
/// let error = "not a number".parse::<usize>().unwrap_err();
/// trace_error!(error, "Parsing error!");
/// trace_error!(error, "Parsing error: {}", uid="1234");
/// # }
/// ```
#[macro_export]
macro_rules! trace_error {
    ($error:expr, $message:expr) => {
        {
            $crate::trace_error!($error, $message,)
        }
    };
    ($error:expr, $message:expr, $($field:tt)*) => {
        {
            let kind = std::any::type_name_of_val(&$error);
            let error_message = format!("{:#}", $error);
            let stack = prima_tracing::macros::error_report::Report::new(&$error);
            let trace = std::backtrace::Backtrace::force_capture();

            $crate::tracing::error!(
                {
                    error.message = error_message,
                    error.kind = kind,
                    error.stack = ?stack,
                    error.trace = %trace,
                    $($field)*
                },
                "{} {}",
                $message,
                $error
            );
        }
    };
    ($error:expr, $message:expr) => {
        {
            $crate::trace_error!($error, $message, {})
        }
    };
}

/// Emit a tracing error event for anyhow error with rich, structured context.
/// It captures the error using the experimental Rust [std::error::Report](https://doc.rust-lang.org/stable/std/error/struct.Report.html)
/// and adding the type name as error.kind, the backtrace as error.trace and the error stack as error.message
///
/// # Examples
///
/// ```rust
/// use anyhow::anyhow
/// use prima_tracing::trace_error;
/// # fn main() {
///
/// let error = anyhow!("an error");
/// trace_anyhow_error!(error, "Throw error!");
/// trace_anyhow_error!(error, "Throw error!", uid="1234");
/// # }
/// ```
#[cfg(feature = "anyhow")]
#[macro_export]
macro_rules! trace_anyhow_error {
    ($error:expr, $message:expr) => {
        {
            $crate::trace_anyhow_error!($error, $message,)
        }
    };
    ($error:expr, $message:expr, $($field:tt)*) => {
        {
            let kind = std::any::type_name_of_val(&$error.root_cause());
            let error_message = format!("{:#}", $error);
            let std_err: &(dyn std::error::Error + 'static) = $error.as_ref();
            let stack = prima_tracing::macros::error_report::Report::new(std_err);

            $crate::tracing::error!(
                {
                    error.message = error_message,
                    error.kind = kind,
                    error.stack = ?stack,
                    error.trace = %$error.backtrace(),
                    $($field)*
                },
                "{} {:?}",
                $message,
                $error
            );
        }
    };
    ($error:expr, $message:expr) => {
        {
            $crate::trace_anyhow_error!($error, $message,)
        }
    };
}
