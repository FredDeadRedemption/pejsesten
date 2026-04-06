import { Server, type Socket } from 'socket.io';
import type { GameStateResponse } from '$lib/server/engine';
import { endTurn, setGameState, getGameState, playCard, attack } from '$lib/server/engine';
import { coinFlip, broadcastGameState, validateTurn } from '$lib/server/lib';
import { deckToCards } from '$lib/shared/cards';
import type { PlayerMetaData } from '$lib/shared/types';
import { makeBotMove } from '$lib/server/bot';
import { BOT_DELAY_MS } from './settings';

const BOT_ID = 'bot';

type ActiveGame = {
	socket1: Socket;
	socket2: Socket;
	isPlayer1White: boolean;
	botGame: boolean;
	botIsWhite?: boolean;
};
const LOGGING = true;
export let ioHandle: Server | null = null;
let queue: string[] = [];
let playerMetaDataMap: Map<string, PlayerMetaData> = new Map<string, PlayerMetaData>();
export let activeGame: ActiveGame | null = null;
// Paste your existing io.on("connection") logic here verbatim
export const setupSocketIO = (io: Server) => {
	ioHandle = io; // Store the io instance for later use if needed
	io.on('connection', (socket: Socket) => {
		// this funtion takes in a function (from the gameState module) with the signature (id: string, data?: any)
		// so thats either a function that just takes the socet id or a funtion that takes the socket id with some data
		// the fire funtion itself returns a funtion that first calls the gameSate funtion passed into it and then does the socket
		// broadcasting and turn validation.
		const fire = (fn: (socket: string, data?: any) => GameStateResponse) => {
			return (data: any) => {
				if (!activeGame) return; // validate active game

				const gamestate = getGameState();

				if (!validateTurn(socket.id, gamestate)) {
					console.log('NOT UR TURN');
					return;
				} // validate turn

				const result = fn(socket.id, data); // execute the given function

				if (result) broadcastGameState(result, activeGame);
			};
		};

		// after broadcastGameState check if it's bot's turn
		const fireAndMaybeBot = (fn: (socket: string, data?: any) => GameStateResponse) => {
			return (data: any) => {
				if (!activeGame) return;
				const gamestate = getGameState();
				if (!validateTurn(socket.id, gamestate)) {
					console.log('NOT UR TURN');
					return;
				}
				const result = fn(socket.id, data);
				if (result) {
					broadcastGameState(result, activeGame);
					// if bot game and it's now the bot's turn
					if (activeGame.botGame && !validateTurn(socket.id, getGameState())) {
						setTimeout(
							() => {
								const botResult = makeBotMove(getGameState(), activeGame!.botIsWhite!);
								if (botResult) broadcastGameState(botResult, activeGame!);
							},
							BOT_DELAY_MS
						);
					}
				}
			};
		};

		// Queue System
		socket.on('queueUp', (playerMetaData: PlayerMetaData) => {
			playerMetaDataMap.set(socket.id, playerMetaData);

			LOGGING && console.log(`QUEUING: ${playerMetaDataMap.get(socket.id)?.username}`);

			if (queue.includes(socket.id)) return;
			queue.push(socket.id);

			// When queue has 2 or more start them a game
			if (queue.length >= 2 && !activeGame) {
				// Pick two connections
				const socketId1 = queue.pop()!;
				const socketId2 = queue.pop()!;
				// Get their socket id's
				const socket1 = io.sockets.sockets.get(socketId1)!;
				const socket2 = io.sockets.sockets.get(socketId2)!;
				// Get their decks :D
				const player1MetaData = playerMetaDataMap.get(socket1.id);
				const player2MetaData = playerMetaDataMap.get(socket2.id);

				if (!player1MetaData || !player2MetaData) return;

				const isPlayer1White = coinFlip(); // Determine who goes first

				activeGame = { socket1, socket2, isPlayer1White, botGame: false }; // Set active game data

				const newGameState = setGameState(
					socket1.id,
					socket2.id,
					isPlayer1White,
					deckToCards(player1MetaData.choosenDeck),
					deckToCards(player2MetaData.choosenDeck)
				);

				broadcastGameState(newGameState, activeGame);

				const newGameURL = `${Math.floor(Math.random() * 10000)}${Date.now()}`;
				activeGame.socket1.emit('redirect', newGameURL);
				activeGame.socket2.emit('redirect', newGameURL);
			}
		});

		socket.on('queueBot', (playerMetaData: PlayerMetaData) => {
			console.log(playerMetaData.choosenDeck)
			const isPlayer1White = coinFlip();
			const botIsWhite = !isPlayer1White;
			activeGame = {
				socket1: socket,
				socket2: socket,
				isPlayer1White,
				botGame: true,
				botIsWhite,
			};

			const newGameState = setGameState(
				socket.id,
				BOT_ID,
				isPlayer1White,
				deckToCards(playerMetaData.choosenDeck),
				deckToCards(playerMetaData.choosenDeck)
			);

			broadcastGameState(newGameState, activeGame);
			const newGameURL = `${Math.floor(Math.random() * 10000)}${Date.now()}`;
			socket.emit('redirect', newGameURL);

			// bot goes first if it's white (white always goes first)
			if (botIsWhite) {
				setTimeout(() => {
					const botResult = makeBotMove(getGameState(), botIsWhite);
					if (botResult) broadcastGameState(botResult, activeGame!);
				}, 1000); // first move delay
			}
		});

		// Attach the handlers to events
		socket.on('endTurn', fireAndMaybeBot(endTurn));
		socket.on('playCard', fireAndMaybeBot(playCard));
		socket.on('attack', fireAndMaybeBot(attack));

		socket.on('resetServer', () => {
			activeGame = null;
			queue = [];
			playerMetaDataMap.clear();
			console.log('RESET GAME');
		});

		socket.on('leaveQueue', () => {
			LOGGING &&
				console.log(`Removing from queue (left): ${playerMetaDataMap.get(socket.id)?.username}`);
			queue = queue.filter((id) => id !== socket.id);
			playerMetaDataMap.delete(socket.id);
		});

		socket.on('disconnect', () => {
			LOGGING &&
				console.log(
					`Removing from queue (disconnect): ${playerMetaDataMap.get(socket.id)?.username}`
				);
			queue = queue.filter((id) => id !== socket.id);
			playerMetaDataMap.delete(socket.id);
		});
	});
};
