import type { Board } from "./bindings/Board";
import type { GameStateClient } from "./bindings/GameStateClient";
import type { GameStateServer } from "./bindings/GameStateServer";
import type { IncantationCard } from "./bindings/IncantationCard";
import type { IncantationEntity } from "./bindings/IncantationEntity";
import type { MinionCard } from "./bindings/MinionCard";
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
		if (r === 'Quickdraw') return cardEntity.just_drawn;
	});
};

export const isTradeable = (card: MinionEntity | IncantationEntity | MinionCard | IncantationCard) => {
    const attributes = 'card' in card ? card.card.attributes : card.attributes;
    return attributes.some((a) => a === 'Tradeable');
};
