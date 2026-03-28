---
tags: [service, backend]
port: 3001
tech: [rust, axum]
---

# Flight Controller

Manages flight schedules, data, and position queries.

## Responsibilities
- CRUD for flights
- Query flight positions via [[Orbital Beacon]]
- Used by [[Cockpit]], [[Flightmare Tracker]], [[Tower of Babel]]

## Endpoints
- `GET /api/v1/flights` — list flights
- `POST /api/v1/flights` — create flight
- `GET /api/v1/flights/{number}` — get flight

## Features
- [[Flight Tracking]]
- [[Distributed Tracing]]

## Links
- [[Sky Tracer]]
