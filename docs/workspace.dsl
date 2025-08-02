workspace {
    model {
        flightStaff = person "✈️ Flight Staff" "Airline and airport personnel monitoring flights" {
            tags "staff"
        }

        traveler = person "🧳 Traveler" "Passenger checking flight status and delays" {
            tags "traveler"
        }

        satelliteOperator = person "🛸 Satellite Operator" "Orbital Beacon staff managing satellites" {
            tags "operator"
        }

        aiAssistant = person "🤖 AI Assistant" "Claude, ChatGPT, and other AI systems using MCP tools" {
            tags "ai"
        }

        skyTracer = softwareSystem "🛩️ Sky Tracer System" "Comprehensive flight tracking and aviation data platform" {
            # Sky Nexus - Central Integration Hub
            skyNexus = container "🌟 Sky Nexus" "Central aviation data integration hub with MCP services" "Rust/Axum" {
                tags "integration"
                component "Airport MCP Tools" "MCP tools for airport data access" "Rust/RMCP"
                component "Flight MCP Tools" "MCP tools for flight management" "Rust/RMCP"
                component "Satellite MCP Tools" "MCP tools for satellite tracking" "Rust/RMCP"
                component "DateTime MCP Tools" "MCP tools for aviation time coordination" "Rust/RMCP"
                component "Babel MCP Tools" "MCP tools for real-time flight tracking" "Rust/RMCP"
                component "REST API Gateway" "Unified REST API for all services" "Rust/Axum"
                component "Service Aggregator" "Aggregates data from microservices" "Rust"
                component "OpenAPI Documentation" "Interactive API documentation" "Swagger UI"
            }

            # Airport Anywhere Container and Components
            airportAnywhere = container "🏢 Airport Anywhere" "Airport information lookup service" "Rust/Axum" {
                tags "core"
                component "Airport Service" "Manages airport data and search" "Rust"
                component "Airport API" "REST API for airport lookups" "Rust/Axum"
                component "Airport UI" "Airport search interface" "Rust/Yew SSR"
                component "Airport Database" "In-memory airport data store" "Rust/HashMap"
            }

            # Flight Controller Container and Components
            flightController = container "🎮 Flight Controller" "Flight management and tracking" "Rust/Axum" {
                tags "core"
                component "Flight Service" "Manages flight data and operations" "Rust"
                component "Flight API" "REST API for flight operations" "Rust/Axum"
                component "Flight UI" "Flight management interface" "Rust/Yew SSR"
                component "Position Calculator" "Calculates flight positions via satellites" "Rust"
                component "Flight Store" "In-memory flight data storage" "Rust/HashMap"
            }

            # Orbital Beacon Container and Components
            orbitalBeacon = container "🛰️ Orbital Beacon" "Satellite tracking and positioning system" "Rust/Axum" {
                tags "core"
                component "Satellite Service" "Manages satellites and positions" "Rust"
                component "Position API" "REST API for position calculations" "Rust/Axum"
                component "Satellite UI" "Satellite management interface" "Rust/Yew SSR"
                component "Launch Control" "Satellite deployment interface" "Rust/Yew SSR"
                component "Position Engine" "Calculates GPS positions using satellite data" "Rust"
            }

            # Tower of Babel Container and Components
            towerOfBabel = container "🗼 Tower of Babel" "Flight aggregation and real-time tracking service" "Rust/Axum" {
                tags "core"
                component "Flight Aggregator" "Aggregates flight information from controllers" "Rust"
                component "Position Tracker" "Tracks real-time flight positions" "Rust"
                component "Babel API" "REST API for flight tracking" "Rust/Axum"
                component "Future Flight Filter" "Filters flights for future departures" "Rust"
            }

            # Delay-O-Rama Container and Components
            delayORama = container "⏰ Delay-O-Rama" "Flight delay monitoring service" "Rust/Axum" {
                tags "frontend"
                component "Delay Monitor" "Monitors and tracks flight delays" "Rust"
                component "Delay Calculator" "Calculates delay patterns and statistics" "Rust"
                component "Delay UI" "Public delay visualization interface" "Rust/Yew SSR"
                component "Delay API" "REST API for delay information" "Rust/Axum"
            }

            # Web Frontends
            cockpit = container "🎯 Cockpit" "Staff flight monitoring dashboard" "Rust/Yew/WebAssembly" {
                tags "frontend"
                component "Flight Map" "Interactive flight tracking map" "Rust/Yew"
                component "Flight List" "Real-time flight status list" "Rust/Yew"
                component "Status Panel" "System status and alerts panel" "Rust/Yew"
                component "Control Interface" "Flight operation controls" "Rust/Yew"
            }

            flightmareTracker = container "😱 Flightmare Tracker" "Public delay simulation and tracking viewer" "Rust/Yew/WebAssembly" {
                tags "frontend"
                component "Delay Simulator" "Simulates and displays flight delays with excuses" "Rust/Yew"
                component "Flight Tracker" "Real-time flight tracking display" "Rust/Yew"
                component "Excuse Generator" "Generates humorous delay explanations" "Rust"
            }

            # Infrastructure
            traefik = container "🔄 Traefik" "Reverse proxy and load balancer" "Traefik" {
                tags "infrastructure"
            }

            jaeger = container "📊 Jaeger" "Distributed tracing system" "Jaeger" {
                tags "infrastructure"
            }

            # User Relationships
            flightStaff -> cockpit "Monitors flights and operations using"
            flightStaff -> skyNexus "Accesses unified aviation data via"

            traveler -> delayORama "Checks real-time delays using"
            traveler -> flightmareTracker "Views delay patterns and tracking using"

            satelliteOperator -> orbitalBeacon "Manages and launches satellites using"

            aiAssistant -> skyNexus "Accesses aviation data via MCP tools from"

            # Core Service Relationships
            skyNexus -> airportAnywhere "Fetches airport data from"
            skyNexus -> flightController "Fetches flight data from"
            skyNexus -> orbitalBeacon "Fetches satellite data from"
            skyNexus -> towerOfBabel "Fetches real-time tracking data from"

            # Service to Service Relationships
            towerOfBabel -> flightController "Fetches flight schedules from"
            towerOfBabel -> orbitalBeacon "Gets real-time position data from"

            flightController -> airportAnywhere "Looks up airport information from"
            flightController -> orbitalBeacon "Calculates flight positions using"

            orbitalBeacon -> airportAnywhere "Gets airport coordinates from"

            delayORama -> towerOfBabel "Gets flight status and delays from"

            # Frontend to Backend Relationships
            cockpit -> skyNexus "Gets comprehensive flight data from"
            cockpit -> flightController "Gets direct flight updates from"
            cockpit -> orbitalBeacon "Gets position updates from"

            flightmareTracker -> flightController "Gets flight data for delay simulation from"

            # Infrastructure Relationships
            traefik -> skyNexus "Routes requests to"
            traefik -> airportAnywhere "Routes requests to"
            traefik -> flightController "Routes requests to"
            traefik -> orbitalBeacon "Routes requests to"
            traefik -> towerOfBabel "Routes requests to"
            traefik -> delayORama "Routes requests to"
            traefik -> cockpit "Routes requests to"
            traefik -> flightmareTracker "Routes requests to"

            skyNexus -> jaeger "Sends traces to"
            airportAnywhere -> jaeger "Sends traces to"
            flightController -> jaeger "Sends traces to"
            orbitalBeacon -> jaeger "Sends traces to"
            towerOfBabel -> jaeger "Sends traces to"
        }
    }

    views {
        systemContext skyTracer "SystemContext" {
            include *
            autoLayout lr
            description "High-level view of the Sky Tracer aviation platform showing users and the main system"
        }

        container skyTracer "Containers" {
            include *
            autoLayout lr
            description "Container view showing all microservices, frontends, and infrastructure components"
        }

        # MCP Integration View - filtered from Containers
        container skyTracer "MCPIntegration" {
            include aiAssistant
            include skyNexus
            include airportAnywhere
            include flightController
            include orbitalBeacon
            include towerOfBabel
            autoLayout lr
            title "MCP Integration Architecture"
            description "Model Context Protocol integration showing AI assistant access to aviation data"
        }

        # Core Services View - filtered from Containers
        container skyTracer "CoreServices" {
            include skyNexus
            include airportAnywhere
            include flightController
            include orbitalBeacon
            include towerOfBabel
            autoLayout tb
            title "Core Aviation Services"
            description "Core aviation microservices and their relationships"
        }

        # Frontend View - filtered from Containers
        container skyTracer "UserFrontends" {
            include flightStaff
            include traveler
            include satelliteOperator
            include cockpit
            include flightmareTracker
            include delayORama
            include skyNexus
            autoLayout lr
            title "User Interface Applications"
            description "User-facing applications and their data sources"
        }

        # Infrastructure View - filtered from Containers
        container skyTracer "Infrastructure" {
            include traefik
            include jaeger
            include skyNexus
            include airportAnywhere
            include flightController
            include orbitalBeacon
            include towerOfBabel
            include delayORama
            include cockpit
            include flightmareTracker
            autoLayout tb
            title "Infrastructure and Routing"
            description "Infrastructure components and request routing"
        }

        component skyNexus "SkyNexusComponents" {
            include *
            autoLayout tb
            description "Sky Nexus integration hub components including MCP tools and REST API gateway"
        }

        component airportAnywhere "AirportComponents" {
            include *
            autoLayout tb
            description "Airport Anywhere service components for airport data management"
        }

        component flightController "FlightComponents" {
            include *
            autoLayout tb
            description "Flight Controller service components for flight management and tracking"
        }

        component orbitalBeacon "SatelliteComponents" {
            include *
            autoLayout tb
            description "Orbital Beacon service components for satellite tracking and positioning"
        }

        component towerOfBabel "TowerComponents" {
            include *
            autoLayout tb
            description "Tower of Babel service components for flight aggregation and real-time tracking"
        }

        component delayORama "DelayComponents" {
            include *
            autoLayout tb
            description "Delay-O-Rama service components for delay monitoring and visualization"
        }

        styles {
            element "Person" {
                shape Person
                background #08427B
                color #ffffff
            }
            element "Software System" {
                background #1168bd
                color #ffffff
            }
            element "Container" {
                background #438dd5
                color #ffffff
            }
            element "Component" {
                background #85bbf0
                color #000000
            }
            element "Web Browser" {
                shape WebBrowser
            }
            element "staff" {
                background #2e7d32
            }
            element "traveler" {
                background #d32f2f
            }
            element "operator" {
                background #f57c00
            }
            element "ai" {
                background #9c27b0
                color #ffffff
            }
            element "infrastructure" {
                background #607d8b
                color #ffffff
            }
            element "core" {
                background #1976d2
                color #ffffff
            }
            element "integration" {
                background #388e3c
                color #ffffff
            }
            element "frontend" {
                background #f57c00
                color #ffffff
            }
        }
    }
}
