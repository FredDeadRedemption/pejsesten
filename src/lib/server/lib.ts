import type { GameStateClient, GameStateServer } from "$lib/shared/types";
import type { Socket } from "socket.io";

/**
 * Simulates a coin flip (50/50 chance)
 * @returns {boolean} True for heads, false for tails
 */
export const coinFlip = (): boolean => Math.random() < 0.5; // The universe decides

/**
 * Validates if it's the specified player's turn
 * @param {string} playerID - The socket ID of the player attempting to make a move
 * @param {GameStateServer} gameState - Current game state
 * @returns {boolean} - True if it's the player's turn, false otherwise
 */
export function validateTurn(playerID: string, gameState: GameStateServer): boolean {
  return (playerID === gameState.whitePlayerID && gameState.whiteTurn ||
          playerID === gameState.blackPlayerID && !gameState.whiteTurn)
}

export const getSourceBoard = (gameState: GameStateServer) => {
  return gameState.whiteTurn ? gameState.white : gameState.black;
}

export const getEnemyBoard = (gameState: GameStateServer) => {
  return gameState.whiteTurn ? gameState.black : gameState.white;
}

/**
 * Switches the turn between white and black players
 * @param {GameStateServer} gameState - Current game state (will be mutated)
 * @returns {boolean} - The new turn state (true for white's turn, false for black's)
 */
export const switchTurn = (gameState: GameStateServer): boolean => gameState.whiteTurn = !gameState.whiteTurn;

/**
 * Transforms server game state into client-specific perspective
 * @param {GameStateServer} gameStateServer - Complete server game state
 * @param {boolean} parsingForWhite - Whether its parsing the view for white player
 * @returns {GameStateClient} - Client-specific game state view
 */
export function parseGameStateToGameStateClient(
  gameStateServer: GameStateServer,
  parsingForWhite: boolean,
): GameStateClient {
  return {
    self: parsingForWhite? gameStateServer.white : gameStateServer.black,
    enemy: parsingForWhite? gameStateServer.black : gameStateServer.white,
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
}