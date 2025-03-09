window.flightMaps = new Map();

window.initializeFlightsMap = function (containerId, mapDataString) {
  try {
    const mapData = JSON.parse(mapDataString);

    if (typeof L === "undefined") {
      console.error("Leaflet not loaded");
      return;
    }

    const container = document.getElementById(containerId);
    if (!container) {
      console.error("Map container not found");
      return;
    }

    // Clean up existing map if it exists
    if (window.flightMaps.has(containerId)) {
      window.flightMaps.get(containerId).remove();
      window.flightMaps.delete(containerId);
    }

    // Initialize map centered on Europe
    const map = L.map(containerId).setView([50.0, 10.0], 4);
    window.flightMaps.set(containerId, map);

    L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
      attribution: "© OpenStreetMap contributors",
    }).addTo(map);

    // Add markers and routes for each flight
    mapData.flights.forEach((flight) => {
      // Add departure airport marker
      L.marker(flight.departure.position, {
        icon: L.divIcon({
          className: "custom-div-icon",
          html: `<div style="background-color: #4CAF50; padding: 5px; border-radius: 50%; color: white;">
                        ${flight.departure.code}</div>`,
          iconSize: [30, 30],
          iconAnchor: [15, 15],
        }),
      }).addTo(map);

      // Add arrival airport marker
      L.marker(flight.arrival.position, {
        icon: L.divIcon({
          className: "custom-div-icon",
          html: `<div style="background-color: #f44336; padding: 5px; border-radius: 50%; color: white;">
                        ${flight.arrival.code}</div>`,
          iconSize: [30, 30],
          iconAnchor: [15, 15],
        }),
      }).addTo(map);

      // If there's a current position, add aircraft marker
      if (flight.position) {
        const marker = L.marker(flight.position, {
          icon: L.divIcon({
            className: "custom-div-icon",
            html: `<div style="background-color: #2196F3; padding: 5px; border-radius: 50%; color: white;">
                            ✈️</div>`,
            iconSize: [20, 20],
            iconAnchor: [10, 10],
          }),
        })
          .bindPopup(
            `
                    Flight ${flight.flightNumber}<br>
                    From: ${flight.departure.code}<br>
                    To: ${flight.arrival.code}<br>
                    Lat: ${flight.position[0].toFixed(4)}<br>
                    Lon: ${flight.position[1].toFixed(4)}
                `,
          )
          .addTo(map);

        // Draw flight path
        L.polyline(
          [flight.departure.position, flight.position, flight.arrival.position],
          {
            color: "#2196F3",
            weight: 2,
            dashArray: "5, 10",
            opacity: 0.6,
          },
        ).addTo(map);
      } else {
        // If no position, just draw direct route
        L.polyline([flight.departure.position, flight.arrival.position], {
          color: "#2196F3",
          weight: 2,
          dashArray: "5, 10",
          opacity: 0.3,
        }).addTo(map);
      }
    });

    // Fit bounds to show all markers if there are flights
    if (mapData.flights.length > 0) {
      const bounds = L.latLngBounds(
        mapData.flights.flatMap((flight) => [
          flight.departure.position,
          flight.arrival.position,
          ...(flight.position ? [flight.position] : []),
        ]),
      );
      map.fitBounds(bounds, { padding: [50, 50] });
    }
  } catch (error) {
    console.error("Error initializing flight map:", error);
    console.error("Map data:", mapDataString);
  }
};

// Cleanup function
window.cleanupFlightMap = function (containerId) {
  if (window.flightMaps.has(containerId)) {
    window.flightMaps.get(containerId).remove();
    window.flightMaps.delete(containerId);
  }
};
