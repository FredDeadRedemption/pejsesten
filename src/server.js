import http from 'http';
import { Server } from 'socket.io';
import { handler } from './build/handler.js';
import { setupSocketIO } from './lib/server/socket.js';

const PORT = process.env.PORT || 3000;
const server = http.createServer(handler);

setupSocketIO(new Server(server, { cors: { origin: '*' } }));

server.listen(PORT, () => {
  console.log(`Running on port ${PORT}`);
});