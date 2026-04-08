import { goto } from '$app/navigation';
import { io, type Socket } from 'socket.io-client';
import type { PlayerMetaData, GameStateClient, AttackData } from '$lib/shared/types';

let socket: Socket | null = null;

// Function to invalidate (disconnect) the socket

export let gameState = $state<GameStateClient>({
	whitePlayerID: '',
	blackPlayerID: '',
	enemy: {
		battlefield: [],
		hand: [],
		hero: {
			attack: 0,
			defence: 0
		},
		graveyard: [],
		deck: [],
		baseMana: 0,
		mana: 0
	},
	self: {
		battlefield: [],
		hand: [],
		hero: {
			attack: 0,
			defence: 0
		},
		graveyard: [],
		deck: [],
		baseMana: 0,
		mana: 0
	},
	turnCount: 0,
	cardsPlayedThisTurn: 0,
	yourTurn: false
});

// Function to connect to the socket server
export const connectSocket = (url: string) => {
	if (socket) return;

	console.log('Creating socket connection...');
	socket = io(url);

	socket.on('redirect', (URL) => {
		goto(`/${URL}`);
	});

	socket.on('newGameState', (newGameState: GameStateClient) => {
		gameState.self = newGameState.self;
		gameState.enemy = newGameState.enemy;
		gameState.whitePlayerID = newGameState.whitePlayerID;
		gameState.blackPlayerID = newGameState.blackPlayerID;
		gameState.yourTurn = newGameState.yourTurn;
		gameState.turnCount = newGameState.turnCount;
		gameState.cardsPlayedThisTurn = newGameState.cardsPlayedThisTurn;
	});

	socket.on('cardDrawn', (card: string) => {
		console.log('Card drawn:', card);
	});

	// Listen for connection events
	socket.on('connect', () => {
		console.log('Connected to socket server:', socket?.id);
	});

	socket.on('connect_error', (err) => {
		console.error('Socket connection error:', err);
		invalidateSocket(); // Disconnect and reset the socket on error
	});
};

// Fires any event with optional data
const fire = (socket: Socket | null, event: string, data?: any) => {
	socket ? socket.emit(event, data) : console.error('socket is null during event: ' + event);
};

export const invalidateSocket = () => {
	socket?.disconnect();
	socket = null;
	console.log('disconnected socket');
};

export const resetServer = () => fire(socket, 'resetServer');

export const queueUp = (data: PlayerMetaData) => fire(socket, 'queueUp', data);

export const queueUpBot = (data: PlayerMetaData) => fire(socket, 'queueBot', data);

export const leaveQueue = () => fire(socket, 'leaveQueue');

export const playCard = (data: { index: number; target?: string }) => {
	fire(socket, 'playCard', data);
	console.log('PlayCard', data.index, data.target);
};

export const attack = (data: AttackData) => fire(socket, 'attack', data);

export const endTurn = () => fire(socket, 'endTurn');
