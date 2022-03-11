use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use prima_tracing::{builder, configure_subscriber, init_subscriber};
use tracing_actix_web::TracingLogger;
// RUN docker run -d -e COLLECTOR_ZIPKIN_HOST_PORT=:9411 -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p 9411:9411  jaegertracing/all-in-one:latest
// for collecting spans into Jaeger
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = configure_subscriber(
        builder("pong")
            .with_env("dev".to_string())
            .with_version("1.0".to_string())
            .with_telemetry(
                "http://localhost:9411/api/v2/spans".to_string(),
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
