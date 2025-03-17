import { redirect } from '@sveltejs/kit';
import { io, type Socket } from 'socket.io-client';

let socket: Socket | null = null;

// Function to invalidate (disconnect) the socket
export const invalidateSocket = () => {
  if (socket) {
    console.log('Disconnecting socket...');
    socket.disconnect();
    socket = null;
  }
};

// Function to connect to the socket server
export const connectSocket = (url: string) => {
  if (!socket) {
    console.log('Creating socket connection...');
    socket = io(url);

    // Listen for connection events
    socket.on('connect', () => {
      console.log('Connected to socket server:', socket?.id);
    });

    socket.on('connect_error', (err) => {
      console.error('Socket connection error:', err);
      invalidateSocket(); // Disconnect and reset the socket on error
    });

    // Register global event listeners
    socket.on('queueAccept', () => {
      console.log('Queue accepted! Redirecting to game...');
      // Handle redirection logic here (e.g., using SvelteKit's `goto` or window.location)
    });
  } else {
    console.log('Socket already connected:', socket.id);
  }
};

// Function to queue up
export const queueUp = (userName: string) => {
  if (socket) {
    console.log('Queuing up:', userName);
    socket.emit('queueUp', userName);
  } else {
    console.error('Socket not connected. Please connect first.');
  }
};