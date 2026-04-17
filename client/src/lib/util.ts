import { browser } from '$app/environment';
import type { CardEntity } from './shared/bindings/CardEntity';
import type { Effect } from './shared/bindings/Effect';
import type { GameStateClient } from './shared/bindings/GameStateClient';
import type { IncantationEntity } from './shared/bindings/IncantationEntity';
import type { MinionEntity } from './shared/bindings/MinionEntity';
import type { TargetSpec } from './shared/bindings/TargetSpec';
import { checkRequirements } from './shared/lib';

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

	const needsFriendlyMinion = card.card.abilities.some(
		(a) =>
			activeAbility(a) &&
			a.effects.some((e) => {
				const spec = getTargetSpec(e as Effect);
				return spec?.target_mode === 'Targeted' && spec.side === 'Friendly' && spec.entity_type === 'Minion';
			})
	);
	const needsEnemyMinion = card.card.abilities.some(
		(a) =>
			activeAbility(a) &&
			a.effects.some((e) => {
				const spec = getTargetSpec(e as Effect);
				return spec?.target_mode === 'Targeted' && spec.side === 'Enemy' && spec.entity_type === 'Minion';
			})
	);
	const needsAnyMinion = card.card.abilities.some(
		(a) =>
			activeAbility(a) &&
			a.effects.some((e) => {
				const spec = getTargetSpec(e as Effect);
				return spec?.target_mode === 'Targeted' && spec.side === 'All' && spec.entity_type === 'Minion';
			})
	);

	if (needsFriendlyMinion && gameState.self_board.battlefield.length === 0) return true;
	if (needsEnemyMinion && gameState.enemy_board.battlefield.length === 0) return true;
	if (
		needsAnyMinion &&
		gameState.self_board.battlefield.length === 0 &&
		gameState.enemy_board.battlefield.length === 0
	)
		return true;
	return false;
};

export const isSpellAndHasNoValidTarget = (
	card: MinionEntity | IncantationEntity,
	gameState: GameStateClient
): boolean => {
	if (isMinion(card)) return false;
	return hasNoValidTarget(card, gameState);
};
