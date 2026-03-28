---
tags: [infrastructure]
---

# Traefik

Reverse proxy and load balancer routing all external traffic.

## Ports
- `:8000` — web entrypoint
- `:8080` — Traefik dashboard

## Routes
| Path | Service |
|------|---------|
| `/airports` | [[Airport Anywhere]] |
| `/flights` | [[Flight Controller]] |
| `/satellites` | [[Orbital Beacon]] |
| `/babel` | [[Tower of Babel]] |
| `/cockpit` | [[Cockpit]] |
| `/delays` | [[Delay-O-Rama]] |
| `/flightmare` | [[Flightmare Tracker]] |
| `/mcp/*` | [[Sky Nexus]] |

## Links
- [[Docker Compose]]
- [[Sky Tracer]]
