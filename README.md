<h1 align="center">prima_tracing.rs</h1>
<div align="center">
 <strong>
  Utilities for configuring a <a href="https://github.com/tokio-rs/tracing">tracing</a> subscriber with support for logging and opentelemetry.
 </strong>
</div>

<br />

## Installation

Install from GitHub

```toml
prima-tracing = { git="https://github.com/primait/prima_tracing.rs", branch="master" }
```

## Cargo features

- `prima-logger-json` use JSON as output format
- `prima-telemetry` integrate opentelemetry with `opentelemetry-zipkin`

## Example

### Simple

```rust
use prima_tracing::{builder, configure_subscriber, init_subscriber};
use tracing::{info, info_span};

fn main() -> std::io::Result<()> {
    let subscriber = configure_subscriber(builder("simple").with_env("dev".to_string()).build());

    let _guard = init_subscriber(subscriber);

    let span = info_span!("MySpan");
    let _guard = span.enter();

    info!("Starting my awesome app");
    Ok(())
}
```

### JSON output

It works like the simple example, but activating the `prima-json-logger` automatically uses the JSON format as output

```rust
use prima_tracing::{builder, configure_subscriber, init_subscriber};
use tracing::{info, info_span};

fn main() -> std::io::Result<()> {
    let subscriber = configure_subscriber(builder("json").with_env("dev".to_string()).build());

    let _guard = init_subscriber(subscriber);

    let span = info_span!("MySpan");
    let _guard = span.enter();

    info!("Starting my awesome app");
    Ok(())
}

```

### Opentelemetry

```rust
use prima_tracing::{builder, configure_subscriber, init_subscriber};
use tracing::{info, info_span};

fn main() -> std::io::Result<()> {
    let subscriber = configure_subscriber(
        builder("myapp")
            .with_env("dev".to_string())
            .with_version("1.0".to_string())
            .with_telemetry(
                "http://localhost:9411/api/v2/spans".to_string(),
                "myapp".to_string(),
            )
            .build(),
    );

    let _guard = init_subscriber(subscriber);

    let span = info_span!("MySpan");
    let _guard = span.enter();

    info!("Starting my awesome app");
    Ok(())
}

```

### Custom Subscriber

```rust
use prima_tracing::json;
use tracing::{info, info_span};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

fn main() -> std::io::Result<()> {
    let subscriber = tracing_subscriber::Registry::default()
        .with(EnvFilter::from_default_env())
        .with(json::storage::layer())
        .with(json::formatter::layer("test".to_owned(), "dev".to_owned()));

    LogTracer::init().expect("Failed to set logger");
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    let span = info_span!("MySpan");
    let _guard = span.enter();

    info!("Starting my awesome app");
    Ok(())
}
```

## Running examples

### Simple

```sh
export RUST_LOG=info
cargo run --example simple
```

### Complex (OpenTelemetry)

Run [Jaeger](https://www.jaegertracing.io) locally

```sh
docker run -d -e COLLECTOR_ZIPKIN_HOST_PORT=:9411 -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p 9411:9411  jaegertracing/all-in-one:latest
```

Run pong service:

```sh
export RUST_LOG=info
cargo run --features=prima-telemetry --example pong
```

Run ping service:

```sh
export RUST_LOG=info
cargo run --features=prima-telemetry --example ping
```

Check health of ping service (which calls pong service)

```sh
curl http://localhost:8081/check
```

Open the browser at `http://localhost:16686` to inspect the traced request

### Custom formatter

```sh
export RUST_LOG=info
cargo run --features=prima-logger-json --example custom_formatter
```

### Custom subscriber with default JSON output

```sh
export RUST_LOG=info
cargo run --features=prima-logger-json --example custom_subscriber
```

### Custom subscriber with custom JSON output

```sh
export RUST_LOG=info
cargo run --features=prima-logger-json --example custom_json_subscriber
```
