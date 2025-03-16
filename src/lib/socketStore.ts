import { writable } from "svelte/store";
import createSocket from "$lib/socket";

// Define the URLs
const prodURL = "https://matrixz-gs.up.railway.app/";
const devURL = "http://localhost:3000/";

// Create a writable store for the current URL
const socketURL = writable(prodURL);

// Create a writable store for the socket module
const socketModule = writable(createSocket(prodURL));

// Function to update the URL and reload the socket module
function setSocketURL(prod: boolean) {
  const url = prod ? prodURL : devURL;
  socketURL.set(url); // Update the URL store
  socketModule.set(createSocket(url)); // Reload the socket module
}

// Export the stores and functions
export { socketURL, setSocketURL, socketModule };