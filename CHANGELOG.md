# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Automatically append /v1/traces to the collector endpoint

This is only done if a /v1/traces suffix isn't already there, meaning old configurations should continue to function

---

## [0.9.0-rc.0] - 2024-03-04

### Changed

- All log metadata is now stored in the top level object instead of under a `metadata` key.

---

## [0.8.1] - 2024-01-05

### Fixed

- Added `opentelemetry_sdk` crate in `traces` feature.

---

## [0.8.0] - 2023-12-22

### Added

- New `opentelemetry_sdk` dependency, which inherits the `rt-tokio` feature

### Changed

- Bump `opentelemetry` to 0.21 and remove the `rt-tokio` feature
- Bump `opentelemetry-otlp` to 0.14
- Bump `tracing-opentelemetry` to 0.22
- Move `rt-tokio-current-thread` to map to `["opentelemetry_sdk/rt-tokio-current-thread"]`, the tokio stuff has been moved to `opentelemetry_sdk`

---

## [0.7.2] - 2023-10-26

### Changed

- Bump `tracing-log` to 0.2

---

## [0.7.1] - 2023-10-11

### Changed

- Bump `tracing-opentelemetry` to 0.21
- Remove QA env support

---

## [0.7.0] - 2023-08-29

### Changed

- Bump otel to v0.20
- ⚠️ Increased the minimum rust version to 1.68

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



[Unreleased]: https://github.com/primait/prima_tracing.rs/compare/0.9.0...HEAD
[0.9.0]: https://github.com/primait/prima_tracing.rs/compare/0.8.1...0.9.0
[0.8.1]: https://github.com/primait/prima_tracing.rs/compare/0.8.0...0.8.1
[0.8.0]: https://github.com/primait/prima_tracing.rs/compare/0.7.2...0.8.0
[0.7.2]: https://github.com/primait/prima_tracing.rs/compare/0.7.1...0.7.2
[0.7.1]: https://github.com/primait/prima_tracing.rs/compare/0.7.0...0.7.1
[0.7.0]: https://github.com/primait/prima_tracing.rs/compare/0.6.3...0.7.0
[0.6.3]: https://github.com/primait/prima_tracing.rs/compare/0.6.2...0.6.3
[0.6.2]: https://github.com/primait/prima_tracing.rs/compare/0.6.1...0.6.2
[0.6.1]: https://github.com/primait/prima_tracing.rs/compare/0.6.0...0.6.1
[0.6.0]: https://github.com/primait/prima_tracing.rs/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/primait/prima_tracing.rs/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/primait/prima_tracing.rs/compare/0.3.1...0.4.0
