import { writable } from "svelte/store";
import createSocket from "$lib/socket/socket";

const prodURL = "https://matrixz-gs.up.railway.app/";
const devURL = "http://localhost:3000/";

const socket = writable(createSocket(devURL)); //initialize connection to dev

const setSocketURL = (prod: boolean) => {
  socket.set(createSocket(prod ? prodURL : devURL)); 
}

export { setSocketURL, socket };