import { goto } from '$app/navigation';
import { io, type Socket } from 'socket.io-client';
import { writable } from 'svelte/store';
import type { PlayerMetaData, GameState } from "$lib/sharedTypes.js";

let socket: Socket | null = null;

// Function to invalidate (disconnect) the socket

export const gameState = writable<GameState>();

// Function to connect to the socket server
export const connectSocket = (url: string) => {
  if (socket) return

  console.log('Creating socket connection...');
  socket = io(url);

  socket.on("redirect", (URL) => {
    goto(`/game/${URL}`);
  })

  socket.on("newGameState", (newGameState: GameState) => {
    gameState.set(newGameState);
    console.log(newGameState) // log fra helvede
  })

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
};

// Fires any event with optional data
const fire = (socket: Socket | null, event: string, data?: any) => {
  socket ? socket.emit(event, data) : console.error("socket is null during event: " + event)
}

export const invalidateSocket = () => { socket?.disconnect(); socket = null; console.log("disconnected socket")}

export const queueUp = (data: PlayerMetaData) => fire(socket, "queueUp", data)

export const leaveQueue = () => fire(socket, "leaveQueue")

export const endTurn = () => fire(socket, "endTurn")
 
export const drawCard = () => fire(socket, "drawCard")


