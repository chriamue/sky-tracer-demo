workspace {
    model {
        user = person "Flight Staff" "Monitors flight information and tracks planes"

        skyTracer = softwareSystem "Sky Tracer System" "Flight tracking and monitoring system" {
            # Airport Anywhere Container and Components
            airportAnywhere = container "Airport Anywhere" "Airport information lookup service" "Rust/Axum" {
                component "Airport Service" "Manages airport data" "Rust"
                component "Airport API" "REST API for airport lookups" "Rust/Axum"
                component "Airport UI" "Airport search interface" "Rust/Yew"
            }

            # Flight Controller Container and Components
            flightController = container "Flight Controller" "Flight management and tracking" "Rust/Axum" {
                component "Flight Service" "Manages flight data" "Rust"
                component "Flight API" "REST API for flight operations" "Rust/Axum"
                component "Flight UI" "Flight management interface" "Rust/Yew"
            }

            # Orbital Beacon Container and Components
            orbitalBeacon = container "Orbital Beacon" "Satellite positioning system" "Rust/Axum" {
                component "Satellite Service" "Manages satellites" "Rust"
                component "Satellite API" "REST API for satellite positioning" "Rust/Axum"
                component "Satellite UI" "Satellite management interface" "Rust/Yew"
            }

            # Web Frontends
            cockpit = container "Cockpit" "Flight monitoring dashboard" "Rust/Yew/WebAssembly" {
                component "Cockpit UI" "Flight monitoring dashboard" "Rust/Yew"
            }

            flightmareTracker = container "Flightmare Tracker" "Flight delay monitoring" "Rust/Yew/WebAssembly" {
                component "Delay UI" "Delay monitoring interface" "Rust/Yew"
            }

            # Relationships
            user -> cockpit "Views flights using"
            user -> flightmareTracker "Monitors delays using"

            flightController -> airportAnywhere "Looks up airport data from"
            orbitalBeacon -> airportAnywhere "Gets airport positions from"
            cockpit -> flightController "Gets flight data from"
            flightmareTracker -> flightController "Monitors flights using"
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
        }
    }
}
