---
tags: [concept, ai]
---

# Model Context Protocol (MCP)

Open protocol for connecting LLMs to external data and tools. Inspired by [[LSP]].

## Roles
- **Host** — LLM app (LM Studio, Claude Desktop)
- **Client** — connector within the host
- **Server** — service exposing tools/resources

## Transport
JSON-RPC 2.0 over HTTP/SSE or stdio

## Lifecycle
1. `initialize` — capability negotiation
2. `tools/list` — discover available tools
3. `tools/call` — invoke a tool
4. close connection

## In Sky Tracer
- [[Sky Nexus]] is the MCP server
- [[AI Integration]] describes setup

## Links
- [[LSP]]
- [[Sky Tracer]]
