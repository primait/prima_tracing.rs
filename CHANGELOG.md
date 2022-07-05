# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Next]

## [0.5.0] - 2022-07-04

### Changed
- SubscriberConfig env field value changed from string to enumeration (Environment)
- Update dependencies  
- ⚠️  Increase the minimum rust version to 1.57.0

### Security
Avoid to depend on time 0.1 that has security issues.

## [0.4.0] - 2022-06-09

### Changed
OpenTelemetry traces are now exported using the **OTLP** format instead of the Zipkin one.  
⚠️  You will need to change the OpenTelemetry collector endpoint to `http://[HOSTNAME]:55681/v1/traces`. 

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

[Next]: https://github.com/primait/prima_tracing.rs/compare/0.5.0...HEAD
[0.5.0]: https://github.com/primait/prima_tracing.rs/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/primait/prima_tracing.rs/compare/0.3.1...0.4.0
