---
tags: [feature, mcp]
---

# AI Integration

Expose aviation data to AI assistants via [[MCP]].

## Server
- [[Sky Nexus]] — MCP server built with `rmcp`

## Supported Clients
- LM Studio (configured via `mcp.json`)
- Claude Desktop

## Available Tools
- Airport search and lookup
- Flight management and search
- Satellite positioning
- Timezone / datetime utilities

## Setup
Start [[Docker Compose]], then point your AI client to:
`http://localhost:8000/mcp/{airports,flights,satellites,datetime,babel}`

## Links
- [[MCP]]
- [[Sky Tracer]]
