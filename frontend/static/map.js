// Initialise Map
var map = L.map('map').setView([35, 13], 4);

L.tileLayer('https://{s}.tile.jawg.io/jawg-light/{z}/{x}/{y}{r}.png?access-token={accessToken}', {
	attribution: '<a href="http://jawg.io" title="Tiles Courtesy of Jawg Maps" target="_blank">&copy; <b>Jawg</b>Maps</a> &copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
	minZoom: 0,
	maxZoom: 22,
	subdomains: 'abcd',
	accessToken: 'PJ7QkPxNaAADH2vKKJQO4J9zYicPbQjWt7PkGSQs9WWxMr4ea3y7uRmxoJoDsXKV',
}).addTo(map);

var flag  = L.icon({
    iconUrl: 'https://www.pngall.com/wp-content/uploads/2/Drawing-Pin.png',
    iconSize:     [20, 20],
    iconAnchor:   [0, 0], // point of the icon which will correspond to marker's location
    shadowAnchor: [4, 62],  // the same for the shadow
    popupAnchor:  [10, 10] // point from which the popup should open relative to the iconAnchor
});

// Plots nodes onto map for each step in diamonds history
function plotPoint(lat, lon, address, owner_rep, certification) {
    var marker = L.marker([lat, lon], {icon: flag}).addTo(map);
    marker.bindPopup(`<p> Owner Address: ${address} <br/> Owner Reputation: ${owner_rep} <br/> Certificate: ${certification} </p>`);
}

// address, owner reputability, 

plotPoint(35.126413,33.429859, "Cypress", "4 Stars", "Cert");
plotPoint(50.503887,4.469936, "Belgium", "5 Stars", "Cert");
plotPoint(46.227638,2.213749, "France", "5 Stars", "Cert");
plotPoint(55.378051,-3.435973, "UK", "4 Stars", "Cert");
plotPoint(31.791702,-7.09262, "Morocco", "4 Stars", "Cert");