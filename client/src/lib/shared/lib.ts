import type { Board } from "./bindings/Board";
import type { GameStateClient } from "./bindings/GameStateClient";
import type { Requirement } from "./bindings/Requirement";


export const checkRequirements = (
	requirements: Requirement[] | undefined,
	_sourceBoard: Board,
	_enemyBoard: Board,
	gameState: GameStateServer | GameStateClient,
	card: CardEntity
): boolean => {
	if (!requirements) return true; // no reqs = always fires
	return requirements.every((r) => {
		if (r.type === 'combo') return gameState.cardsPlayedThisTurn > 0;
		if (r.type === 'quickdraw') return card.justDrawn === true;
	});
};

export const isTradeable = (card: CardEntity | Card) =>
	card?.attributes?.some((a) => a === 'tradeable');
