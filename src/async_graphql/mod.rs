use std::sync::Arc;

use async_graphql::extensions::{
    Extension, ExtensionContext, ExtensionFactory, NextParseQuery, NextRequest, NextResolve,
    NextValidation, ResolveInfo,
};
use async_graphql::parser::types::ExecutableDocument;
use async_graphql::{Response, ServerError, ServerResult, ValidationResult, Value, Variables};
use tracing::{info_span, Instrument};

/// A GraphQL extension that traces every executed root-level field via `tracing`.
///
/// For each root-level GraphQL field it emits:
/// - An `INFO` span named `graphql_root_field` containing the field name,
///   the operation type, the parent type, and the return type
/// - `TRACE` logs for start/completion
/// - An `ERROR` log if the field resolution returned errors
///
/// Additionally, it proactively tracks schema contract violations to surface breaking changes:
/// - An `ERROR` log when the incoming query document fails to parse (syntax error)
/// - An `ERROR` log per validation error when the query references fields or types that don't
///   exist in the schema (e.g. unknown fields, wrong argument types, missing required args)
pub struct TracingRootFieldsExtension {
    schema: Arc<str>,
}

impl TracingRootFieldsExtension {
    pub fn new(schema: impl Into<Arc<str>>) -> Self {
        Self {
            schema: schema.into(),
        }
    }
}

impl ExtensionFactory for TracingRootFieldsExtension {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(TracingRootFieldsExtensionInstance {
            schema: self.schema.clone(),
        })
    }
}

struct TracingRootFieldsExtensionInstance {
    schema: Arc<str>,
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
            tracing::error!(
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
                tracing::error!(
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
            tracing::trace!("graphql field started");
            next.run(ctx, info)
                .await
                .inspect(|_| tracing::trace!("graphql field completed successfully"))
                .inspect_err(|err| tracing::error!(error = %err, "graphql root resolver {} resolved with error", root_field_name))
        }
        .instrument(span)
        .await
    }
}
