---
tags: [service, backend]
port: 3003
tech: [rust, axum]
---

# Tower of Babel

Flight aggregation and position service — central data hub.

## Responsibilities
- Aggregate flight and position data
- Used by [[Delay-O-Rama]] and [[Sky Nexus]]
- Delegates to [[Flight Controller]] and [[Orbital Beacon]]

## Endpoints
- `GET /api/v1/babel` — get aggregated flight data

## Features
- [[Flight Tracking]]
- [[Delay Monitoring]]
- [[Distributed Tracing]]

## Links
- [[Sky Tracer]]
