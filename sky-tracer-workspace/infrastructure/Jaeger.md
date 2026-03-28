---
tags: [infrastructure, observability]
---

# Jaeger

Distributed tracing backend — stores and visualizes traces.

## Ports
- `:16686` — Jaeger UI
- `:4317` — OTLP gRPC collector (internal)

## Used By
All services with `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT=http://jaeger:4317`

## Links
- [[OpenTelemetry]]
- [[Distributed Tracing]]
- [[Docker Compose]]
- [[Sky Tracer]]
