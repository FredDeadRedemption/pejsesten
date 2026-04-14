import type {
	CardEntity,
	FollowUpRequirement,
	GameStateClient,
	GameStateServer,
  Hero
} from '$lib/shared/types';
import type { Socket } from 'socket.io';

// fisher yates shuffle
export const shuffle = (deck: CardEntity[]) => {
	for (let i = deck.length - 1; i > 0; i--) {
		const j = Math.floor(Math.random() * (i + 1));
		[deck[i], deck[j]] = [deck[j], deck[i]];
	}
	return deck;
};

export const isMinion = (entity: CardEntity | Hero) => 'exhausted' in entity;

export const checkFollowUpRequirements = (requirements: FollowUpRequirement[], card: CardEntity) => {
	return requirements.every((r) => {
		if (r.type === 'isRace') {
      if (!isMinion(card)) return false;
			return card.races.some((race) => {
				return race === r.race;
			});
		}
		return true;
	});
};

export const coinFlip = (): boolean => Math.random() < 0.5; // The universe decides

export function validateTurn(playerID: string, gameState: GameStateServer): boolean {
	return (
		(playerID === gameState.whitePlayerID && gameState.whiteTurn) ||
		(playerID === gameState.blackPlayerID && !gameState.whiteTurn)
	);
}

export const getSourceBoard = (gameState: GameStateServer) => {
	return gameState.whiteTurn ? gameState.white : gameState.black;
};

export const getEnemyBoard = (gameState: GameStateServer) => {
	return gameState.whiteTurn ? gameState.black : gameState.white;
};

export const switchTurn = (gameState: GameStateServer): boolean =>
	(gameState.whiteTurn = !gameState.whiteTurn);

/**
 * Transforms server game state into client-specific perspective
 * @param {GameStateServer} gameStateServer - Complete server game state
 * @param {boolean} parsingForWhite - Whether its parsing the view for white player
 * @returns {GameStateClient} - Client-specific game state view
 */
export function parseGameStateToGameStateClient(
	gameStateServer: GameStateServer,
	parsingForWhite: boolean
): GameStateClient {
	return {
		self: parsingForWhite ? gameStateServer.white : gameStateServer.black,
		enemy: parsingForWhite ? gameStateServer.black : gameStateServer.white,
		whitePlayerID: gameStateServer.whitePlayerID,
		blackPlayerID: gameStateServer.blackPlayerID,
		yourTurn: parsingForWhite === gameStateServer.whiteTurn,
		turnCount: gameStateServer.turnCount,
		cardsPlayedThisTurn: gameStateServer.cardsPlayedThisTurn
	};
}

/**
 * Broadcasts game state to both players with their respective perspectives
 * @param {GameStateServer} newGameState - The complete server-side game state
 * @param {Object} activeGame - The active game session
 * @param {boolean} activeGame.isPlayer1White - Whether Player 1 is white
 * @param {Socket} activeGame.socket1 - Player 1's socket connection
 * @param {Socket} activeGame.socket2 - Player 2's socket connection
 * @returns {void} - Client-specific game state view
 */
export const broadcastGameState = (
	newGameState: GameStateServer,
	activeGame: {
		isPlayer1White: boolean;
		socket1: Socket;
		socket2: Socket;
		botGame: boolean;
	}
): void => {
	// Create player-specific views of the game state
	const clientGameState1 = parseGameStateToGameStateClient(
		newGameState,
		activeGame.isPlayer1White === true
	);

	const clientGameState2 = parseGameStateToGameStateClient(
		newGameState,
		activeGame.isPlayer1White === false
	);

	// in a bot game socket1 === socket2, only send player's perspective
	if (activeGame.botGame) {
		activeGame.socket1.emit('newGameState', clientGameState1);
		return;
	}

	// Send to respective players
	activeGame.socket1.emit('newGameState', clientGameState1);
	activeGame.socket2.emit('newGameState', clientGameState2);
};
