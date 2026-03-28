---
tags: [service, backend]
port: 3000
tech: [rust, axum, ssr]
---

# Airport Anywhere

Comprehensive airport information lookup service.

## Responsibilities
- Serve airport data from [[airports.dat]]
- Search airports by code or name
- Used by [[Flight Controller]] and [[Orbital Beacon]]

## Endpoints
- `GET /api/v1/airports` — list airports
- `GET /api/v1/airports/{code}` — get by IATA code

## Features
- [[Airport Lookup]]
- [[Distributed Tracing]]

## Links
- [[Sky Tracer]]
