import { writable } from "svelte/store";
import { io } from "socket.io-client";

const socket = io("https://bachelorgameserver-production-4ede.up.railway.app/"); //Prod URL
//const socket = io("http://http://localhost:3000/"); //Dev URL

const messages = writable<string[]>([]);

// Listen for messages
socket.on("message", (data: string) => {
  messages.update((msgs) => [...msgs, data]);
});

// Function to send a message
function sendMessage(message: string) {
  socket.emit("message", message);
}

export { messages, sendMessage };