use prima_tracing::{builder, configure_subscriber, init_subscriber};
use tracing::{info, info_span};

fn main() -> std::io::Result<()> {
    let subscriber = configure_subscriber(builder("simple").with_env("dev".to_string()).build());

    let _trace_guard = init_subscriber(subscriber);

    let span = info_span!("MySpan");
    let _guard = span.enter();

    info!("Starting my awesome app");
    Ok(())
}
