import { browser } from '$app/environment';
import type { CardEntity, GameStateClient } from './shared/types';

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

export const isSpellAndHasNoValidTarget = (card: CardEntity, gameState: GameStateClient): boolean => {
    if (card.type === 'minion') return false;

    const needsFriendlyMinion = card.abilities.some(
        (a) =>
            a.trigger === 'onPlay' &&
            a.effects.some(
                (e) =>
                    'targetSpec' in e &&
                    e.targetSpec.scope === 'single' &&
                    e.targetSpec.side === 'friendly' &&
                    e.targetSpec.entityType === 'minion'
            )
    );
    const needsEnemyMinion = card.abilities.some(
        (a) =>
            a.trigger === 'onPlay' &&
            a.effects.some(
                (e) =>
                    'targetSpec' in e &&
                    e.targetSpec.scope === 'single' &&
                    e.targetSpec.side === 'enemy' &&
                    e.targetSpec.entityType === 'minion'
            )
    );

    if (needsFriendlyMinion && gameState.self.battlefield.length === 0) return true;
    if (needsEnemyMinion && gameState.enemy.battlefield.length === 0) return true;
    return false;
};
