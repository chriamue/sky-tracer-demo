function initializeMap(containerId, mapData) {
  if (typeof L === "undefined") {
    console.error("Leaflet not loaded");
    return;
  }

  const container = document.getElementById(containerId);
  if (!container) {
    console.error("Map container not found");
    return;
  }

  const map = L.map(containerId).setView(mapData.currentPosition, 4);
  L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
    attribution: "© OpenStreetMap contributors",
  }).addTo(map);

  // Add departure airport marker
  L.marker(mapData.departure, {
    icon: L.divIcon({
      className: "custom-div-icon",
      html: `<div style="background-color: #4CAF50; padding: 5px; border-radius: 50%; color: white;">
                    ${mapData.depCode}</div>`,
      iconSize: [30, 30],
      iconAnchor: [15, 15],
    }),
  }).addTo(map);

  // Add arrival airport marker
  L.marker(mapData.arrival, {
    icon: L.divIcon({
      className: "custom-div-icon",
      html: `<div style="background-color: #f44336; padding: 5px; border-radius: 50%; color: white;">
                    ${mapData.arrCode}</div>`,
      iconSize: [30, 30],
      iconAnchor: [15, 15],
    }),
  }).addTo(map);

  // Add current position marker
  L.marker(mapData.currentPosition, {
    icon: L.divIcon({
      className: "custom-div-icon",
      html: '<div style="background-color: #2196F3; padding: 5px; border-radius: 50%;"></div>',
      iconSize: [15, 15],
      iconAnchor: [7.5, 7.5],
    }),
  }).addTo(map);

  // Draw flight path
  L.polyline([mapData.departure, mapData.currentPosition, mapData.arrival], {
    color: "#2196F3",
    weight: 2,
    dashArray: "5, 10",
    opacity: 0.6,
  }).addTo(map);

  // Fit bounds to show all markers
  const bounds = L.latLngBounds([
    mapData.departure,
    mapData.currentPosition,
    mapData.arrival,
  ]);
  map.fitBounds(bounds, { padding: [50, 50] });
}

// Initialize maps when DOM is loaded
document.addEventListener("DOMContentLoaded", function () {
  const mapElements = document.querySelectorAll("[data-map]");
  mapElements.forEach((element) => {
    const mapData = JSON.parse(element.getAttribute("data-map"));
    initializeMap(element.id, mapData);
  });
});
