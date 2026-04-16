import http from 'http';
import { Server } from 'socket.io';
// @ts-ignore - build folder is generated at build time
import { handler } from './build/handler.js';

const server = http.createServer(handler);
const io = new Server(server, { cors: { origin: '*' } });

const { setupSocketIO } = await import('./src/lib/server/socket.js');
setupSocketIO(io);

const PORT = parseInt(process.env.PORT || '3002');
server.listen(PORT, '0.0.0.0', () => {
  console.log(`Running on port ${PORT}`);
});