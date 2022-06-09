# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Next]

## [0.4.0] - 2022-06-09

### Changed
OpenTelemetry traces are now exported using the **OTLP** format instead of the Zipkin one.  
⚠️ You will need to change the OpenTelemetry collector endpoint to `http://[HOSTNAME]:55681/v1/traces`.   
If you are using Jaeger to collect traces locally on your machine, you will also need to change the Docker image to `jaegertracing/opentelemetry-all-in-one:latest`.

[Next]: https://github.com/primait/prima_tracing.rs/compare/0.4.0...HEAD
[0.4.0]: https://github.com/primait/prima_tracing.rs/compare/0.3.1...0.4.0
