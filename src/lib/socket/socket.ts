import { goto } from '$app/navigation';
import { redirect } from '@sveltejs/kit';
import { io, type Socket } from 'socket.io-client';
import { writable } from 'svelte/store';

let socket: Socket | null = null;

export const queueHasPartner = writable(false);

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
      queueHasPartner.set(true);
      // Handle redirection logic here (e.g., using SvelteKit's `goto` or window.location)
    });

    socket.on("startGame", () => {
      goto("/yeehaw")
    })
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

export const readyUp = () => {
  if (socket) {
    console.log('Readying up');
    socket.emit('readyUp');
  } else {
    console.error('Socket not connected. Please connect first.');
  }
}