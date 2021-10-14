use std::sync::Arc;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use prima_tracing::{builder, configure_subscriber, init_subscriber};
use tracing_actix_web::TracingLogger;

use prima_bridge::{prelude::*, Bridge, Request};
// RUN docker run -d -e COLLECTOR_ZIPKIN_HOST_PORT=:9411 -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p 9411:9411  jaegertracing/all-in-one:latest
// for collecting spans into Jaeger
//
//

type HttpClient = Arc<Bridge>;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = configure_subscriber(
        builder("ping")
            .with_env("dev".to_string())
            .with_version("1.0".to_string())
            .with_telemetry(
                "http://localhost:9411/api/v2/spans".to_string(),
                "ping".to_string(),
            )
            .build(),
    );

    let _guard = init_subscriber(subscriber);

    let bridge = Arc::new(Bridge::new("http://localhost:8082".parse().unwrap()));
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
