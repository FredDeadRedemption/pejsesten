
// src/hooks.server.ts
import type { Handle } from '@sveltejs/kit';
import { Server } from 'socket.io';
import { createServer } from 'net';
import { setupSocketIO } from '$lib/server/socket';

const isPortInUse = (port: number): Promise<boolean> =>
  new Promise((resolve) => {
    const tester = createServer()
      .once('error', () => resolve(true))
      .once('listening', () => tester.close(() => resolve(false)))
      .listen(port);
  });

if (!(await isPortInUse(3002))) {
  const { Server } = await import('socket.io');
  const { setupSocketIO } = await import('$lib/server/socket');
  const io = new Server(3002, { cors: { origin: '*' } });
  setupSocketIO(io);
  console.log("Socket.IO server running on port 3002");
}

export const handle: Handle = async ({ event, resolve }) => {
  return resolve(event);
};