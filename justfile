# Load required environment variables
set dotenv-load

# Default recipe to show available commands
default:
    @just --list

# Build and start all services
up:
    docker compose up -d
    @echo "Services are starting..."
    @echo "Access the applications at:"
    @echo "- Landing Page:        http://localhost:8000"
    @echo "- Cockpit Dashboard:   http://localhost:8000/cockpit"
    @echo "- Delay-O-Rama:        http://localhost:8000/delays"
    @echo "- Flightmare:          http://localhost:8000/flightmare"
    @echo "- Airport Anywhere:    http://localhost:8000/airports"
    @echo "- Flight Controller:   http://localhost:8000/flights"
    @echo "- Orbital Beacon:      http://localhost:8000/satellites"
    @echo "- Tower of Babel:      http://localhost:8000/babel"
    @echo "- Sky Nexus (MCP):     http://localhost:8000/mcp"
    @echo "- Sky Nexus API Docs:  http://localhost:8000/nexus/docs"
    @echo "- Traefik Dashboard:   http://localhost:8080"
    @echo "- Jaeger Tracing:      http://localhost:16686"
    @echo "- Structurizr:         http://localhost:8082"

# Stop all services
down:
    docker compose down

# View logs of all services
logs:
    docker compose logs -f

# Open the Cockpit dashboard in default browser
open:
    #!/usr/bin/env sh
    if command -v xdg-open > /dev/null; then
        xdg-open http://localhost:8000
    elif command -v open > /dev/null; then
        open http://localhost:8000
    else
        echo "Could not detect the web browser opener"
    fi

# Start services and open dashboard
start: up open

# Clean up everything including volumes
clean:
    docker compose down -v

# Run all tests
test:
    cargo test --workspace

# Check code formatting
fmt:
    cargo fmt --all -- --check

# Run clippy lints
lint:
    cargo clippy --workspace -- -D warnings

# Start Structurizr Lite
structurizr:
    docker compose up -d structurizr
    @echo "Structurizr is starting at http://localhost:8082"
    #!/usr/bin/env sh
    if command -v xdg-open > /dev/null; then xdg-open http://localhost:8082; \
    elif command -v open > /dev/null; then open http://localhost:8082; \
    else echo "Could not detect the web browser opener"; fi

# Stop Structurizr
structurizr-down:
    docker compose down structurizr

# Start Sky Nexus MCP server (and its dependencies)
nexus:
    docker compose up -d sky-nexus
    @echo "Sky Nexus is starting..."
    @echo "- MCP endpoint:  http://localhost:8000/mcp"
    @echo "- API docs:      http://localhost:8000/nexus/docs"

# Stop Sky Nexus
nexus-down:
    docker compose down sky-nexus

# View Sky Nexus logs
nexus-logs:
    docker compose logs -f sky-nexus

# Test Sky Nexus MCP server with MCP Inspector
mcp-inspect:
    npx @modelcontextprotocol/inspector http://localhost:8000/mcp
