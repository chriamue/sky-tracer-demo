---
tags: [service, backend]
port: 3002
tech: [rust, axum]
---

# Orbital Beacon

Satellite positioning service — calculates real-time flight positions.

## Responsibilities
- Manage satellites
- Calculate flight position from satellite data
- Depends on [[Airport Anywhere]] for coordinates

## Endpoints
- `GET /api/v1/satellites` — list satellites
- `POST /api/v1/satellites` — create satellite
- `POST /api/v1/satellites/position` — calculate position

## Features
- [[Satellite Positioning]]
- [[Distributed Tracing]]

## Links
- [[Sky Tracer]]
