# sky-tracer-demo

Demo for Rust web services with Yew, Axum, and Traefik

## 🚀 Features

- ✈️ **Airport Information**: Complete airport database with search functionality
- 🛩️ **Flight Management**: Track and manage flights between airports
- 🛰️ **Satellite Positioning**: Real-time flight position calculation
- 🌐 **Web Frontends**: Modern web interfaces built with Yew/WebAssembly
- 🔄 **Axum Web Services**: High-performance async web services
- 🚦 **Traefik Integration**: Smart request routing and load balancing
- 🎯 **C4 Architecture**: Visualized system architecture using Structurizr
- 🐳 **Docker Deployment**: Complete containerization of all components

## Services

- **Airport Anywhere**: Airport information lookup service
- **Flight Controller**: Flight management and tracking
- **Orbital Beacon**: Satellite positioning system
- **Cockpit**: Flight monitoring dashboard
- **Flightmare Tracker**: Flight delay monitoring
- **Traefik**: Routes traffic between services
- **Structurizr**: Architecture visualization

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
- `/flightmare` - Flight delay monitoring

Additional endpoints:
- Traefik Dashboard: http://localhost:8080
- Structurizr Documentation: http://localhost:8082

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

## Assets Directory

This directory contains various data files and assets used by the application.

### airports.dat

The file `airports.dat` is sourced from the OpenFlights database, providing comprehensive airport data including locations, codes, and other essential information.

For more details and the most up-to-date data, visit [OpenFlights on GitHub](https://github.com/jpatokal/openflights).

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
