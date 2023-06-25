// Import the required dependencies for polkadot interaction
const { ApiPromise, Keyring } = require('@polkadot/api');
const { WsProvider } = require('@polkadot/rpc-provider');

const locationsDict = new Map([
    ["Cyprus", [35.126413, 33.429859]],
    ["Belgium", [50.503887, 4.469936]],
    ["France", [46.227638, 2.213749]],
    ["Morocco", [31.791702, -7.09262]],
    ["UK", [55.378051, -3.435973]],
]);

// Draws the map of previouse owners of a specific diamond
function displayDiamondHistory(diamond) {
    var prev = null;
    diamond.ownership_history.forEach((OwnershipChange) => {
        var latLon = locationsDict.get(OwnershipChange.previous_location);
        if (prev != null) {
            var cur = plotPoint(latLon[0], latLon[1], "", "", "");
            drawArc(cur, prev);
            prev = cur;
        } else {
            prev = plotPoint(latLon[0], latLon[1], "", "", "");
        }
    });
}


async function main() {

    // Connect to the local Substrate node
    const provider = new WsProvider('ws://localhost:9944');
    const api = await ApiPromise.create({ provider });

    // Set the account that will interact with the contract
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice', { name: 'Alice' });

    // Retrieve the contract instance
    const contractAddress = '0x1234567890abcdef'; // Replace with the actual contract address
    const contract = await api.query.contract.contractAt(contractAddress);

    // Retrieve the diamonds hashmap
    const diamonds = await contract.diamonds.entries();

    // Loop through the diamonds hashmap entries
    // diamonds.forEach(([diamondId, diamond]) => {
    //   console.log('Diamond ID:', diamondId.toNumber());
    //   console.log('Diamond:', diamond.toHuman());
    //   console.log('------------------------');
    // });
    // Prints each diamonds history

    // diamonds.forEach(([diamondId, diamond]) => {
    //     displayDiamondHistory(diamond);
    // })

    displayDiamondHistory(diamonds.get(userInput));
    // Disconnect from the Substrate node
    provider.disconnect();
}

main().catch(console.error);



// MAP:


// Initialise Map
var map = L.map('map').setView([35, 13], 4);

L.tileLayer('https://{s}.tile.jawg.io/jawg-light/{z}/{x}/{y}{r}.png?access-token={accessToken}', {
    attribution: '<a href="http://jawg.io" title="Tiles Courtesy of Jawg Maps" target="_blank">&copy; <b>Jawg</b>Maps</a> &copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
    minZoom: 0,
    maxZoom: 22,
    subdomains: 'abcd',
    accessToken: 'PJ7QkPxNaAADH2vKKJQO4J9zYicPbQjWt7PkGSQs9WWxMr4ea3y7uRmxoJoDsXKV',
}).addTo(map);

var pin = L.icon({
    iconUrl: 'https://www.pngall.com/wp-content/uploads/2/Drawing-Pin.png',
    iconSize: [20, 20],
    iconAnchor: [10, 10],
    popupAnchor: [0, 0]
});

// Plots nodes onto map for each step in diamonds history
function plotPoint(lat, lon, address, owner_rep, certification) {
    var marker = L.marker([lat, lon], { icon: pin }).addTo(map);
    marker.bindPopup(`<p> Owner Address: ${address} <br/> Owner Reputation: ${owner_rep} <br/> Certificate: ${certification} </p>`);
    return marker;
}

function drawArc(marker1, marker2) {
    L.Polyline.Arc(marker1.getLatLng(), marker2.getLatLng(), {
        color: 'red',
        vertices: 500
    }).addTo(map);
}

// Example : 
var p1 = plotPoint(35.126413, 33.429859, "Cyprus", "4 Stars", "Cert");
var p2 = plotPoint(50.503887, 4.469936, "Belgium", "5 Stars", "Cert");
var p3 = plotPoint(46.227638, 2.213749, "France", "5 Stars", "Cert");
var p4 = plotPoint(55.378051, -3.435973, "UK", "4 Stars", "Cert");
var p5 = plotPoint(31.791702, -7.09262, "Morocco", "4 Stars", "Cert");

drawArc(p1, p2);
drawArc(p2, p3);
drawArc(p3, p4);
drawArc(p4, p5);