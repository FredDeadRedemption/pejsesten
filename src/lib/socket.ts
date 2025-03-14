import { writable } from "svelte/store";
import { io } from "socket.io-client";

const socket = io("http://localhost:5174"); // Update with your server URL

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