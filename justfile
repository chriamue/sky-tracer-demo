# Load required environment variables
set dotenv-load

# Default recipe to show available commands
default:
    @just --list

# Build and start all services
up:
    docker compose up --build -d
    @echo "Services are starting..."
    @echo "Access the applications at:"
    @echo "- Cockpit Dashboard: http://localhost:8080"
    @echo "- Airport Anywhere: http://localhost:3000"
    @echo "- Flight Controller: http://localhost:3001"
    @echo "- Orbital Beacon: http://localhost:3002"
    @echo "- Flightmare Tracker: http://localhost:3003"
    @echo "- Structurizr: http://localhost:8081"

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
