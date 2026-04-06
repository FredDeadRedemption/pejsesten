// server.js
import http from 'http';
import { Server } from 'socket.io';
import { handler } from './build/handler.js';

// inline the setup or use a relative path that exists at runtime
const PORT = process.env.PORT || 3000;
const server = http.createServer(handler);
const io = new Server(server, { cors: { origin: '*' } });

// import directly from src — it's still there in prod
const { setupSocketIO } = await import('./src/lib/server/socket.js');
setupSocketIO(io);

server.listen(PORT, () => {
  console.log(`Running on port ${PORT}`);
});