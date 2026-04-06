import http from 'http';
import { Server } from 'socket.io';
// @ts-ignore - build folder is generated at build time
import { handler } from './build/handler.js';

const PORT = process.env.PORT || 3000;
const server = http.createServer(handler);
const io = new Server(server, { cors: { origin: '*' } });

const { setupSocketIO } = await import('./src/lib/server/socket.js');
setupSocketIO(io);

server.listen(PORT, () => {
  console.log(`Running on port ${PORT}`);
});