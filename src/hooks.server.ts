
// src/hooks.server.ts
import type { Handle } from '@sveltejs/kit';
import { Server } from 'socket.io';
import { setupSocketIO } from '$lib/server/socket';

declare global {
  var __socketio__: Server | undefined;
}

// prevent re-binding on hot reload
if (!global.__socketio__) {
  const io = new Server(3002, { cors: { origin: '*' } });
  setupSocketIO(io);
  global.__socketio__ = io;
  console.log("Socket.IO server running on port 3002");
}

export const handle: Handle = async ({ event, resolve }) => {
  return resolve(event);
};