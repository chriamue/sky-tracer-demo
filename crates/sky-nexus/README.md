# Sky Nexus

Sky Nexus is a Model Context Protocol (MCP) server that provides comprehensive aviation data services. It acts as a central hub for aviation data, integrating with multiple microservices and exposing both REST API endpoints and specialized MCP tools for AI assistants.

## Features

- **Multi-Service MCP Server**: Provides separate MCP endpoints for airports, flights, satellites, and datetime utilities
- **REST API**: Traditional HTTP endpoints for all aviation data
- **OpenAPI Documentation**: Interactive Swagger UI for API exploration
- **Microservice Integration**: Connects to Airport Anywhere, Flight Controller, and Orbital Beacon services
- **DateTime Service**: Comprehensive timezone and datetime utilities for aviation operations
- **Observability**: Built-in tracing and OpenTelemetry support

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   AI Assistant  │    │   Web Client    │    │   MCP Client    │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          │                      │                      │
          │ HTTP REST API        │                      │ MCP Protocol
          │                      │                      │
          └──────────────────────┼──────────────────────┘
                                 │
                    ┌─────────────▼───────────────┐
                    │        Sky Nexus            │
                    │                             │
                    │  ┌─────────────────────────┐│
                    │  │     REST Endpoints      ││
                    │  │   /api/v1/airports      ││
                    │  │   /api/v1/flights       ││
                    │  │   /api/v1/satellites    ││
                    │  │   /api/v1/datetime      ││
                    │  └─────────────────────────┘│
                    │                             │
                    │  ┌─────────────────────────┐│
                    │  │     MCP Endpoints       ││
                    │  │   /mcp/airports         ││
                    │  │   /mcp/flights          ││
                    │  │   /mcp/satellites       ││
                    │  │   /mcp/datetime         ││
                    │  └─────────────────────────┘│
                    │                             │
                    │  ┌─────────────────────────┐│
                    │  │    Documentation        ││
                    │  │    /swagger-ui          ││
                    │  └─────────────────────────┘│
                    └─────────────┬───────────────┘
                                  │
          ┌───────────────────────┼───────────────────────┐
          │                       │                       │
┌─────────▼───────────┐  ┌────────▼────────┐  ┌──────────▼──────────┐
│  Airport Anywhere   │  │ Flight Controller│  │  Orbital Beacon     │
│  (Port 3000)        │  │ (Port 3001)     │  │  (Port 3002)        │
│                     │  │                 │  │                     │
│  Airport Data       │  │  Flight Data    │  │  Satellite Data     │
└─────────────────────┘  └─────────────────┘  └─────────────────────┘
```

## Quick Start

### Prerequisites

- Rust 1.70 or later
- Running Airport Anywhere service (port 3000)
- Running Flight Controller service (port 3001)
- Running Orbital Beacon service (port 3002)

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd sky-tracer-demo/crates/sky-nexus
```

2. Build the project:
```bash
cargo build --release
```

3. Run the server:
```bash
just run
```

The server will start on the configured port (default: 9093).

## Configuration

### Environment Variables

| Variable | Description | Default Value |
|----------|-------------|---------------|
| `PORT` | Port for Sky Nexus server | `8080` |
| `AIRPORT_SERVICE_URL` | URL for the Airport Anywhere service | `http://localhost:3000/api` |
| `FLIGHT_SERVICE_URL` | URL for the Flight Controller service | `http://localhost:3001/api` |
| `SATELLITE_SERVICE_URL` | URL for the Orbital Beacon service | `http://localhost:3002/api` |
| `RUST_LOG` | Logging level configuration | `info` |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | OpenTelemetry collector endpoint | Not set |

### Example Configuration

```bash
# Set custom service URLs
export AIRPORT_SERVICE_URL="http://localhost:3000/api"
export FLIGHT_SERVICE_URL="http://localhost:3001/api"
export SATELLITE_SERVICE_URL="http://localhost:3002/api"

# Enable debug logging
export RUST_LOG="debug,sky_nexus=trace"

# Configure OpenTelemetry
export OTEL_EXPORTER_OTLP_ENDPOINT="http://localhost:4317"

# Run the server
just run
```

## API Documentation

Once the server is running, you can access:

- **Swagger UI**: http://localhost:9093/swagger-ui
- **OpenAPI Spec**: http://localhost:9093/api-docs/openapi.json

### REST Endpoints

#### Airports
- `GET /api/v1/nexus/airports` - List all airports
- `GET /api/v1/nexus/airports/{code}` - Get airport by code

#### Flights
- `GET /api/v1/nexus/flights` - List all flights
- `POST /api/v1/nexus/flights` - Create a new flight
- `GET /api/v1/nexus/flights/{flight_number}` - Get flight by number

#### Satellites
- `GET /api/v1/nexus/satellites` - List all satellites
- `POST /api/v1/nexus/satellites` - Create a new satellite
- `PUT /api/v1/nexus/satellites/{id}/status` - Update satellite status
- `POST /api/v1/nexus/satellites/position` - Calculate flight position

#### DateTime
- `GET /api/v1/nexus/datetime/current` - Get current date and time
- `GET /api/v1/nexus/datetime/aviation-times` - Get time in major aviation hubs
- `POST /api/v1/nexus/datetime/compare` - Compare two timezones

## DateTime Service

The DateTime service provides comprehensive timezone and time utilities specifically designed for aviation operations.

### REST API Examples

#### Get Current Time
```bash
# Get current UTC time
curl "http://localhost:9093/api/v1/nexus/datetime/current"

# Get current time in specific timezone
curl "http://localhost:9093/api/v1/nexus/datetime/current?timezone=America/New_York&format=%Y-%m-%d %H:%M:%S %Z"

# Get current time in aviation format
curl "http://localhost:9093/api/v1/nexus/datetime/current?timezone=UTC&format=%d%H%MZ"
```

#### Get Aviation Hub Times
```bash
# Get current time in all major aviation hubs
curl "http://localhost:9093/api/v1/nexus/datetime/aviation-times"
```

#### Compare Timezones
```bash
# Compare two timezones
curl -X POST "http://localhost:9093/api/v1/nexus/datetime/compare" \
  -H "Content-Type: application/json" \
  -d '{
    "from_timezone": "America/New_York",
    "to_timezone": "Europe/London"
  }'
```

### Supported Timezones

The service supports all IANA timezone identifiers. Common aviation timezones include:

- **UTC**: Coordinated Universal Time
- **America/New_York**: Eastern Time (JFK, LGA, EWR)
- **America/Los_Angeles**: Pacific Time (LAX)
- **America/Chicago**: Central Time (ORD, MDW)
- **America/Denver**: Mountain Time (DEN)
- **Europe/London**: GMT/BST (LHR, LGW)
- **Europe/Paris**: CET/CEST (CDG, ORY)
- **Europe/Berlin**: CET/CEST (FRA)
- **Asia/Tokyo**: JST (NRT, HND)
- **Australia/Sydney**: AEST/AEDT (SYD)
- **Asia/Dubai**: GST (DXB)
- **Asia/Singapore**: SGT (SIN)

## MCP Integration

Sky Nexus provides separate MCP endpoints for different aviation data categories, allowing AI assistants to connect to specific tool sets.

### Available MCP Endpoints

- **Airport Tools**: `/mcp/airports`
  - `list_airports`: List all available airports
  - `get_airport`: Get detailed information about a specific airport by code

- **Flight Tools**: `/mcp/flights`
  - `list_flights`: List all flights with optional filters
  - `get_flight`: Get detailed information about a specific flight
  - `create_flight`: Create a new flight
  - `search_flights_by_route`: Search flights by departure and arrival

- **Satellite Tools**: `/mcp/satellites`
  - `list_satellites`: List all satellites
  - `create_satellite`: Create a new satellite
  - `update_satellite_status`: Update satellite status
  - `calculate_position`: Calculate flight position using satellites

- **DateTime Tools**: `/mcp/datetime`
  - `get_current_datetime`: Get current date and time with optional timezone and formatting
  - `get_aviation_times`: Get current time in major aviation hubs around the world
  - `get_timezone_difference`: Calculate time difference between two timezones

### Using with Claude Desktop

Add these configurations to your Claude Desktop config file:

```json
{
  "mcpServers": {
    "sky-nexus-airports": {
      "url": "http://127.0.0.1:9093/mcp/airports"
    },
    "sky-nexus-flights": {
      "url": "http://127.0.0.1:9093/mcp/flights"
    },
    "sky-nexus-satellites": {
      "url": "http://127.0.0.1:9093/mcp/satellites"
    },
    "sky-nexus-datetime": {
      "url": "http://127.0.0.1:9093/mcp/datetime"
    }
  }
}
```

### Using with MCP Inspector

You can test each MCP service separately:

```bash
# Test airports
npx @modelcontextprotocol/inspector http://localhost:9093/mcp/airports

# Test flights
npx @modelcontextprotocol/inspector http://localhost:9093/mcp/flights

# Test satellites
npx @modelcontextprotocol/inspector http://localhost:9093/mcp/satellites

# Test datetime tools
npx @modelcontextprotocol/inspector http://localhost:9093/mcp/datetime
```

## DateTime Tools Usage Examples

The datetime tools are particularly useful for aviation operations that span multiple time zones:

### MCP Tool Examples

#### Get Current Time
```json
{
  "method": "tools/call",
  "params": {
    "name": "get_current_datetime",
    "arguments": {
      "timezone": "America/New_York",
      "format": "%Y-%m-%d %H:%M:%S %Z"
    }
  }
}
```

#### Aviation Hub Times
```json
{
  "method": "tools/call",
  "params": {
    "name": "get_aviation_times",
    "arguments": {}
  }
}
```

#### Compare Timezones
```json
{
  "method": "tools/call",
  "params": {
    "name": "get_timezone_difference",
    "arguments": {
      "from_timezone": "America/New_York",
      "to_timezone": "Europe/London"
    }
  }
}
```

### Use Cases

- **Flight Planning**: Coordinate departures and arrivals across time zones
- **Crew Scheduling**: Manage crew schedules across different airports
- **Operations Coordination**: Real-time coordination of aviation operations
- **Passenger Information**: Display accurate local times for connections
- **Cargo Operations**: Schedule cargo handling across time zones
- **Air Traffic Control**: Coordinate with international ATC centers

## Development

### Project Structure

```
src/
├── lib.rs              # Library entry point
├── main.rs             # Application entry point
├── mcp/                # MCP server implementation
│   ├── mod.rs          # MCP module exports
│   └── tools/          # MCP tools
│       ├── mod.rs      # Tools module exports
│       ├── airports.rs # Airport-related MCP tools
│       ├── datetime.rs # DateTime utility MCP tools
│       ├── flights.rs  # Flight-related MCP tools
│       └── satellites.rs # Satellite-related MCP tools
├── openapi.rs          # OpenAPI documentation setup
├── routes/             # HTTP route handlers
│   ├── mod.rs          # Routes module
│   └── v1/             # API v1 routes
│       ├── mod.rs      # V1 routes module
│       ├── airports.rs # Airport endpoints
│       ├── datetime.rs # DateTime endpoints
│       ├── flights.rs  # Flight endpoints
│       └── satellites.rs # Satellite endpoints
└── services/           # Service logic
    ├── mod.rs          # Services module
    ├── airports.rs     # Airport service client
    ├── datetime.rs     # DateTime service logic
    ├── flights.rs      # Flight service client
    └── satellites.rs   # Satellite service client
```

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

### Using Justfile

Sky Nexus includes a `justfile` for easy development:

```bash
# List available commands
just

# Run the server with environment variables
just run

# Run with custom port
PORT=8080 just run
```

## Dependencies

### Core Dependencies
- **axum**: Web framework for HTTP API
- **tokio**: Async runtime
- **rmcp**: Model Context Protocol implementation
- **reqwest**: HTTP client for external services
- **serde**: Serialization/deserialization
- **utoipa**: OpenAPI documentation generation
- **chrono**: Date and time handling
- **chrono-tz**: Timezone support

### Observability
- **tracing**: Structured logging
- **init-tracing-opentelemetry**: OpenTelemetry integration

## Deployment

Sky Nexus can be deployed using Docker:

```bash
# Build the container
docker build -t sky-nexus .

# Run with environment variables
docker run -p 9093:8080 \
  -e AIRPORT_SERVICE_URL=http://host.docker.internal:3000/api \
  -e FLIGHT_SERVICE_URL=http://host.docker.internal:3001/api \
  -e SATELLITE_SERVICE_URL=http://host.docker.internal:3002/api \
  sky-nexus
```

Or using Docker Compose with the full sky-tracer-demo stack:

```bash
cd sky-tracer-demo
docker-compose up sky-nexus
```
