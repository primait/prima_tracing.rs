use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use chrono::{DateTime, Utc};
use prima_tracing::{json, ContextInfo, EventFormatter};
use serde::Serialize;
use serde_json;
use tracing_actix_web::TracingLogger;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = tracing_subscriber::Registry::default()
        .with(EnvFilter::from_default_env())
        .with(json::storage::layer())
        .with(
            json::formatter::layer("test".to_owned(), "dev".to_owned())
                .with_formatter(MyCustomFormatter {}),
        );

    LogTracer::init().expect("Failed to set logger");
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(TracingLogger::default())
            .route("/check", web::get().to(check))
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await
}

#[tracing::instrument]
async fn check() -> HttpResponse {
    tracing::info!("Checking heath status");
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{}")
}

pub struct MyCustomFormatter {}

impl EventFormatter for MyCustomFormatter {
    fn format_event<S>(
        &self,
        _event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
        info: ContextInfo<'_>,
    ) -> Result<Vec<u8>, std::io::Error>
    where
        S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    {
        serde_json::to_vec(&MyEvent {
            timestamp: Utc::now(),
            app_name: info.app_name(),
            environment: info.environment(),
        })
        .map(Ok)?
    }
}
#[derive(Serialize)]
struct MyEvent<'a> {
    timestamp: DateTime<Utc>,
    app_name: &'a str,
    environment: &'a str,
}
