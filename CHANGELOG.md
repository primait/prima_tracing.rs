# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

---

## [0.18.0] - 2025-10-31

### Changed

- Bumped opentelemetry version to 0.31
  - Removed internal access to `OtelData.builder`, which is no longer public in
    `tracing_opentelemetry` 0.32
- MSRV is now 1.83
- `ErrorLayer` now emits standard OpenTelemetry **`exception` events** and sets
  span status using `Status::error`
  - Legacy `error.*` attributes are still attached to the `exception` event
- `KubeEnvLayer` and `VersionLayer` responsibilities moved to **Resource
  attributes**: K8s environment variables and service version are now attached
  once at provider creation instead of being injected per-span
  - `service.version` is now populated from `SubscriberConfig.version` using
    semantic conventions

### Removed

- Deprecated `error.stack`, `error.kind`, and other non-standard span attributes
- Removed redundant `version` attribute in favor of standardized
  `service.version`

### Added

- Dependency on `opentelemetry_semantic_conventions` to avoid hard-coded
  attribute keys

---

## [0.17.0] - 2025-06-24

### Updated

- Bumped opentelemetry version to 0.30
- MSRV is now 1.82

---

## [0.16.0] - 2025-04-16

### Updated

- Bumped opentelemetry version to 0.29

---

## [0.15.0] - 2025-02-26

### Updated

- Bumped opentelemetry version to 0.28
- MSRV is now 1.81

---

## [0.14.2] - 2025-02-17

### Added

- `report_error` macro

---

## [0.14.1] - 2024-12-03

### Changed

- No longer set the `tracing` max level features. This allows you to enable more
  verbose logging on runtime via the `RUST_LOG` environment variable. This
  should not affect most users, as the env filter by default is already set to
  `error`. In order to restore previous behavior you can enable the `tracing`
  features yourself

```
tracing = {version = "0.1", features = ["max_level_debug", "release_max_level_info"]}
```

---

## [0.14.0] - 2024-12-03

### Updated

- Bumped opentelemetry version to 0.27

---

## [0.13.1] - 2024-10-18

### Added

- Error impl for EnvironmentParseError

---

## [0.13.0] - 2024-10-09

### Updated

- Bumped opentelemetry version to 0.26

---

## [0.12.0] - 2024-09-17

### Updated

- Bumped opentelemetry version to 0.25

---

## [0.11.1] - 2024-09-12

### Fixed

- Shutdown the tracing provider before exiting
  ([bug](https://github.com/open-telemetry/opentelemetry-rust/issues/1961))

---

## [0.11.0] - 2024-07-24

### Updated

- Bumped opentelemetry version to 0.24

---

## [0.10.0] - 2024-07-08

--

## (yanked) [0.9.5] - 2024-07-03

### Updated

- Bumped opentelemetry version to 0.23

---

## [0.9.4] - 2024-05-20

### Fixed

- Added `error.type`, `error.kind`, `error.stack` and `error.message` to trace
  events (logs).

---

## [0.9.3] - 2024-05-14

### Fixed

- Added `error.kind` to `ErrorLayer` hopefully fixing datadog error tracking.

---

## [0.9.2] - 2024-05-03

### Added

- Added metadata based on the kubernetes environment variables:
  - KUBE_APP_PART_OF
  - KUBE_APP_MANAGED_BY
  - KUBE_APP_VERSION
  - KUBE_APP_INSTANCE

---

## [0.9.1] - 2024-04-08

### Added

- Added an `ErrorLayer` to add `error.message`, `error.type` and `error.stack`
  to the log metadata when a `tracing::Event` is an `Error`.

---

## [0.9.0] - 2024-03-26

No new changes since 0.9.0-rc.1

### Changed

- Automatically append /v1/traces to the collector endpoint

This is only done if a /v1/traces suffix isn't already there, meaning old
configurations should continue to function

- All log metadata is now stored in the top level object instead of under a
  `metadata` key.

---

## [0.9.0-rc.1] - 2024-03-11

### Changed

- Automatically append /v1/traces to the collector endpoint

This is only done if a /v1/traces suffix isn't already there, meaning old
configurations should continue to function

---

## [0.9.0-rc.0] - 2024-03-04

### Changed

- All log metadata is now stored in the top level object instead of under a
  `metadata` key.

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
- Move `rt-tokio-current-thread` to map to
  `["opentelemetry_sdk/rt-tokio-current-thread"]`, the tokio stuff has been
  moved to `opentelemetry_sdk`

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

- Removed the default `prima-logger` feature. This is a non-breaking change
  since the library already failed to compile without it.

## [0.5.0] - 2022-07-04

### Changed

- SubscriberConfig env field value changed from string to enumeration
  (Environment)
- Update dependencies
- ⚠️ Increased the minimum rust version to 1.57.0

### Security

Avoid to depend on time 0.1 that has security issues.

## [0.4.0] - 2022-06-09

### Changed

OpenTelemetry traces are now exported using the **OTLP** format instead of the
Zipkin one.\
⚠️ You will need to change the OpenTelemetry collector endpoint to
`http://[HOSTNAME]:55681/v1/traces`.

If you are using Jaeger to collect traces locally on your machine, you will need
to update your Docker Compose setup to the following:

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

[Unreleased]: https://github.com/primait/prima_tracing.rs/compare/0.18.0...HEAD
[0.18.0]: https://github.com/primait/prima_tracing.rs/compare/0.17.0...0.18.0
[0.17.0]: https://github.com/primait/prima_tracing.rs/compare/0.16.0...0.17.0
[0.16.0]: https://github.com/primait/prima_tracing.rs/compare/0.15.0...0.16.0
[0.15.0]: https://github.com/primait/prima_tracing.rs/compare/0.14.2...0.15.0
[0.14.2]: https://github.com/primait/prima_tracing.rs/compare/0.14.1...0.14.2
[0.14.1]: https://github.com/primait/prima_tracing.rs/compare/0.14.0...0.14.1
[0.14.0]: https://github.com/primait/prima_tracing.rs/compare/0.13.1...0.14.0
[0.13.1]: https://github.com/primait/prima_tracing.rs/compare/0.13.0...0.13.1
[0.13.0]: https://github.com/primait/prima_tracing.rs/compare/0.12.0...0.13.0
[0.12.0]: https://github.com/primait/prima_tracing.rs/compare/0.11.1...0.12.0
[0.11.1]: https://github.com/primait/prima_tracing.rs/compare/0.11.0...0.11.1
[0.11.0]: https://github.com/primait/prima_tracing.rs/compare/0.10.0...0.11.0
[0.10.0]: https://github.com/primait/prima_tracing.rs/compare/0.9.5...0.10.0
[0.9.4]: https://github.com/primait/prima_tracing.rs/compare/0.9.3...0.9.4
[0.9.3]: https://github.com/primait/prima_tracing.rs/compare/0.9.2...0.9.3
[0.9.2]: https://github.com/primait/prima_tracing.rs/compare/0.9.1...0.9.2
[0.9.1]: https://github.com/primait/prima_tracing.rs/compare/0.9.0...0.9.1
[0.9.0]: https://github.com/primait/prima_tracing.rs/compare/0.9.0-rc.1...0.9.0
[0.9.0-rc.1]: https://github.com/primait/prima_tracing.rs/compare/0.9.0-rc.0...0.9.0-rc.1
[0.9.0-rc.0]: https://github.com/primait/prima_tracing.rs/compare/0.8.1...0.9.0-rc.0
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
