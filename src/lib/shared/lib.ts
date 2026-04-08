import type { Board, GameStateClient, GameStateServer, Requirement } from "./types";

export const checkRequirement = (
  rec: Requirement | undefined,
  _sourceBoard: Board,
  _enemyBoard: Board,
  gameState: GameStateServer | GameStateClient
): boolean => {
  if (!rec) return true; // no rec = always fires
  if (rec.type === 'combo') return gameState.cardsPlayedThisTurn > 0;
  if (rec.type === 'firstCard') return gameState.cardsPlayedThisTurn === 0;
  return true;
};