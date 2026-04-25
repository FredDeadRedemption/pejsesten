import { goto } from '$app/navigation';
import type { AttackData } from '$lib/shared/bindings/AttackData';
import type { GameStateClient } from '$lib/shared/bindings/GameStateClient';
import type { PlayerMetaData } from '$lib/shared/bindings/PlayerMetaData';
import { io, type Socket } from 'socket.io-client';
import type { GamePhase } from './shared/bindings/GamePhase';

let socket: Socket | null = null;

// Function to invalidate (disconnect) the socket

export let gameState = $state<GameStateClient>({
	white_player_id: '',
	black_player_id: '',
	enemy_board: {
		battlefield: [],
		hand: [],
		hero: {
			attack: 0,
			defence: 0,
			entity_id: 0
		},
		graveyard: [],
		deck: [],
		base_mana: 0,
		mana: 0,
		embers: 0
	},
	self_board: {
		battlefield: [],
		hand: [],
		hero: {
			attack: 0,
			defence: 0,
			entity_id: 0
		},
		graveyard: [],
		deck: [],
		base_mana: 0,
		mana: 0,
		embers: 0
	},
	turn_count: 0,
	cards_played_this_turn: 0,
	your_turn: false,
	phase: 'Mulligan' as GamePhase,
	mulligan_submitted: false
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
		Object.assign(gameState, newGameState);
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

export const queueUpBot = (data: PlayerMetaData) => fire(socket, 'queueUpBot', data);

export const leaveQueue = () => fire(socket, 'leaveQueue');

export const playCard = (data: { index: number; target?: number }) => {
	fire(socket, 'playCard', data);
	console.log('PlayCard', data.index, data.target);
};

export const tradeCard = (data: { index: number }) => fire(socket, 'tradeCard', data);

export const attack = (data: AttackData) => fire(socket, 'attack', data);

export const endTurn = () => fire(socket, 'endTurn');

export const submitMulligan = (indices: number[]) => fire(socket, 'submitMulligan', { indices });
