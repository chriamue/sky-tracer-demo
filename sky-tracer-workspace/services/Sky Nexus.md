---
tags: [service, mcp, backend]
port: 8083
tech: [rust, axum, rmcp]
---

# Sky Nexus

[[MCP]] server — exposes aviation data to AI assistants.

## Responsibilities
- MCP endpoints for all aviation data
- REST API with OpenAPI docs
- Integrates with [[Airport Anywhere]], [[Flight Controller]], [[Orbital Beacon]], [[Tower of Babel]]

## MCP Endpoints
- `/mcp/airports` — [[Airport Lookup]] tools
- `/mcp/flights` — [[Flight Tracking]] tools
- `/mcp/satellites` — [[Satellite Positioning]] tools
- `/mcp/datetime` — timezone utilities
- `/mcp/babel` — aggregated flight tools

## Features
- [[AI Integration]]
- [[Distributed Tracing]]

## Links
- [[Sky Tracer]]
