import { getEnemyBoard, getSourceBoard, switchTurn } from '$lib/server/lib';
import type {
	Ability,
	AttackData,
	Board,
	CardEntity,
	Effect,
	GameStateServer,
	Hero,
	MinionEntity,
	TargetSpec,
	Trigger
} from '$lib/shared/types';
import { STARTING_HAND_SIZE, STARTING_HP } from './settings';

let gameState: GameStateServer;

export type GameStateResponse = GameStateServer | null;

export const getGameState = (): GameStateServer => gameState;

// QueuedEffect. Stores refrence to the sourceBoard where the effect came from
type QueuedEffect = {
	effect: Effect;
	targetID?: string;
	sourceBoard: Board;
	enemyBoard: Board;
};

const findEntity = (id: string): MinionEntity | Hero | undefined => {
	if (id === 'heroEnemy') return getEnemyBoard(gameState).hero;
	if (id === 'heroSelf') return getSourceBoard(gameState).hero;
	return [...gameState.white.battlefield, ...gameState.black.battlefield].find(
		(e) => e.entityID === id
	);
};

let effectQueue: QueuedEffect[] = [];

// enqueueTrigger scans the active player's battlefield
// for any minion that has an ability matching the given trigger,
// and pushes all those effects onto the queue.
// Then processEffectQueue consumes them.
const enqueueTrigger = (trigger: Trigger) => {
	const sourceBoard = getSourceBoard(gameState);
	const enemyBoard = getEnemyBoard(gameState);

	sourceBoard.battlefield.forEach((minion) => {
		minion.abilities.forEach((ability) => {
			if (ability.trigger !== trigger) return;
			ability.effects.forEach((effect) => {
				effectQueue.push({ effect, sourceBoard, enemyBoard });
			});
		});
	});
};

//
const checkForDeaths = () => {
	const sourceBoard = getSourceBoard(gameState);
	const enemyBoard = getEnemyBoard(gameState);

	[sourceBoard, enemyBoard].forEach((board) => {
		board.battlefield = board.battlefield.filter((minion) => {
			if (minion.defence <= 0) {
				board.graveyard.push(minion);
				return false;
			}
			return true;
		});
	});
};

// Consume and apply one effect at a time
const processEffectQueue = () => {
	while (effectQueue.length > 0) {
		const queued = effectQueue.shift()!;
		const target = queued.targetID ? findEntity(queued.targetID) : undefined;
		applyEffect(queued.effect, queued.sourceBoard, queued.enemyBoard, target);
	}
};

const resolveTargets = (
	spec: TargetSpec,
	sourceBoard: Board,
	enemyBoard: Board
): (MinionEntity | Hero)[] => {
	let pool: (MinionEntity | Hero)[] = [];

	if (spec.entityType === 'minion' || spec.entityType === 'all') {
		if (spec.side === 'friendly' || spec.side === 'all') pool.push(...sourceBoard.battlefield);
		if (spec.side === 'enemy' || spec.side === 'all') pool.push(...enemyBoard.battlefield);
	}

	if (spec.entityType === 'hero' || spec.entityType === 'all') {
		if (spec.side === 'friendly' || spec.side === 'all') pool.push(sourceBoard.hero);
		if (spec.side === 'enemy' || spec.side === 'all') pool.push(enemyBoard.hero);
	}

	// note: for 'single' scope effects, a target is always pre-selected by the player
	// so resolveTargets is only ever called for 'all' scope effects.
	// the pool.slice(0, 1) is a safety fallback that should never be hit in practice.
	return spec.scope === 'single' ? pool.slice(0, 1) : pool;
};

const applyEffect = (
	effect: Effect,
	sourceBoard: Board,
	enemyBoard: Board,
	target?: MinionEntity | Hero
) => {
	if (effect.type === 'buff') {
		const targets = target ? [target] : resolveTargets(effect.targetSpec, sourceBoard, enemyBoard);
		targets.forEach((t) => {
			t.attack += effect.attack;
			t.defence += effect.defence;
		});
	}
	if (effect.type === 'damage') {
		const targets = target ? [target] : resolveTargets(effect.targetSpec, sourceBoard, enemyBoard);
		targets.forEach((t) => {
			t.defence -= effect.damage;
		});
	}
	if (effect.type === 'draw') {
		sourceBoard.hand.push(...sourceBoard.deck.draw(effect.drawAmount));
	}
};

export const setGameState = (
	player1ID: string,
	player2ID: string,
	isPlayer1White: boolean,
	player1Deck: CardEntity[],
	player2Deck: CardEntity[]
) => {
	// Assign decks
	const whiteDeck = isPlayer1White ? player1Deck : player2Deck;
	const blackDeck = isPlayer1White ? player2Deck : player1Deck;

	// Draw 5 random cards from each deck
	let startingHandWhite = whiteDeck.shuffle().draw(STARTING_HAND_SIZE);
	let startingHandBlack = blackDeck.shuffle().draw(STARTING_HAND_SIZE + 1);

	gameState = {
		white: {
			deck: whiteDeck,
			hand: startingHandWhite,
			graveyard: [],
			battlefield: [],
			hero: {
				attack: 0,
				defence: STARTING_HP
			},
			mana: 1
		},
		black: {
			deck: blackDeck,
			hand: startingHandBlack,
			graveyard: [],
			battlefield: [],
			hero: {
				attack: 0,
				defence: STARTING_HP
			},
			mana: 1
		},
		whitePlayerID: isPlayer1White ? player1ID : player2ID,
		blackPlayerID: isPlayer1White ? player2ID : player1ID,
		whiteTurn: true,
		turnCount: 0
	};
	console.log(`Game started! First turn: ${gameState.whitePlayerID}`);
	return gameState;
};

export const endTurn = (): GameStateResponse => {
	gameState.turnCount++;
	switchTurn(gameState); // flip first

	// getSourceBoard returns the NEW active player
	const sourceBoard = getSourceBoard(gameState);

	// draw for the new active player
	sourceBoard.hand.push(...sourceBoard.deck.draw(1));

	// unexhaust the new active player's minions
	sourceBoard.battlefield.forEach((card) => {
		if (card.type === 'minion') card.exhausted = false;
	});

	enqueueTrigger('onDraw');
	processEffectQueue();

	return gameState;
};

export const playCard = (
	_socketID: string,
	data: { index: number; target?: string }
): GameStateResponse => {
	const sourceBoard = getSourceBoard(gameState);
	const enemyBoard = getEnemyBoard(gameState);

	const card = sourceBoard.hand[data.index]; // peek first, don't splice yet
	if (!card) return null;

	if (card.type === 'incantation') {
		// if card has effect with targetSpec that requires a
		// single target and there is no target, return
		const target = data.target ? findEntity(data.target) : undefined;
		const needsValidation = card.abilities.some((a) =>
			a.effects.some((e) => 'targetSpec' in e && e.targetSpec.scope === 'single')
		);
		if (needsValidation && !target) return null;

		if (target) {
			const isMinion = 'exhausted' in target;
			const isOnFriendlyBoard =
				sourceBoard.battlefield.some((m) => m.entityID === (target as MinionEntity).entityID) ||
				sourceBoard.hero === target;

			// validate the target against every effect's targetSpec —
			// if ANY effect disagrees with the chosen target, the whole play is invalid
			const invalidTarget = card.abilities.some((a) =>
				a.effects.some((e) => {
					if (!('targetSpec' in e)) return false;
					if (e.targetSpec.entityType === 'minion' && !isMinion) return true; // spell wants minion, got hero
					if (e.targetSpec.entityType === 'hero' && isMinion) return true; // spell wants hero, got minion
					if (e.targetSpec.side === 'friendly' && !isOnFriendlyBoard) return true; // spell wants friendly, got enemy
					if (e.targetSpec.side === 'enemy' && isOnFriendlyBoard) return true; // spell wants enemy, got friendly
					return false;
				})
			);
			if (invalidTarget) return null;
		}
	}

	// validation passed, now consume the card
	const [consumed] = sourceBoard.hand.splice(data.index, 1);

	if (consumed.type === 'minion') {
		consumed.exhausted = true;
		sourceBoard.battlefield.push(consumed);
	}

	consumed.abilities.forEach((ability) => {
		if (ability.trigger !== 'onPlay') return;
		ability.effects.forEach((effect) => {
			effectQueue.push({
				effect,
				sourceBoard,
				enemyBoard,
				targetID:
					'targetSpec' in effect && effect.targetSpec.scope === 'single' ? data.target : undefined
			});
		});
	});
	processEffectQueue();
	checkForDeaths();

	return gameState;
};

export const attack = (_socketID: string, attackData: AttackData): GameStateResponse => {
	const attacker = findEntity(attackData.originID);

	// invalid input || not a minion || has attacked / just spawned
	if (!attacker || !('exhausted' in attacker) || attacker.exhausted) return null;
	attacker.exhausted = true;

	const target = findEntity(attackData.targetID);
	if (!target) return null;

	target.defence -= attacker.attack;

	// only retaliate if target is a minion
	if ('exhausted' in target) {
		attacker.defence -= target.attack;
	}

	checkForDeaths();

	return gameState;
};
