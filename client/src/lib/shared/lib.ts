import type { Board } from "./bindings/Board";
import type { Card } from "./bindings/Card";
import type { CardEntity } from "./bindings/CardEntity";
import type { GameStateClient } from "./bindings/GameStateClient";
import type { GameStateServer } from "./bindings/GameStateServer";
import type { IncantationEntity } from "./bindings/IncantationEntity";
import type { MinionEntity } from "./bindings/MinionEntity";
import type { Requirement } from "./bindings/Requirement";


export const checkRequirements = (
	requirements: Requirement[] | undefined,
	_sourceBoard: Board,
	_enemyBoard: Board,
	gameState: GameStateServer | GameStateClient,
	cardEntity: MinionEntity | IncantationEntity
): boolean => {
	if (!requirements) return true; // no reqs = always fires
	return requirements.every((r) => {
		if (r === 'Combo') return gameState.cards_played_this_turn > 0;
		if (r === 'Quickdraw') return cardEntity.base.just_drawn;
	});
};

export const isTradeable = (card: CardEntity | Card) =>
	card?.attributes?.some((a) => a === 'tradeable');
