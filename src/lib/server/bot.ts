import type { GameStateServer, MinionEntity } from '$lib/shared/types';
import { attack, endTurn, playCard } from './engine';
import { BOT_DELAY_MS } from './settings';

// ── Evaluation helpers ────────────────────────────────────────────────────────

// How threatening is a minion? Higher = more dangerous to leave alive
const threatScore = (minion: MinionEntity): number => minion.attack * 2 + minion.defence;

// How much total damage can a set of minions deal?
const totalAttack = (minions: MinionEntity[]): number =>
	minions.reduce((sum, m) => sum + m.attack, 0);

// Would attacking this minion result in our minion dying?
const isTradeLethal = (attacker: MinionEntity, defender: MinionEntity): boolean =>
	defender.attack >= attacker.defence;

// Would we kill the defender?
const wouldKill = (attacker: MinionEntity, defender: MinionEntity): boolean =>
	attacker.attack >= defender.defence;

// Can we kill the enemy hero this turn with available minions?
const canGoLethal = (minions: MinionEntity[], enemyHeroDefence: number): MinionEntity[] | null => {
	const attackers = minions.filter((m) => !m.exhausted);
	const dmg = totalAttack(attackers);
	return dmg >= enemyHeroDefence ? attackers : null;
};

// Score a potential attack trade: higher = better trade for us
// Favors: killing their minion, not dying, killing high-threat targets
const tradeScore = (attacker: MinionEntity, defender: MinionEntity): number => {
	let score = 0;
	if (wouldKill(attacker, defender)) score += threatScore(defender) * 3;
	if (isTradeLethal(attacker, defender)) score -= threatScore(attacker) * 2;
	// bonus for killing without dying (favourable trade)
	if (wouldKill(attacker, defender) && !isTradeLethal(attacker, defender)) score += 10;
	return score;
};

// Pick the best target on the enemy board for a given attacker
const bestTarget = (attacker: MinionEntity, enemies: MinionEntity[]): MinionEntity | null => {
	if (enemies.length === 0) return null;
	return enemies.reduce((best, candidate) =>
		tradeScore(attacker, candidate) > tradeScore(attacker, best) ? candidate : best
	);
};

// Evaluate the board state for the bot. Higher = better for bot.
const evaluateBoard = (
	botMinions: MinionEntity[],
	enemyMinions: MinionEntity[],
	botHeroHp: number,
	enemyHeroHp: number
): number => {
	const botStrength = botMinions.reduce((s, m) => s + threatScore(m), 0);
	const enemyStrength = enemyMinions.reduce((s, m) => s + threatScore(m), 0);
	return botStrength - enemyStrength + (botHeroHp - enemyHeroHp) * 2;
};

// ── Target selection for single-target spells ─────────────────────────────────

const pickBestSpellTarget = (
	gameState: GameStateServer,
	botIsWhite: boolean,
	effectType: 'buff' | 'damage' | 'returnToHand',
	side: 'friendly' | 'enemy' | 'all',
	entityType: 'minion' | 'hero' | 'all'
): string | undefined => {
	const botBoard = botIsWhite ? gameState.white : gameState.black;
	const enemyBoard = botIsWhite ? gameState.black : gameState.white;

	const friendlyMinions = botBoard.battlefield;
	const enemyMinions = enemyBoard.battlefield;

	if (effectType === 'damage') {
		// prioritize killing a minion, then highest threat, then hero
		if ((side === 'enemy' || side === 'all') && enemyMinions.length > 0) {
			// prefer a minion we can kill
			const killable = enemyMinions
				.filter((m) => m.defence <= 4) // rough damage estimate
				.sort((a, b) => threatScore(b) - threatScore(a));
			if (killable.length > 0) return killable[0]!.entityID;
			// otherwise highest threat
			const highest = [...enemyMinions].sort((a, b) => threatScore(b) - threatScore(a));
			return highest[0]!.entityID;
		}
		if ((side === 'enemy' || side === 'all') && entityType === 'hero') {
			return 'heroEnemy';
		}
	}

	if (effectType === 'buff') {
		// buff the highest attack friendly minion
		if (friendlyMinions.length > 0) {
			const best = [...friendlyMinions].sort((a, b) => b.attack - a.attack);
			return best[0]!.entityID;
		}
	}

	if (effectType === 'returnToHand') {
		// return the weakest friendly minion (least tempo loss)
		if (friendlyMinions.length > 0) {
			const weakest = [...friendlyMinions].sort((a, b) => threatScore(a) - threatScore(b));
			return weakest[0]!.entityID;
		}
	}

	return undefined;
};

// Resolve the best target for a card's single-target effects
const resolveCardTarget = (
	card: { abilities: GameStateServer['white']['hand'][0]['abilities'] },
	gameState: GameStateServer,
	botIsWhite: boolean
): string | undefined => {
	for (const ability of card.abilities) {
		if (ability.trigger !== 'onPlay') continue;
		for (const effect of ability.effects) {
			if (!('targetSpec' in effect)) continue;
			if (effect.targetSpec.scope !== 'single') continue;
			if (effect.type === 'buff' || effect.type === 'damage' || effect.type === 'returnToHand') {
				return pickBestSpellTarget(
					gameState,
					botIsWhite,
					effect.type,
					effect.targetSpec.side,
					effect.targetSpec.entityType
				);
			}
		}
	}
	return undefined;
};

// ── Card play ordering ────────────────────────────────────────────────────────

// Score a card for play priority. Higher = play sooner.
const cardPlayScore = (
	card: GameStateServer['white']['hand'][0],
	botBoard: GameStateServer['white'],
	enemyBoard: GameStateServer['black']
): number => {
	let score = 0;

	// prefer playing cheap cards first to enable combo
	score -= card.cost;

	for (const ability of card.abilities) {
		if (ability.trigger !== 'onPlay') continue;
		for (const effect of ability.effects) {
			if (effect.type === 'damage') {
				// high value if we can kill something or go face
				score += effect.damage * 2;
				if (enemyBoard.battlefield.length === 0) score += 5; // face damage
			}
			if (effect.type === 'buff') {
				score += (effect.attack + effect.defence) * 1.5;
				if (botBoard.battlefield.length === 0) score -= 5; // no targets
			}
			if (effect.type === 'draw') {
				score += effect.drawAmount * 3;
			}
			if (effect.type === 'returnToHand') {
				score += 2; // generally useful for combo setup
			}
		}
	}

	// minions with charge are high priority
	if (card.type === 'minion' && card.attributes.includes('charge')) score += 8;

	return score;
};

// ── Main bot logic ────────────────────────────────────────────────────────────
const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

export const makeBotMove = async (gameState: GameStateServer, botIsWhite: boolean) => {
	if (gameState.whiteTurn !== botIsWhite) {
		console.error('makeBotMove called but it is not the bot turn!');
		return null;
	}

	await sleep(BOT_DELAY_MS);

	const botBoard = botIsWhite ? gameState.white : gameState.black;
	const enemyBoard = botIsWhite ? gameState.black : gameState.white;

	// ── Phase 1: check for lethal before doing anything ──────────────────────
	const lethalAttackers = canGoLethal(botBoard.battlefield, enemyBoard.hero.defence);
	if (lethalAttackers) {
		lethalAttackers.forEach((m) => {
			attack('bot', { originID: m.entityID, targetID: 'heroEnemy' });
		});
		endTurn();
		return gameState;
	}

	// ── Phase 2: play cards in smart order ───────────────────────────────────
	let keepPlaying = true;
	while (keepPlaying) {
		// re-read board each iteration since playing a card mutates state
		const currentBotBoard = botIsWhite ? gameState.white : gameState.black;
		const currentEnemyBoard = botIsWhite ? gameState.black : gameState.white;

		const affordable = currentBotBoard.hand
			.map((card, index) => ({ card, index }))
			.filter(({ card }) => card.cost <= currentBotBoard.mana)
			.sort(
				(a, b) =>
					cardPlayScore(b.card, currentBotBoard, currentEnemyBoard) -
					cardPlayScore(a.card, currentBotBoard, currentEnemyBoard)
			);

		if (affordable.length === 0) {
			keepPlaying = false;
			break;
		}

		const { card, index } = affordable[0]!;
		const target = resolveCardTarget(card, gameState, botIsWhite);
		const result = playCard('bot', { index, target });

		// if playCard failed, stop trying to avoid infinite loop
		if (!result) {
			keepPlaying = false;
			break;
		}

		// check lethal after each card play
		const lethals = canGoLethal(
			(botIsWhite ? gameState.white : gameState.black).battlefield,
			(botIsWhite ? gameState.black : gameState.white).hero.defence
		);
		if (lethals) {
			lethals.forEach((m) => {
				attack('bot', { originID: m.entityID, targetID: 'heroEnemy' });
			});
			endTurn();
			return gameState;
		}
	}

	// ── Phase 3: attack with minions ─────────────────────────────────────────
	// re-read board after playing cards
	const finalBotBoard = botIsWhite ? gameState.white : gameState.black;
	const finalEnemyBoard = botIsWhite ? gameState.black : gameState.white;

	const attackers = finalBotBoard.battlefield.filter((m) => !m.exhausted);

	attackers.forEach((attacker) => {
		if (finalEnemyBoard.battlefield.length > 0) {
			const target = bestTarget(attacker, finalEnemyBoard.battlefield);
			if (!target) return;

			// only trade if it's a good trade, otherwise go face
			const score = tradeScore(attacker, target);
			if (score > 0) {
				attack('bot', { originID: attacker.entityID, targetID: target.entityID });
			} else {
				// bad trade — go face instead
				attack('bot', { originID: attacker.entityID, targetID: 'heroEnemy' });
			}
		} else {
			attack('bot', { originID: attacker.entityID, targetID: 'heroEnemy' });
		}
	});

	endTurn();
	return gameState;
};
