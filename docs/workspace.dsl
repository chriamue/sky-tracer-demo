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

        skyTracer = softwareSystem "🛩️ Sky Tracer System" "Flight tracking and monitoring system" {
            # Airport Anywhere Container and Components
            airportAnywhere = container "🏢 Airport Anywhere" "Airport information lookup service" "Rust/Axum" {
                component "Airport Service" "Manages airport data and search" "Rust"
                component "Airport API" "REST API for airport lookups" "Rust/Axum"
                component "Airport UI" "Airport search interface" "Rust/Yew SSR"
            }

            # Flight Controller Container and Components
            flightController = container "🎮 Flight Controller" "Flight management and tracking" "Rust/Axum" {
                component "Flight Service" "Manages flight data" "Rust"
                component "Flight API" "REST API for flight operations" "Rust/Axum"
                component "Flight UI" "Flight management interface" "Rust/Yew SSR"
                component "Position Calculator" "Calculates flight positions" "Rust"
            }

            # Orbital Beacon Container and Components
            orbitalBeacon = container "🛰️ Orbital Beacon" "Satellite tracking and positioning system" "Rust/Axum" {
                component "Satellite Service" "Manages satellites and positions" "Rust"
                component "Position API" "REST API for position calculations" "Rust/Axum"
                component "Satellite UI" "Satellite management interface" "Rust/Yew SSR"
                component "Launch Control" "Satellite deployment interface" "Rust/Yew SSR"
            }

            # Tower of Babel Container and Components
            towerOfBabel = container "🗼 Tower of Babel" "Flight aggregation and position service" "Rust/Axum" {
                component "Flight Aggregator" "Aggregates flight information" "Rust"
                component "Position API" "REST API for flight positions" "Rust/Axum"
            }

            # Delay-O-Rama Container and Components
            delayORama = container "⏰ Delay-O-Rama" "Flight delay monitoring service" "Rust/Axum" {
                component "Delay Monitor" "Monitors flight delays" "Rust"
                component "Delay UI" "Public delay visualization interface" "Rust/Yew SSR"
            }

            # Web Frontends
            cockpit = container "🎯 Cockpit" "Staff flight monitoring dashboard" "Rust/Yew/WebAssembly" {
                component "Flight Map" "Interactive flight map" "Rust/Yew"
                component "Flight List" "Real-time flight list" "Rust/Yew"
                component "Status Panel" "Flight status display" "Rust/Yew"
            }

            flightmareTracker = container "😱 Flightmare Tracker" "Public delay simulation viewer" "Rust/Yew/WebAssembly" {
                component "Delay Simulator" "Simulates flight delays" "Rust/Yew"
                component "Delay Display" "Visualizes delay patterns" "Rust/Yew"
            }

            # Staff Relationships
            flightStaff -> cockpit "Monitors flights using"
            flightStaff -> airportAnywhere "Looks up airport information using"

            # Traveler Relationships
            traveler -> delayORama "Checks real-time delays using"
            traveler -> flightmareTracker "Views delay patterns using"

            # Satellite Operator Relationships
            satelliteOperator -> orbitalBeacon "Manages and launches satellites using"

            # System Relationships
            towerOfBabel -> flightController "Fetches flight data from"
            towerOfBabel -> orbitalBeacon "Gets position data from"

            delayORama -> towerOfBabel "Gets flight information from"

            flightController -> airportAnywhere "Looks up airport data from"
            flightController -> orbitalBeacon "Calculates positions using"

            orbitalBeacon -> airportAnywhere "Gets airport positions from"

            cockpit -> flightController "Gets flight data from"
            cockpit -> orbitalBeacon "Gets position updates from"

            flightmareTracker -> flightController "Simulates delays using flight data from"
        }
    }

    views {
        systemContext skyTracer "SystemContext" {
            include *
            autoLayout lr
        }

        container skyTracer "Containers" {
            include *
            autoLayout lr
        }

        component airportAnywhere "AirportComponents" {
            include *
            autoLayout tb
        }

        component flightController "FlightComponents" {
            include *
            autoLayout tb
        }

        component orbitalBeacon "SatelliteComponents" {
            include *
            autoLayout tb
        }

        component towerOfBabel "TowerComponents" {
            include *
            autoLayout tb
        }

        component delayORama "DelayComponents" {
            include *
            autoLayout tb
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
        }
    }
}
