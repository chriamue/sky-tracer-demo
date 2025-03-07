# sky-tracer-demo

Demo for opentelemetry yew ratatui axum

## 🚀 Features

- ✈️ **Air Traffic Simulation**: Realistic simulation of communication between airplanes, control tower, and satellite systems
- 📊 **OpenTelemetry Integration**: Complete instrumentation of all services
- 🔍 **Jaeger Tracing UI**: Visual exploration of distributed traces
- 🌐 **Yew Web Frontend**: Modern, responsive dashboard using WebAssembly
- 💻 **Ratatui Terminal UI**: Real-time air traffic visualization in your terminal
- 🔄 **Axum Web Servers**: High-performance async web services
- 🐳 **Docker Deployment**: Complete containerization of all components

## Services

- **Satellite Positioning System (SPS)**: Simulates GPS/GNSS satellite signals
- **Airplane Cockpit System (ACS)**: Processes positioning data and communicates with tower
- **Tower Control System (TCS)**: Coordinates air traffic and distributes information
- **Airport Information System (AIS)**: Displays real-time flight information (Ratatui UI)
- **Airline Clients Portal (ACP)**: Web dashboard for airlines (Yew/WebAssembly)
- **Traefik Proxy**: Routes traffic between services
- **Jaeger**: Collects and visualizes distributed traces

## 🚀 Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.75+ recommended)
- [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (for Yew frontend)

### Local Development

Clone the repository:

```sh
git clone https://github.com/chriamue/sky-tracer-demo.git
cd sky-tracer-demo
```

## 📊 Viewing Traces

Once the system is running:

1. Open Jaeger UI at [http://localhost:16686](http://localhost:16686)
2. View the Airline Portal at [http://localhost:8080](http://localhost:8080)
3. The terminal UI runs directly in your console

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
