import { writable } from "svelte/store";
import { io } from "socket.io-client";

const p = "https://matrixz-gs.up.railway.app/";
const d = "http://localhost:3000/";

const socketURL = p; // CHANGE URL HERE <------------

export const socketLabel = socketURL === p ? "Production" : "Development"

const socket = io(socketURL);
console.log("CREATING SOCKET SERVER")

// Queue up
export const queueUp = (userName: string) => {
  socket.emit("queueUp", userName);
}
