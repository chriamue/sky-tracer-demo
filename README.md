# sky-tracer-demo

Demo for Rust web services with Yew, Axum, Traefik, and MCP

[📽️ View Presentation](assets/index/presentation/index.html)

## 🚀 Features

- ✈️ **Airport Information**: Complete airport database with search functionality
- 🛩️ **Flight Management**: Track and manage flights between airports
- 🛰️ **Satellite Positioning**: Real-time flight position calculation
- ⏰ **Delay Tracking**: Monitor flight delays in real-time
- 🌐 **Web Frontends**: Modern web interfaces built with Yew/WebAssembly
- 🔄 **Axum Web Services**: High-performance async web services
- 🚦 **Traefik Integration**: Smart request routing and load balancing
- 🎯 **C4 Architecture**: Visualized system architecture using Structurizr
- 🐳 **Docker Compose**: Complete containerization of all components
- 🤖 **MCP Server**: AI assistant integration via Model Context Protocol

## 🌐 Service Access

### Main Entry Point
- 📍 **Landing Page**: [http://localhost:8000](http://localhost:8000)
- 🎭 **Presentation**: [http://localhost:8000/presentation/](http://localhost:8000/presentation/)

### User Interfaces
- 🎯 **Cockpit Dashboard**: [http://localhost:8000/cockpit/](http://localhost:8000/cockpit/) (Flight Staff)
- ⏰ **Delay-O-Rama**: [http://localhost:8000/delays/](http://localhost:8000/delays/) (Travelers)
- 😱 **Flightmare**: [http://localhost:8000/flightmare/](http://localhost:8000/flightmare/) (Travelers)

### Core Services
- 🏢 **Airport Anywhere**: [http://localhost:8000/airports](http://localhost:8000/airports)
- 🎮 **Flight Controller**: [http://localhost:8000/flights](http://localhost:8000/flights)
- 🛰️ **Orbital Beacon**: [http://localhost:8000/satellites](http://localhost:8000/satellites)
- 🗼 **Tower of Babel**: [http://localhost:8000/babel](http://localhost:8000/babel)
- 🤖 **Sky Nexus (MCP)**: [http://localhost:8000/mcp](http://localhost:8000/mcp)

### API Documentation
- 📚 **API Docs**: [http://localhost:8000/flights/api/docs](http://localhost:8000/flights/api/docs)
- 📚 **Sky Nexus API Docs**: [http://localhost:8000/nexus/docs](http://localhost:8000/nexus/docs)

### Infrastructure & Monitoring
- 🔄 **Traefik Dashboard**: [http://localhost:8080](http://localhost:8080)
- 📊 **Jaeger Tracing**: [http://localhost:16686](http://localhost:16686)
- 🏗️ **Architecture Docs**: [http://localhost:8082](http://localhost:8082)

## User Roles

- ✈️ **Flight Staff**: Access to Cockpit and Airport Anywhere
- 🧳 **Travelers**: Access to Delay-O-Rama and Flightmare Tracker
- 🛸 **Satellite Operators**: Access to Orbital Beacon
- 🤖 **AI Assistants**: Access to Sky Nexus MCP server

## 🚀 Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.75+ recommended)
- [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/)
- [just](https://github.com/casey/just) command runner

### Local Development

Clone and start the services:

```sh
# Clone the repository
git clone https://github.com/chriamue/sky-tracer-demo.git
cd sky-tracer-demo

# Start all services
just start

# View architecture documentation
just structurizr
```

## 🌐 Service URLs

All services are available through Traefik at http://localhost:8000:

- `/airports` - Airport information lookup
- `/flights` - Flight management
- `/satellites` - Satellite positioning
- `/cockpit` - Flight monitoring dashboard
- `/flightmare` - Flight delay simulation
- `/delays` - Real-time delay monitoring
- `/babel` - Flight aggregation API
- `/mcp` - MCP server for AI assistants

Additional endpoints:
- Traefik Dashboard: http://localhost:8080
- Structurizr Documentation: http://localhost:8082
- Jaeger Tracing: http://localhost:16686

## 🤖 MCP Integration

Sky Nexus exposes all aviation data as a single [Model Context Protocol](https://modelcontextprotocol.io) server, allowing AI assistants to query airports, flights, satellites, and more.

### Available Tools

| Category | Tools |
|----------|-------|
| Airports | `list_airports`, `get_airport` |
| Flights | `list_flights`, `get_flight`, `create_flight`, `search_flights_by_route` |
| Satellites | `list_satellites`, `create_satellite`, `update_satellite_status`, `calculate_position` |
| DateTime | `get_current_datetime`, `get_aviation_times`, `get_timezone_difference`, `compare_timezones` |
| Tracking | `get_flights_by_airport`, `get_flight_position` |

### Configure LM Studio

Start the stack, then add to your `mcp.json` in LM Studio (Program tab → Install → Edit mcp.json):

```json
{
  "mcpServers": {
    "sky-nexus": {
      "url": "http://localhost:8000/mcp"
    }
  }
}
```

### Configure Claude Desktop

Add to your Claude Desktop config file:

```json
{
  "mcpServers": {
    "sky-nexus": {
      "url": "http://localhost:8000/mcp"
    }
  }
}
```

### Test with MCP Inspector

```sh
npx @modelcontextprotocol/inspector http://localhost:8000/mcp
```

## 📝 Available Commands

```sh
# Start all services
just start

# View service logs
just logs

# Stop all services
just down

# View architecture documentation
just structurizr

# Stop Structurizr
just structurizr-down

# Run tests
just test

# Check code formatting
just fmt

# Run linter
just lint
```

## 🗂️ Project Structure

```
sky-tracer-demo/
├── assets/                 # Shared assets and data files
├── crates/                # Rust crates
│   ├── airport-anywhere/  # Airport lookup service
│   ├── cockpit/          # Staff dashboard
│   ├── delay-orama/      # Delay monitoring
│   ├── flight-controller/ # Flight management
│   ├── flightmare-tracker/# Delay simulation
│   ├── orbital-beacon/    # Satellite positioning
│   ├── sky-nexus/        # MCP server (AI integration)
│   ├── sky-tracer/       # Shared library
│   └── tower-of-babel/   # Flight aggregation
└── docs/                  # Architecture documentation
```

## 📊 Data Sources

### airports.dat

The file `airports.dat` is sourced from the OpenFlights database, providing comprehensive airport data including locations, codes, and other essential information.

For more details and the most up-to-date data, visit [OpenFlights on GitHub](https://github.com/jpatokal/openflights).

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
