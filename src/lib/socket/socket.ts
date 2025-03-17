import { writable } from "svelte/store";
import { io } from "socket.io-client";

const p = "https://matrixz-gs.up.railway.app/";
const d = "http://localhost:3000/";

const socketURL = p; // CHANGE URL HERE <------------

export const socketLabel = socketURL === p ? "Production" : "Development"

const socket = io(socketURL);
console.log("CREATING SOCKET SERVER")

export const messages = writable<string[]>([]);

// Listen for messages
socket.on("message", (data) => {
  messages.update((msgs) => [...msgs, data]);
});

// Function to send a message
export const sendMessage = (message: string) => {
  socket.emit("message", message);
}

// Queue up
export const queueUp = (userName: string) => {
  socket.emit("queueUp", userName);
}
