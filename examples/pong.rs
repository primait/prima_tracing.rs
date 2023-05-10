use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use prima_tracing::{builder, configure_subscriber, init_subscriber, Country, Environment};
use tracing_actix_web::TracingLogger;

// This example requires Jaeger to be running in order to collect traces (see the README)
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = configure_subscriber(
        builder("pong")
            .with_env(Environment::Dev)
            .with_country(Country::Es)
            .with_version("1.0".to_string())
            .with_telemetry(
                "http://localhost:55681/v1/traces".to_string(),
                "pong".to_string(),
            )
            .build(),
    );

    let _guard = init_subscriber(subscriber);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(TracingLogger::default())
            .route("/check", web::get().to(check))
    })
    .bind("127.0.0.1:8082")?
    .run()
    .await
}

#[tracing::instrument]
async fn check() -> impl Responder {
    tracing::info!("Checking heath status");
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{}")
}
