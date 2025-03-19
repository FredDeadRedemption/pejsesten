import { goto } from '$app/navigation';
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

export const gameAvailable = writable<boolean>(false);
export const yourTurn = writable<boolean>(false);

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

    socket.on('startGame', (gameId : string) => {
      console.log(gameId);
      console.log("gameID");
      goto(`/game/${gameId}`);
    });

    socket.on('startTurn', (currentTurn) => {
      yourTurn.set(currentTurn);
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

export const leaveQueue = () => {
  if (socket) {
    console.log('leaving queueu:');
    socket.emit('leaveQueue');
  } else {
    console.error('Socket not connected. Please connect first.');
  }
};

export const endTurn = () => {
  if (socket) {
    console.log('endTurn:');
    socket.emit('endTurn');
  } else {
    console.error('Socket not connected. Please connect first.');
  }
};


