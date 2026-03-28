---
tags: [feature, mcp]
---

# AI Integration

Expose aviation data to AI assistants via [[MCP]].

## Server
- [[Sky Nexus]] — MCP server built with `rmcp`

## Supported Clients
- LM Studio (tools only — resources/prompts not yet supported by client)
- Claude Desktop
- VS Code GitHub Copilot
- Postman
- MCP Inspector (`npx @modelcontextprotocol/inspector --transport http --server-url http://localhost:8000/mcp`)

## MCP Primitives

### Tools
- Airport search and lookup
- Flight management and search
- Satellite positioning
- Timezone / datetime utilities
- Aggregated flight data (Tower of Babel)
- SVG flight map generation (`generate_flight_map`)

### Resources
- `airports://{code}` — live airport data by IATA code (e.g. `airports://FRA`)
- 8 featured airports listed as static resources

### Prompts
- `airport-briefing` — detailed airport briefing with live data
- `flight-route-analysis` — route analysis between two airports
- `delay-investigation` — delay status and root cause analysis
- `aviation-network-overview` — full capability overview (no arguments)

## Setup
Start [[Docker Compose]], then point your AI client to:
`http://localhost:8000/mcp`

## Links
- [[MCP]]
- [[Sky Tracer]]
