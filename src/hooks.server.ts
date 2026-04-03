
import { type Handle } from '@sveltejs/kit';
import { building } from '$app/environment';
import { setupSocketIO } from '$lib/server/socket';
import { Server } from 'socket.io';

export let io: Server;

export const handle: Handle = async ({ event, resolve }) => {
  if (!io && !building) {
    // @ts-ignore — grab the underlying Node HTTP server
    const httpServer = event.platform?.server ?? globalThis.__socketio_server__;
    
    if (httpServer) {
      io = new Server(httpServer);
      setupSocketIO(io);
    } else {
      console.error('Failed to initialize Socket.IO: No HTTP server found.');
    }
  }

  return resolve(event);
};