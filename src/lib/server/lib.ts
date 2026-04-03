import type { GameStateClient, GameState } from "$lib/shared/types";
import type { Socket } from "socket.io-client";

/**
 * Simulates a coin flip (50/50 chance)
 * @returns {boolean} True for heads, false for tails
 */
export const coinFlip = (): boolean => Math.random() < 0.5; // The universe decides

/**
 * Returns a random number from the given array of numbers (card IDs).
 * If the array is empty, returns `null` instead.
 * 
 * @param {number[]} arr - Array of numbers (e.g., card IDs).
 * @returns {number | null} - Random number from the array, or `null` if the array is empty.
 */
export const getRandomValue = (arr: number[]): number | null => arr[Math.floor(Math.random() * arr.length)] ?? null;

/**
 * Validates if it's the specified player's turn
 * @param {string} playerID - The socket ID of the player attempting to make a move
 * @param {GameState} gameState - Current game state
 * @returns {boolean} - True if it's the player's turn, false otherwise
 */
export function validateTurn(playerID: string, gameState: GameState): boolean {
  return (playerID === gameState.whitePlayerID && gameState.whiteTurn ||
          playerID === gameState.blackPlayerID && !gameState.whiteTurn)
}

/**
 * Switches the turn between white and black players
 * @param {GameState} gameState - Current game state (will be mutated)
 * @returns {boolean} - The new turn state (true for white's turn, false for black's)
 */
export const switchTurn = (gameState: GameState): boolean => gameState.whiteTurn = !gameState.whiteTurn;

/**
 * Transforms server game state into client-specific perspective
 * @param {GameState} gameStateServer - Complete server game state
 * @param {boolean} parsingForWhite - Whether its parsing the view for white player
 * @returns {GameStateClient} - Client-specific game state view
 */
export function parseGameStateToGameStateClient(
  gameStateServer: GameState,
  parsingForWhite: boolean,
): GameStateClient {
  return {
    self: parsingForWhite? gameStateServer.white : gameStateServer.black,
    enemy: parsingForWhite? gameStateServer.black : gameStateServer.white,
    whitePlayerID: gameStateServer.whitePlayerID,
    blackPlayerID: gameStateServer.blackPlayerID,
    yourTurn: parsingForWhite === gameStateServer.whiteTurn,
    state: parsingForWhite === gameStateServer.whiteTurn ? gameStateServer.state : "inactive",
    stateEnemy: parsingForWhite === gameStateServer.whiteTurn ? "inactive" : gameStateServer.state,  
    turnCount: gameStateServer.turnCount,
  };
}

/**
 * Broadcasts game state to both players with their respective perspectives
 * @param {GameState} newGameState - The complete server-side game state
 * @param {Object} activeGame - The active game session
 * @param {boolean} activeGame.isPlayer1White - Whether Player 1 is white
 * @param {Socket} activeGame.socket1 - Player 1's socket connection
 * @param {Socket} activeGame.socket2 - Player 2's socket connection
 * @returns {void} - Client-specific game state view
 */
export const broadcastGameState = (
  newGameState: GameState,
  activeGame: {
    isPlayer1White: boolean;
    socket1: Socket;
    socket2: Socket;
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

  // Send to respective players
  activeGame.socket1.emit('newGameState', clientGameState1);
  activeGame.socket2.emit('newGameState', clientGameState2);
}