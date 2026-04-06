import { getEnemyBoard, getSourceBoard, switchTurn } from '$lib/server/lib';
import type {
	Ability,
	AttackData,
	Board,
	CardEntity,
	Effect,
	GameStateServer,
	MinionEntity,
	TargetSpec,
	Trigger
} from '$lib/shared/types';

let gameState: GameStateServer;

export type GameStateResponse = GameStateServer | null;

export const getGameState = (): GameStateServer => gameState;

type QueuedEffect = {
	effect: Effect;
	target?: number;
	sourceBoard: Board;
	enemyBoard: Board;
};

let effectQueue: QueuedEffect[] = [];

// Instead of processing immediately, enqueue
const enqueueTrigger = (trigger: Trigger) => {
	const sourceBoard = gameState.whiteTurn ? gameState.white : gameState.black;
	const enemyBoard = gameState.whiteTurn ? gameState.black : gameState.white;

	sourceBoard.battlefield.forEach((minion) => {
		minion.abilities.forEach((ability) => {
			if (ability.trigger !== trigger) return;
			ability.effects.forEach((effect) => {
				effectQueue.push({ effect, sourceBoard, enemyBoard });
			});
		});
	});
};

// Consume and apply one effect at a time
const processEffectQueue = () => {
	while (effectQueue.length > 0) {
		const queued = effectQueue.shift()!;
		const target =
			queued.target !== undefined ? queued.enemyBoard.battlefield[queued.target] : undefined;
		applyEffect(queued.effect, queued.sourceBoard, queued.enemyBoard, target);
	}
};

const resolveTargets = (
	spec: TargetSpec,
	activeBoard: Board,
	enemyBoard: Board
): MinionEntity[] => {
	let pool: MinionEntity[] = [];
	if (spec.side === 'friendly' || spec.side === 'all') pool.push(...activeBoard.battlefield);
	if (spec.side === 'enemy' || spec.side === 'all') pool.push(...enemyBoard.battlefield);
	// hero targeting handled separately
	return spec.scope === 'single' ? [pool[0]] : pool; // single needs UI targeting still
};

const applyEffect = (
	effect: Effect,
	activeBoard: Board,
	enemyBoard: Board,
	target?: MinionEntity
) => {
	if (effect.type === 'buff') {
		const targets = target ? [target] : resolveTargets(effect.targetSpec, activeBoard, enemyBoard);
		targets.forEach((t) => {
			t.attack += effect.attack;
			t.defence += effect.defence;
		});
	}
	if (effect.type === 'damage') {
		const targets = target ? [target] : resolveTargets(effect.targetSpec, activeBoard, enemyBoard);
		targets.forEach((t) => {
			t.defence -= effect.damage;
		});
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
	let startingHandWhite = whiteDeck.shuffle().draw(3);
	let startingHandBlack = blackDeck.shuffle().draw(4);

	gameState = {
		white: {
			deck: whiteDeck,
			hand: startingHandWhite,
			graveyard: [],
			battlefield: [],
			hp: 30,
			mana: 1
		},
		black: {
			deck: blackDeck,
			hand: startingHandBlack,
			graveyard: [],
			battlefield: [],
			hp: 30,
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

	switchTurn(gameState);

	// draw a card at the start of the turn
	gameState.whiteTurn
		? gameState.white.hand.push(...gameState.white.deck.draw(1))
		: gameState.black.hand.push(...gameState.black.deck.draw(1));

	// unexhaust minions at the start of the turn
	const battlefield = gameState.whiteTurn
		? gameState.white.battlefield
		: gameState.black.battlefield;
	battlefield.forEach((card) => {
		if (card.type === 'minion') card.exhausted = false; // unexhaust minions at the start of the turn
	});

	enqueueTrigger('onDraw');
	processEffectQueue();

	return gameState;
};

export const playCard = (
	_socketID: string,
	data: { index: number; target?: number }
): GameStateResponse => {
	console.log('PLAYING CARD IN HAND: ' + data.index);

	const sourceBoard = getSourceBoard(gameState);
	const enemyBoard = getEnemyBoard(gameState);

	const [card] = sourceBoard.hand.splice(data.index, 1);

	if (card === null) {
		console.log('Trying to play NULL card!');
		return null;
	}

	if (card.type === 'minion') {
		card.exhausted = true; // minions enter the battlefield exhausted
		sourceBoard.battlefield.push(card);
	}

	if (card.type === 'incantation') {
		card.abilities.forEach((ability) => {
			if (ability.trigger !== 'onPlay') return;
			ability.effects.forEach((effect) => {
				effectQueue.push({
					effect,
					sourceBoard,
					enemyBoard,
					target: effect.targetSpec.scope === 'single' ? data.target : undefined
				});
			});
		});
		processEffectQueue();
	}

	return gameState;
};

export const attack = (_socketID: string, attackData: AttackData): GameStateResponse => {
	const sourceBoard = getSourceBoard(gameState);
	const enemyBoard = getEnemyBoard(gameState);

	const attacker = sourceBoard.battlefield[attackData.origin];

	if (attacker.exhausted) return null;
	attacker.exhausted = true;

	const target = enemyBoard.battlefield[attackData.target];

	if (attackData.face) {
		// early return if attacking face - noooooooooo ;_;

		console.log('ATTACKING CARD: ' + attacker.attack);

		// mutate directly as primitives are always passed by value not refrence
		gameState.whiteTurn
			? (gameState.black.hp -= attacker.attack)
			: (gameState.white.hp -= attacker.attack);
	} else {
		target.defence -= attacker.attack;
		attacker.defence -= target.attack;

		// check if attacked card died, if so move to graveyard
		if (target.defence <= 0) {
			const [deadCard] = enemyBoard.battlefield.splice(attackData.target, 1);

			enemyBoard.graveyard.push(deadCard);
		}
		if (attacker.defence <= 0) {
			const [deadCard] = sourceBoard.battlefield.splice(attackData.origin, 1);

			sourceBoard.graveyard.push(deadCard);
		}
	}

	console.log('FROM: ' + attackData.origin);
	console.log('TO: ' + attackData.target);

	return gameState;
};
