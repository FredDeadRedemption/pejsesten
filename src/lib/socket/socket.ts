import { goto } from '$app/navigation';
import { io, type Socket } from 'socket.io-client';
import { writable } from 'svelte/store';
import type { PlayerMetaData, GameState, AttackData } from "$lib/sharedTypes.js";

let socket: Socket | null = null;

// Function to invalidate (disconnect) the socket

export const gameState = writable<GameState>();

type manas = {
  green: number,
  orange: number,
  red: number,
  purple: number,
  black: number,
  white: number,
}

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

  socket.on("cardDrawn", (card: string) => {
    console.log("Card drawn:", card);
  });

  type atk = { 
    targetAttack: number,
    attackerAttack: number,
    targetDied: boolean,
    attackerDied: boolean,
    attackData: AttackData,
  }

  socket.on("attacked", (data: atk) => {
    console.log("ATK:" + data.attackerAttack)
    console.log("ATK:" + data.targetAttack)
    console.log("ATK:" + data.targetDied)
    console.log("ATK:" + data.attackerDied)
    console.log("ATK:" + data.attackData)
  })

  socket.on("mana", (manas: manas) => {
    console.log("MANAS red: " + manas.red);
    console.log("MANAS purple: " + manas.purple);
    console.log("MANAS black: " + manas.black);
    console.log("MANAS white: " + manas.white);
    console.log("MANAS orange: " + manas.orange);
    console.log("MANAS green: " + manas.green);
  });

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

export const playCard = (index: number) => fire(socket, "playCard", index);

export const attack = (data: AttackData) => fire(socket, "attack", data);

export const leaveQueue = () => fire(socket, "leaveQueue")

export const endTurn = () => fire(socket, "endTurn")
 
export const drawCard = () => fire(socket, "drawCard")


