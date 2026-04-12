import type { Board, CardEntity, GameStateClient, GameStateServer, Requirement } from "./types";

export const checkRequirement = (
  rec: Requirement | undefined,
  _sourceBoard: Board,
  _enemyBoard: Board,
  gameState: GameStateServer | GameStateClient,
  card: CardEntity
): boolean => {
  if (!rec) return true; // no rec = always fires
  if (rec.type === 'combo') return gameState.cardsPlayedThisTurn > 0;
  if (rec.type === "quickdraw") return card.justDrawn === true;
  return true;
};