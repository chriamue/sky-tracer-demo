---
tags: [service, mcp, backend]
port: 8083
tech: [rust, axum, rmcp, yew]
---

# Sky Nexus

[[MCP]] server — exposes aviation data to AI assistants.

## Responsibilities
- Single MCP endpoint aggregating all aviation data
- REST API with OpenAPI docs
- Integrates with [[Airport Anywhere]], [[Flight Controller]], [[Orbital Beacon]], [[Tower of Babel]]

## MCP Endpoint
`/mcp` — single streamable-HTTP endpoint (MCP 2025-03-26 spec)

## Tools
| Tool | Description |
|------|-------------|
| `list_airports` | List all airports |
| `get_airport` | Lookup airport by IATA code |
| `list_flights` | List all flights |
| `get_flight` | Get flight by number |
| `create_flight` | Create a new flight |
| `search_flights_by_route` | Search by departure/arrival |
| `list_satellites` | List tracked satellites |
| `create_satellite` | Register a satellite |
| `update_satellite_status` | Update satellite status |
| `calculate_position` | Calculate satellite position |
| `get_current_datetime` | Current UTC time |
| `get_aviation_times` | Airport local times |
| `get_timezone_difference` | Difference between two timezones |
| `compare_timezones` | Compare multiple timezones |
| `get_flights_by_airport` | Flights at an airport (via Tower of Babel) |
| `get_flight_position` | Live flight position |
| `search_flights_by_airport_pattern` | Search by airport code pattern |
| `generate_flight_map` | Generate SVG world map with routes |

## Resources
- `airports://{code}` — live airport JSON by IATA code
- 8 featured airports as static resources (FRA, LHR, CDG, JFK, DXB, SIN, AMS, HND)

## Prompts
- `airport-briefing` — structured Markdown briefing for an airport
- `flight-route-analysis` — route analysis between two airports
- `delay-investigation` — delay status and root cause analysis
- `aviation-network-overview` — full server capability overview

## Dependencies
- `rmcp` — MCP server framework
- `flight-map` — Yew SSR SVG map generation

## Features
- [[AI Integration]]
- [[Distributed Tracing]]

## Links
- [[Sky Tracer]]
