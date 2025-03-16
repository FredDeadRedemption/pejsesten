import { writable } from "svelte/store";
import { io } from "socket.io-client";

// Initialize the socket with a given URL
function createSocket(url: string) {
  const socket = io(url);

  const messages = writable<string[]>([]);

  // Listen for messages
  socket.on("message", (data) => {
    messages.update((msgs) => [...msgs, data]);
  });

  // Function to send a message
  function sendMessage(message: string) {
    socket.emit("message", message);
  }

  return { socket, messages, sendMessage };
}

export default createSocket;