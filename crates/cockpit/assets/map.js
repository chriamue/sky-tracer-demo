class FlightMap {
  static maps = new Map();

  static createIcon(color, text, size = 30) {
    return L.divIcon({
      className: "custom-div-icon",
      html: `<div style="background-color: ${color}; padding: 5px; border-radius: 50%; color: white;">
                    ${text}</div>`,
      iconSize: [size, size],
      iconAnchor: [size / 2, size / 2],
    });
  }

  static createFlightMarker(flight) {
    // Create departure marker
    const departureMarker = L.marker(flight.departure.position, {
      icon: this.createIcon("#4CAF50", flight.departure.code),
    });

    // Create arrival marker
    const arrivalMarker = L.marker(flight.arrival.position, {
      icon: this.createIcon("#f44336", flight.arrival.code),
    });

    // Create flight path and aircraft marker if position exists
    const flightPath = flight.position
      ? L.polyline(
          [flight.departure.position, flight.position, flight.arrival.position],
          {
            color: "#2196F3",
            weight: 2,
            dashArray: "5, 10",
            opacity: 0.6,
          },
        )
      : L.polyline([flight.departure.position, flight.arrival.position], {
          color: "#2196F3",
          weight: 2,
          dashArray: "5, 10",
          opacity: 0.3,
        });

    const aircraftMarker = flight.position
      ? L.marker(flight.position, {
          icon: this.createIcon("#2196F3", "✈️", 20),
        }).bindPopup(`
                Flight ${flight.flightNumber}<br>
                From: ${flight.departure.code}<br>
                To: ${flight.arrival.code}<br>
                Lat: ${flight.position[0].toFixed(4)}<br>
                Lon: ${flight.position[1].toFixed(4)}
            `)
      : null;

    return {
      departureMarker,
      arrivalMarker,
      flightPath,
      aircraftMarker,
    };
  }

  static initialize(containerId, mapDataString) {
    try {
      const mapData = JSON.parse(mapDataString);

      if (typeof L === "undefined") {
        throw new Error("Leaflet not loaded");
      }

      const container = document.getElementById(containerId);
      if (!container) {
        throw new Error("Map container not found");
      }

      // Clean up existing map
      this.cleanup(containerId);

      // Initialize new map
      const map = L.map(containerId).setView([50.0, 10.0], 4);
      this.maps.set(containerId, map);

      // Add tile layer
      L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
        attribution: "© OpenStreetMap contributors",
      }).addTo(map);

      // Add flight markers and paths
      const markers = [];
      mapData.flights.forEach((flight) => {
        const { departureMarker, arrivalMarker, flightPath, aircraftMarker } =
          this.createFlightMarker(flight);

        departureMarker.addTo(map);
        arrivalMarker.addTo(map);
        flightPath.addTo(map);
        if (aircraftMarker) {
          aircraftMarker.addTo(map);
        }

        markers.push(
          flight.departure.position,
          flight.arrival.position,
          ...(flight.position ? [flight.position] : []),
        );
      });

      // Fit bounds if there are markers
      if (markers.length > 0) {
        const bounds = L.latLngBounds(markers);
        map.fitBounds(bounds, { padding: [50, 50] });
      }
    } catch (error) {
      console.error("Error initializing flight map:", error);
      console.error("Map data:", mapDataString);
    }
  }

  static cleanup(containerId) {
    if (this.maps.has(containerId)) {
      this.maps.get(containerId).remove();
      this.maps.delete(containerId);
    }
  }
}

// Export to window
window.initializeFlightsMap = FlightMap.initialize.bind(FlightMap);
window.cleanupFlightMap = FlightMap.cleanup.bind(FlightMap);
