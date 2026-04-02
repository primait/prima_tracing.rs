use std::sync::Arc;

// `tracing::event!` embeds the level into a `static` metadata block and therefore requires a
// compile-time constant. This local macro dispatches to the appropriate level-specific macro so
// that we can use a runtime `tracing::Level` value.
macro_rules! log_at_level {
    ($level:expr, $($args:tt)*) => {
        match $level {
            ::tracing::Level::TRACE => ::tracing::trace!($($args)*),
            ::tracing::Level::DEBUG => ::tracing::debug!($($args)*),
            ::tracing::Level::INFO  => ::tracing::info!($($args)*),
            ::tracing::Level::WARN  => ::tracing::warn!($($args)*),
            ::tracing::Level::ERROR => ::tracing::error!($($args)*),
        }
    };
}

use async_graphql::extensions::{
    Extension, ExtensionContext, ExtensionFactory, NextParseQuery, NextRequest, NextResolve,
    NextValidation, ResolveInfo,
};
use async_graphql::parser::types::ExecutableDocument;
use async_graphql::{Response, ServerError, ServerResult, ValidationResult, Value, Variables};
use tracing::{info_span, Instrument, Level};

/// A GraphQL extension that traces every executed root-level field via `tracing`.
///
/// For each root-level GraphQL field it emits:
/// - An `INFO` span named `graphql_root_field` containing the field name,
///   the operation type, the parent type, and the return type
/// - Configurable-level logs for field start/completion (default: `TRACE`)
/// - A configurable-level log if the field resolution returned errors (default: `TRACE`)
///
/// Additionally, it proactively tracks schema contract violations to surface breaking changes:
/// - A configurable-level log when the incoming query document fails to parse (default: `TRACE`)
/// - A configurable-level log per validation error when the query references fields or types
///   that don't exist in the schema (e.g. unknown fields, wrong argument types, missing required
///   args) (default: `TRACE`)
///
/// Use the builder methods to override the log level for each category:
///
/// ```rust
/// use prima_tracing::async_graphql::TracingRootFieldsExtension;
/// use tracing::Level;
///
/// TracingRootFieldsExtension::new("my_schema")
///     .with_parse_level(Level::ERROR)
///     .with_validation_level(Level::ERROR)
///     .with_resolve_level(Level::WARN)
///     .with_field_started_level(Level::DEBUG)
///     .with_field_completed_level(Level::DEBUG);
/// ```
pub struct TracingRootFieldsExtension {
    schema: Arc<str>,
    /// Log level emitted when a query document fails to parse.
    parse_level: Level,
    /// Log level emitted per validation error (unknown fields, wrong types, missing args, …).
    validation_level: Level,
    /// Log level emitted when a root-field resolver returns an error.
    resolve_level: Level,
    /// Log level emitted when a root-field resolver begins execution.
    field_started_level: Level,
    /// Log level emitted when a root-field resolver completes successfully.
    field_completed_level: Level,
}

impl TracingRootFieldsExtension {
    pub fn new(schema: impl Into<Arc<str>>) -> Self {
        Self {
            schema: schema.into(),
            parse_level: Level::TRACE,
            validation_level: Level::TRACE,
            resolve_level: Level::TRACE,
            field_started_level: Level::TRACE,
            field_completed_level: Level::TRACE,
        }
    }

    /// Set the log level for query parse errors.
    pub fn with_parse_level(mut self, level: Level) -> Self {
        self.parse_level = level;
        self
    }

    /// Set the log level for schema validation errors.
    pub fn with_validation_level(mut self, level: Level) -> Self {
        self.validation_level = level;
        self
    }

    /// Set the log level for root-field resolver errors.
    pub fn with_resolve_level(mut self, level: Level) -> Self {
        self.resolve_level = level;
        self
    }

    /// Set the log level emitted when a root-field resolver begins execution.
    pub fn with_field_started_level(mut self, level: Level) -> Self {
        self.field_started_level = level;
        self
    }

    /// Set the log level emitted when a root-field resolver completes successfully.
    pub fn with_field_completed_level(mut self, level: Level) -> Self {
        self.field_completed_level = level;
        self
    }
}

impl ExtensionFactory for TracingRootFieldsExtension {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(TracingRootFieldsExtensionInstance {
            schema: self.schema.clone(),
            parse_level: self.parse_level,
            validation_level: self.validation_level,
            resolve_level: self.resolve_level,
            field_started_level: self.field_started_level,
            field_completed_level: self.field_completed_level,
        })
    }
}

struct TracingRootFieldsExtensionInstance {
    schema: Arc<str>,
    parse_level: Level,
    validation_level: Level,
    resolve_level: Level,
    field_started_level: Level,
    field_completed_level: Level,
}

#[async_trait::async_trait]
impl Extension for TracingRootFieldsExtensionInstance {
    async fn request(&self, ctx: &ExtensionContext<'_>, next: NextRequest<'_>) -> Response {
        let span = info_span!("graphql_request", schema = self.schema.as_ref());
        next.run(ctx).instrument(span).await
    }

    async fn parse_query(
        &self,
        ctx: &ExtensionContext<'_>,
        query: &str,
        variables: &Variables,
        next: NextParseQuery<'_>,
    ) -> ServerResult<ExecutableDocument> {
        next.run(ctx, query, variables).await.inspect_err(|err| {
            log_at_level!(
                self.parse_level,
                error = %err,
                "graphql query parse error: request does not match expected schema syntax"
            );
        })
    }

    async fn validation(
        &self,
        ctx: &ExtensionContext<'_>,
        next: NextValidation<'_>,
    ) -> Result<ValidationResult, Vec<ServerError>> {
        next.run(ctx).await.inspect_err(|errors| {
            for err in errors {
                log_at_level!(
                    self.validation_level,
                    error = %err.message,
                    locations = ?err.locations,
                    "graphql validation error: request violates schema contract"
                );
            }
        })
    }

    async fn resolve(
        &self,
        ctx: &ExtensionContext<'_>,
        info: ResolveInfo<'_>,
        next: NextResolve<'_>,
    ) -> ServerResult<Option<Value>> {
        if info.path_node.parent.is_some() {
            return next.run(ctx, info).await;
        }

        let root_field_name = info.path_node.field_name();

        let registry = &ctx.schema_env.registry;
        let operation_type = if Some(info.parent_type) == registry.mutation_type.as_deref() {
            "mutation"
        } else if Some(info.parent_type) == registry.subscription_type.as_deref() {
            "subscription"
        } else {
            "query"
        };

        let span = info_span!(
            "graphql_root_field",
            name = root_field_name,
            operation_type = operation_type,
            parent_type = %info.parent_type,
            return_type = %info.return_type
        );
        async move {
            log_at_level!(self.field_started_level, "graphql field started");
            next.run(ctx, info)
                .await
                .inspect(|_| log_at_level!(self.field_completed_level, "graphql field completed successfully"))
                .inspect_err(|err| log_at_level!(self.resolve_level, error = %err, "graphql root resolver {} resolved with error", root_field_name))
        }
        .instrument(span)
        .await
    }
}
