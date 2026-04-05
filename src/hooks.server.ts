
// src/hooks.server.ts
import type { Handle } from '@sveltejs/kit';
import { Server } from 'socket.io';
import { setupSocketIO } from '$lib/server/socket';

export const io = new Server(3002, { cors: { origin: '*' } });
setupSocketIO(io);
console.log("Socket.IO server running on port 3002");

export const handle: Handle = async ({ event, resolve }) => {
  return resolve(event);
};