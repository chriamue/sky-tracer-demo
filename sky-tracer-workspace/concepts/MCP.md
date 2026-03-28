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
JSON-RPC 2.0 over streamable HTTP, SSE, or stdio

## Primitives
| Primitive | Description |
|-----------|-------------|
| **Tools** | Executable functions the LLM can invoke |
| **Resources** | Server-exposed data addressable by URI |
| **Prompts** | Pre-defined prompt templates with arguments |
| **Instructions** | Server-provided guidance included at startup |

## Lifecycle
1. `initialize` — capability negotiation (tools / resources / prompts)
2. `tools/list` → `tools/call` — tool discovery and invocation
3. `resources/list` → `resources/read` — URI-based data access
4. `prompts/list` → `prompts/get` — prompt template retrieval
5. close connection

## In Sky Tracer
- [[Sky Nexus]] is the MCP server
- [[AI Integration]] describes setup

## Links
- [[LSP]]
- [[Sky Tracer]]
