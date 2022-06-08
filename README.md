<h1 align="center">prima_tracing.rs</h1>
<div align="center">
 <strong>
  Utilities for configuring a <a href="https://github.com/tokio-rs/tracing">tracing</a> subscriber with support for logging and opentelemetry.
 </strong>
</div>

<br />

## Installation

Install from [crates.io](https://crates.io/crates/prima-tracing)

```toml
prima-tracing = "0.3.1"
```

## Cargo features

- `prima-logger-json` outputs traces to standard output in JSON format
- `prima-logger-datadog` extends `prima-logger-json` output
  with [trace and span information](https://docs.datadoghq.com/tracing/connect_logs_and_traces/opentelemetry/) allowing
  Datadog to connect logs and traces
- `prima-telemetry` exports OpenTelemetry traces using the [opentelemetry-otlp](https://crates.io/crates/opentelemetry-otlp) exporter
- `rt-tokio-current-thread` configures the OpenTelemetry tracer to use Tokio’s current thread runtime
  (e.g. `actix_web::main`). Without this feature, the Tokio multi-thread runtime is used by default.

## Usage examples

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

### OpenTelemetry

You need to have an OpenTelemetry collector (such as Jaeger) running locally.

```rust
use prima_tracing::{builder, configure_subscriber, init_subscriber};
use tracing::{info, info_span};

fn main() -> std::io::Result<()> {
    let subscriber = configure_subscriber(
        builder("myapp")
            .with_env("dev".to_string())
            .with_version("1.0".to_string())
            .with_telemetry(
                "http://localhost:55681/v1/traces".to_string(),
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
RUST_LOG=info cargo run --example simple
```

### Complex (OpenTelemetry)

Run [Jaeger](https://www.jaegertracing.io) locally

```sh
docker run --rm -d -p 16686:16686 -p 55681:55681 jaegertracing/opentelemetry-all-in-one:latest
```

Run pong service:

```sh
RUST_LOG=info cargo run --features=prima-telemetry --example pong
```

Run ping service:

```sh
RUST_LOG=info cargo run --features=prima-telemetry --example ping
```

Check health of ping service (which calls pong service)

```sh
curl http://localhost:8081/check
```

Open the browser at <http://localhost:16686> to inspect the traced request

#### OpenTelemetry + JSON logger with Datadog correlation IDs

```sh
RUST_LOG=info cargo run --features=prima-logger-datadog,prima-telemetry --example datadog_json_logger
```

### Custom formatter

```sh
RUST_LOG=info cargo run --features=prima-logger-json --example custom_formatter
```

### Custom subscriber with default JSON output

```sh
RUST_LOG=info cargo run --features=prima-logger-json --example custom-subscriber
```

### Custom subscriber with custom JSON output

```sh
RUST_LOG=info cargo run --features=prima-logger-json --example custom-json-subscriber
```