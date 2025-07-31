# Sky Nexus

Sky Nexus is a Model Context Protocol (MCP) server that provides airport and flight information services. It acts as a central hub for aviation data, integrating with external services and exposing both REST API endpoints and MCP tools for AI assistants.

## Features

- **MCP Server**: Provides airport lookup tools via the Model Context Protocol
- **REST API**: Traditional HTTP endpoints for airport and flight data
- **OpenAPI Documentation**: Interactive Swagger UI for API exploration
- **External Service Integration**: Connects to Airport Anywhere and Tower of Babel services
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
                    │  └─────────────────────────┘│
                    │                             │
                    │  ┌─────────────────────────┐│
                    │  │     MCP Endpoints       ││
                    │  │      /mcp               ││
                    │  └─────────────────────────┘│
                    │                             │
                    │  ┌─────────────────────────┐│
                    │  │    Documentation        ││
                    │  │    /swagger-ui          ││
                    │  └─────────────────────────┘│
                    └─────────────┬───────────────┘
                                  │
              ┌───────────────────┼───────────────────┐
              │                   │                   │
    ┌─────────▼───────────┐      │      ┌───────────▼─────────────┐
    │  Airport Anywhere   │      │      │   Tower of Babel        │
    │  (Port 8001)        │      │      │   (Port 8002)           │
    │                     │      │      │                         │
    │  Airport Data       │      │      │  Flight Data            │
    └─────────────────────┘      │      └─────────────────────────┘
                                 │
                    ┌─────────────▼───────────────┐
                    │     External Services       │
                    │   (Future integrations)     │
                    └─────────────────────────────┘
```

## Quick Start

### Prerequisites

- Rust 1.70 or later
- Running Airport Anywhere service (port 8001)
- Running Tower of Babel service (port 8002)

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
cargo run
```

The server will start on `http://localhost:8080`.

## Configuration

### Environment Variables

| Variable | Description | Default Value |
|----------|-------------|---------------|
| `AIRPORT_SERVICE_URL` | URL for the Airport Anywhere service | `http://localhost:8001/api/v1/airports` |
| `FLIGHT_SERVICE_URL` | URL for the Tower of Babel service | `http://localhost:8002/api/v1/flights` |
| `RUST_LOG` | Logging level configuration | `info` |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | OpenTelemetry collector endpoint | Not set |

### Example Configuration

```bash
# Set custom service URLs
export AIRPORT_SERVICE_URL="https://api.airports.example.com/v1/airports"
export FLIGHT_SERVICE_URL="https://api.flights.example.com/v1/flights"

# Enable debug logging
export RUST_LOG="debug,sky_nexus=trace"

# Configure OpenTelemetry
export OTEL_EXPORTER_OTLP_ENDPOINT="http://localhost:4317"

# Run the server
cargo run
```

## API Documentation

Once the server is running, you can access:

- **Swagger UI**: http://localhost:8080/swagger-ui
- **OpenAPI Spec**: http://localhost:8080/api-docs/openapi.json

### REST Endpoints

#### Airports
- `GET /api/v1/airports` - List all airports
- `GET /api/v1/airports/{code}` - Get airport by code

#### Flights
- `GET /api/v1/flights` - List all flights
- `POST /api/v1/flights` - Create a new flight
- `GET /api/v1/flights/{flight_number}` - Get flight by number

## MCP Integration

Sky Nexus provides MCP tools for AI assistants to interact with aviation data.

### Available MCP Tools

1. **list_airports**: List all available airports
2. **get_airport**: Get detailed information about a specific airport by code

### Using with Claude Desktop

Add this configuration to your Claude Desktop config file:

```json
{
  "mcpServers": {
    "sky-nexus": {
      "url": "http://127.0.0.1:8083/mcp"
    }
  }
}
```

### Using with MCP Inspector

You can test the MCP functionality using the MCP Inspector:

```bash
npx @modelcontextprotocol/inspector
```

Then connect to `http://localhost:8080/mcp`.

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
│       └── airports.rs # Airport-related MCP tools
├── openapi.rs          # OpenAPI documentation setup
├── routes/             # HTTP route handlers
│   ├── mod.rs          # Routes module
│   ├── mcp.rs          # MCP endpoint handlers
│   └── v1/             # API v1 routes
│       ├── mod.rs      # V1 routes module
│       ├── airports.rs # Airport endpoints
│       └── flights.rs  # Flight endpoints
└── services/           # External service clients
    ├── mod.rs          # Services module
    ├── airports.rs     # Airport service client
    └── flights.rs      # Flight service client
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

## Dependencies

### Core Dependencies
- **axum**: Web framework for HTTP API
- **tokio**: Async runtime
- **rmcp**: Model Context Protocol implementation
- **reqwest**: HTTP client for external services
- **serde**: Serialization/deserialization
- **utoipa**: OpenAPI documentation generation

### Observability
- **tracing**: Structured logging
- **init-tracing-opentelemetry**: OpenTelemetry integration
