---
tags: [concept, observability]
---

# OpenTelemetry

Vendor-neutral observability framework for distributed systems.

## Key Concepts
- **Trace** — complete request flow across services
- **Span** — single operation within a trace
- **Context** — carries trace/span IDs between services

## In Sky Tracer
- All Rust services use `init-tracing-opentelemetry`
- Traces exported via OTLP gRPC to [[Jaeger]]

## Links
- [[Distributed Tracing]]
- [[Jaeger]]
- [[Sky Tracer]]
