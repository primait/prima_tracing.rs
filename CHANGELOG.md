# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.3] - 2023-06-13

### Added

- Add `Common` to `Country`, since some services are common to all countries

## [0.6.2] - 2023-05-30

### Changed

- Trace resources are all set correctly

## [0.6.1] - 2023-05-30

### Changed

- `country` is actually added as an attribute to traces

## [0.6.0] - 2023-05-26

### Added

- `dev` and `live` feature sets
- ⚠️️ Added mandatory `country` field

### Changed

- Renamed feature flags
  - prima-logger-datadog -> datadog
  - prima-logger-json -> json-logger
  - prima-telemetry -> traces

  Old feature names will continue to function as aliases to the new names

- ⚠️️ `env` is now required and will not default to `Dev` anymore
- Bumped opentelemetry to v0.19

### Removed

- Removed the default `prima-logger` feature. This is a non-breaking change since the library already failed to compile without it.

## [0.5.0] - 2022-07-04

### Changed

- SubscriberConfig env field value changed from string to enumeration (Environment)
- Update dependencies  
- ⚠️ Increased the minimum rust version to 1.57.0

### Security

Avoid to depend on time 0.1 that has security issues.

## [0.4.0] - 2022-06-09

### Changed

OpenTelemetry traces are now exported using the **OTLP** format instead of the Zipkin one.  
⚠️ You will need to change the OpenTelemetry collector endpoint to `http://[HOSTNAME]:55681/v1/traces`.

If you are using Jaeger to collect traces locally on your machine, you will need to update your Docker Compose setup to the following:

```yaml
  jaeger:
    image: jaegertracing/all-in-one:1.35
    ports:
      - 16686:16686
      - 55681:55681
    environment:
      COLLECTOR_OTLP_ENABLED: true
      COLLECTOR_OTLP_HTTP_HOST_PORT: 55681
```

[Unreleased]: https://github.com/primait/prima_tracing.rs/compare/0.6.3...HEAD
[0.6.3]: https://github.com/primait/prima_tracing.rs/compare/0.6.2...0.6.3
[0.6.2]: https://github.com/primait/prima_tracing.rs/compare/0.6.1...0.6.2
[0.6.1]: https://github.com/primait/prima_tracing.rs/compare/0.6.0...0.6.1
[0.6.0]: https://github.com/primait/prima_tracing.rs/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/primait/prima_tracing.rs/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/primait/prima_tracing.rs/compare/0.3.1...0.4.0
