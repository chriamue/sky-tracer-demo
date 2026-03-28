---
tags: [feature, visualization]
---

# Flight Map

SVG world map showing airports and active flight routes, generated server-side.

## Crate
- `flight-map` — standalone library using Yew 0.23 SSR

## Components
| Component | Role |
|-----------|------|
| `FlightMapSvg` | Root — composes all sub-components |
| `Grid` | Lat/lon grid lines (equator highlighted) |
| `AirportPinEl` | Airport dot + IATA code label |
| `RouteArcEl` | Bezier arc between departure and arrival |
| `PositionMarker` | Yellow dot for current in-flight position |
| `Title` | Optional centred title |
| `Legend` | Airport/in-flight key + stats |

## Projection
Equirectangular — longitude → x, latitude → y (1000 × 500 px).

## MCP Tool
`generate_flight_map` in [[Sky Nexus]]:
- No args → full network map
- `airport_code` → routes for a single airport
- Returns raw SVG markup

## Links
- [[AI Integration]]
- [[Sky Tracer]]
