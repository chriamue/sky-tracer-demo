---
tags: [concept]
---

# Language Server Protocol (LSP)

Standard protocol for language tooling across editors. Inspiration for [[MCP]].

## Idea
One language server implementation works with all editors (VS Code, Vim, Neovim, etc.) via a shared protocol.

## Parallel to MCP
| LSP | MCP |
|-----|-----|
| Editor | AI Host |
| Language Server | MCP Server |
| Code completion | Tool call |
| Diagnostics | Resource |

## Links
- [[MCP]]
- [[Sky Tracer]]
