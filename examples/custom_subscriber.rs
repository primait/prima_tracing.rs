use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use prima_tracing::json;
use tracing_actix_web::TracingLogger;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = tracing_subscriber::Registry::default()
        .with(EnvFilter::from_default_env())
        .with(json::storage::layer())
        .with(json::formatter::layer(
            "test".to_owned(),
            "common".to_owned(),
            "dev".to_owned(),
        ));

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
