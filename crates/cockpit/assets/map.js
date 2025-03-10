class FlightMap {
  static maps = new Map();

  // Define initial view settings for FRA-LIS
  static initialView = {
    center: [45.0, 0.0], // Between Frankfurt and Lisbon
    zoom: 4,
  };

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
    const departureMarker = L.marker(flight.departure.position, {
      icon: this.createIcon("#4CAF50", flight.departure.code),
    });

    const arrivalMarker = L.marker(flight.arrival.position, {
      icon: this.createIcon("#f44336", flight.arrival.code),
    });

    let flightPath = null;
    let aircraftMarker = null;

    if (flight.position) {
      aircraftMarker = L.marker(flight.position, {
        icon: this.createIcon("#2196F3", "✈️", 20),
      }).bindPopup(`
                Flight ${flight.flightNumber}<br>
                From: ${flight.departure.code}<br>
                To: ${flight.arrival.code}<br>
                Lat: ${flight.position[0].toFixed(4)}<br>
                Lon: ${flight.position[1].toFixed(4)}
            `);

      flightPath = L.polyline(
        [flight.departure.position, flight.position, flight.arrival.position],
        {
          color: "#2196F3",
          weight: 2,
          dashArray: "5, 10",
          opacity: 0.6,
        },
      );
    } else {
      flightPath = L.polyline(
        [flight.departure.position, flight.arrival.position],
        {
          color: "#2196F3",
          weight: 2,
          dashArray: "5, 10",
          opacity: 0.3,
        },
      );
    }

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

      let map = this.maps.get(containerId);

      // Create new map only if it doesn't exist
      if (!map) {
        map = L.map(containerId, {
          center: this.initialView.center,
          zoom: this.initialView.zoom,
          minZoom: 3,
          maxZoom: 10,
        });

        L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
          attribution: "© OpenStreetMap contributors",
        }).addTo(map);

        this.maps.set(containerId, map);
        map.flightMarkers = new Map(); // Store flight markers on the map object
      }

      // Update or add flight markers
      mapData.flights.forEach((flight) => {
        let flightMarkers = map.flightMarkers.get(flight.flightNumber);

        if (flightMarkers) {
          // Update existing markers
          flightMarkers.departureMarker.setLatLng(flight.departure.position);
          flightMarkers.arrivalMarker.setLatLng(flight.arrival.position);

          // Update aircraft marker and path
          if (flight.position && flightMarkers.aircraftMarker) {
            flightMarkers.aircraftMarker.setLatLng(flight.position);
            flightMarkers.flightPath.setLatLngs([
              flight.departure.position,
              flight.position,
              flight.arrival.position,
            ]);
            // Ensure aircraft marker is added to the map
            if (!map.hasLayer(flightMarkers.aircraftMarker)) {
              flightMarkers.aircraftMarker.addTo(map);
            }
          } else if (flight.position && !flightMarkers.aircraftMarker) {
            // Create aircraft marker if it doesn't exist
            const aircraftMarker = L.marker(flight.position, {
              icon: this.createIcon("#2196F3", "✈️", 20),
            }).bindPopup(`
                      Flight ${flight.flightNumber}<br>
                      From: ${flight.departure.code}<br>
                      To: ${flight.arrival.code}<br>
                      Lat: ${flight.position[0].toFixed(4)}<br>
                      Lon: ${flight.position[1].toFixed(4)}
                  `);
            aircraftMarker.addTo(map);
            flightMarkers.aircraftMarker = aircraftMarker;
          } else if (!flight.position && flightMarkers.aircraftMarker) {
            // Remove aircraft marker if position is no longer available
            map.removeLayer(flightMarkers.aircraftMarker);
            flightMarkers.aircraftMarker = null;
          }

          flightMarkers.flightPath.setLatLngs([
            flight.departure.position,
            flight.position ? flight.position : flight.arrival.position,
          ]);
        } else {
          // Create new markers
          const { departureMarker, arrivalMarker, flightPath, aircraftMarker } =
            this.createFlightMarker(flight);

          departureMarker.addTo(map);
          arrivalMarker.addTo(map);
          flightPath.addTo(map);
          if (aircraftMarker) {
            aircraftMarker.addTo(map);
          }

          map.flightMarkers.set(flight.flightNumber, {
            departureMarker,
            arrivalMarker,
            flightPath,
            aircraftMarker,
          });
        }
      });
    } catch (error) {
      console.error("Error initializing flight map:", error);
      console.error("Map data:", mapDataString);
    }
  }

  static cleanup(containerId) {
    if (this.maps.has(containerId)) {
      const map = this.maps.get(containerId);
      map.eachLayer((layer) => {
        if (layer instanceof L.Marker || layer instanceof L.Polyline) {
          map.removeLayer(layer);
        }
      });
      this.maps.delete(containerId);
    }
  }
}

// Export to window
window.initializeFlightsMap = FlightMap.initialize.bind(FlightMap);
window.cleanupFlightMap = FlightMap.cleanup.bind(FlightMap);
