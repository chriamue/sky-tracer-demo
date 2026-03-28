---
tags: [infrastructure]
---

# Docker Compose

Single-command startup for the entire Sky Tracer stack.

## Start
```sh
docker compose up -d
```

## Services
- [[Traefik]] on `:8000` (web), `:8080` (dashboard)
- [[Airport Anywhere]] on `:3000`
- [[Flight Controller]] on `:3001`
- [[Orbital Beacon]] on `:3002`
- [[Tower of Babel]] on `:3003`
- [[Delay-O-Rama]] on `:3004`
- [[Cockpit]] on `:8081`
- [[Sky Nexus]] on `:8083`
- [[Jaeger]] on `:16686`

## Links
- [[Sky Tracer]]
