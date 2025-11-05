use std::collections::HashMap;

use tracing::{
    field::{Field, Visit},
    span, Subscriber,
};
use tracing_subscriber::{layer::Context, Layer};

#[derive(Default)]
pub struct PrimaJsonVisitor<'a> {
    fields: HashMap<&'a str, serde_json::Value>,
}
/// Tracing layer providing a store for attributes associated to spans.
/// Inspired (almost a copy-paste) of `JsonStorageLayer` from `tracing-bunyan-formatter`
#[derive(Default)]
pub struct PrimaJsonStorage;

/// Build a [`PrimaJsonStorage`] layer
pub fn layer() -> PrimaJsonStorage {
    PrimaJsonStorage
}
impl<S> Layer<S> for PrimaJsonStorage
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();

        if let Some(visitor) = extensions.get_mut::<PrimaJsonVisitor>() {
            attrs.record(visitor);
        } else {
            let mut visitor = PrimaJsonVisitor::default();
            attrs.record(&mut visitor);
            extensions.insert(visitor);
        }
    }

    fn on_record(&self, id: &span::Id, values: &span::Record<'_>, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();

        if let Some(visitor) = extensions.get_mut::<PrimaJsonVisitor>() {
            values.record(visitor);
        } else {
            let mut visitor = PrimaJsonVisitor::default();
            values.record(&mut visitor);
            extensions.insert(visitor);
        }
    }
}

impl<'a> PrimaJsonVisitor<'a> {
    pub fn fields(&self) -> &HashMap<&'a str, serde_json::Value> {
        &self.fields
    }
}

impl<'a> PrimaJsonVisitor<'a> {
    pub fn get<T: FromValue<'a>>(&'a self, field: &'a str) -> Option<T> {
        self.fields.get(field).and_then(T::from_value)
    }
}

impl Visit for PrimaJsonVisitor<'_> {
    fn record_i64(&mut self, field: &Field, value: i64) {
        self.fields
            .insert(field.name(), serde_json::Value::from(value));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.fields
            .insert(field.name(), serde_json::Value::from(value));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.fields
            .insert(field.name(), serde_json::Value::from(value));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.fields
            .insert(field.name(), serde_json::Value::from(value));
    }
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.fields.insert(
            field.name(),
            serde_json::Value::from(format!("{:?}", value)),
        );
    }
}

pub trait FromValue<'a> {
    fn from_value(value: &'a serde_json::Value) -> Option<Self>
    where
        Self: Sized;
}

impl<'a> FromValue<'a> for &'a str {
    fn from_value(value: &'a serde_json::Value) -> Option<Self>
    where
        Self: Sized,
    {
        match value {
            serde_json::Value::String(string) => Some(string),
            _ => None,
        }
    }
}

impl<'a> FromValue<'a> for u32 {
    fn from_value(value: &'a serde_json::Value) -> Option<Self>
    where
        Self: Sized,
    {
        match value {
            serde_json::Value::Number(number) => number.as_u64().map(|number| number as u32),
            _ => None,
        }
    }
}
