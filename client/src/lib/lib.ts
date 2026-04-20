import { browser } from '$app/environment';
import type { CardEntity } from './shared/bindings/CardEntity';
import type { Effect } from './shared/bindings/Effect';
import type { GameStateClient } from './shared/bindings/GameStateClient';
import type { GameStateServer } from './shared/bindings/GameStateServer';
import type { IncantationCard } from './shared/bindings/IncantationCard';
import type { IncantationEntity } from './shared/bindings/IncantationEntity';
import type { MinionCard } from './shared/bindings/MinionCard';
import type { MinionEntity } from './shared/bindings/MinionEntity';
import type { Requirement } from './shared/bindings/Requirement';
import type { TargetFilter } from './shared/bindings/TargetFilter';
import type { TargetSpec } from './shared/bindings/TargetSpec';

export const enterFullscreen = (divID: string) => {
	if (!browser) return;
	const elem = document.getElementById(divID);
	if (elem?.requestFullscreen) {
		elem.requestFullscreen();
	}
};

export const getRandomDeckName = () => {
	const names = [
		"Pixie's Rejected Tarot",
		'Rider-Waite Smackdown',
		'Moonlight Malpractice',
		'DMT Elf encounter',
		'Demonology for Dummies'
	];

	return names[Math.floor(Math.random() * names.length)];
};

export const getEntity = (card: CardEntity): MinionEntity | IncantationEntity => {
	if ('Minion' in card) return card.Minion;
	return (card as { Incantation: IncantationEntity }).Incantation;
};

export const isMinion = (
	card: CardEntity | MinionEntity | IncantationEntity
): card is { Minion: MinionEntity } | MinionEntity => {
	return 'Minion' in card || 'attack' in card;
};

const getTargetSpec = (effect: Effect): TargetSpec | null => {
	if ('Buff' in effect) return effect.Buff.target_spec;
	if ('Damage' in effect) return effect.Damage.target_spec;
	if ('ReturnToHand' in effect) return effect.ReturnToHand.target_spec;
	if ('Destroy' in effect) return effect.Destroy.target_spec;
	if ('Draw' in effect) return null;

	const _exhaustive: never = effect;
	return _exhaustive;
};

export const checkRequirements = (
	requirements: Requirement[] | undefined,
	gameState: GameStateServer | GameStateClient,
	cardEntity: MinionEntity | IncantationEntity
): boolean => {
	if (!requirements) return true; // no reqs = always fires
	return requirements.every((r) => {
		if (r === 'Combo') return gameState.cards_played_this_turn > 0;
		if (r === 'Quickdraw') return cardEntity.just_drawn;
	});
};

export const matchesFilters = (minion: MinionEntity, filters: TargetFilter[]): boolean => {
	return filters.every(f => {
		if ('IsRace' in f) return minion.card.races.some(r => r === f.IsRace.race);
		if ('HasAttribute' in f) return minion.card.attributes.some(a => a === f.HasAttribute.attribute);
		return true;
	});
};

export const getActiveTargetSpec = (card: MinionEntity | IncantationEntity, gameState: GameStateClient): TargetSpec | null => {
	for (const ability of card.card.abilities) {
		if (!abilityWillFire(ability, card, gameState)) continue;
		for (const effect of ability.effects) {
			const spec = getTargetSpec(effect as Effect);
			if (spec?.target_mode === 'Targeted') return spec;
		}
	}
	return null;
};

export const isTradeable = (card: MinionEntity | IncantationEntity | MinionCard | IncantationCard) => {
		const attributes = 'card' in card ? card.card.attributes : card.attributes;
		return attributes.some((a) => a === 'Tradeable');
};

const abilityWillFire = (a: { trigger: string; requirements: unknown[] }, card: MinionEntity | IncantationEntity, gameState: GameStateClient) =>
	checkRequirements(a.requirements as never, gameState, card);

export const needsTarget = (card: MinionEntity | IncantationEntity, gameState: GameStateClient) => {
	return card.card.abilities.some((a) =>
		abilityWillFire(a, card, gameState) &&
		a.effects.some((e) => {
			const spec = getTargetSpec(e as Effect);
			return spec?.target_mode === 'Targeted';
		})
	);
};

export const hasNoValidTarget = (
	card: MinionEntity | IncantationEntity,
	gameState: GameStateClient
): boolean => {
	const activeAbility = (a: { trigger: string; requirements: unknown[] }) =>
		a.trigger === 'OnPlay' && abilityWillFire(a, card, gameState);

	for (const ability of card.card.abilities) {
		if (!activeAbility(ability)) continue;
		for (const effect of ability.effects) {
			const spec = getTargetSpec(effect as Effect);
			if (!spec || spec.target_mode !== 'Targeted') continue;

			const self = gameState.self_board.battlefield.filter(m => matchesFilters(m, spec.filters));
			const enemy = gameState.enemy_board.battlefield.filter(m => matchesFilters(m, spec.filters));

			if (spec.entity_type === 'Minion') {
				if (spec.side === 'Friendly' && self.length === 0) return true;
				if (spec.side === 'Enemy' && enemy.length === 0) return true;
				if (spec.side === 'All' && self.length === 0 && enemy.length === 0) return true;
			}
		}
	}
	return false;
};

export const isSpellAndHasNoValidTarget = (
	card: MinionEntity | IncantationEntity,
	gameState: GameStateClient
): boolean => {
	if (isMinion(card)) return false;
	return hasNoValidTarget(card, gameState);
};
