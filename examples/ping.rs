use std::sync::Arc;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use prima_bridge::{prelude::*, Bridge, Request};
use prima_tracing::{builder, configure_subscriber, init_subscriber, Country, Environment};
use tracing_actix_web::TracingLogger;

type HttpClient = Arc<Bridge>;

// This example requires Jaeger to be running in order to collect traces (see the README)
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = configure_subscriber(
        builder("ping")
            .with_env(Environment::Dev)
            .with_country(Country::Common)
            .with_version("1.0".to_string())
            .with_telemetry(
                "http://localhost:55681/v1/traces".to_string(),
                "ping".to_string(),
            )
            .build(),
    );

    let _guard = init_subscriber(subscriber);

    let bridge: Arc<Bridge> =
        Arc::new(Bridge::builder().build("http://localhost:8082".parse().unwrap()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(bridge.clone()))
            .wrap(Logger::default())
            .wrap(TracingLogger::default())
            .route("/check", web::get().to(check))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}

#[tracing::instrument]
async fn check(data: web::Data<HttpClient>) -> HttpResponse {
    tracing::info!("Checking heath status of ping service");

    match Request::rest(data.as_ref()).to("/check").send().await {
        Ok(response) if response.is_ok() => HttpResponse::Ok()
            .content_type("application/json")
            .body("{}"),
        _ => HttpResponse::InternalServerError().finish(),
    }
}
