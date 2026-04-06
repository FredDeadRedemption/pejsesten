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
import { STARTING_HAND_SIZE } from './settings';

let gameState: GameStateServer;

export type GameStateResponse = GameStateServer | null;

export const getGameState = (): GameStateServer => gameState;

type QueuedEffect = {
	effect: Effect;
	targetID?: string;
	sourceBoard: Board;
	enemyBoard: Board;
};

const findEntity = (id: string): MinionEntity | undefined =>
  [...gameState.white.battlefield, ...gameState.black.battlefield]
    .find(e => e.entityID === id);

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
	activeBoard: Board,
	enemyBoard: Board
): MinionEntity[] => {
	let pool: MinionEntity[] = [];
	if (spec.side === 'friendly' || spec.side === 'all') pool.push(...activeBoard.battlefield);
	if (spec.side === 'enemy' || spec.side === 'all') pool.push(...enemyBoard.battlefield);
	// hero targeting handled separately
	return spec.scope === 'single' ? pool.slice(0, 1) : pool;
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
	let startingHandWhite = whiteDeck.shuffle().draw(STARTING_HAND_SIZE);
	let startingHandBlack = blackDeck.shuffle().draw(STARTING_HAND_SIZE + 1);

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
	data: { index: number; target?: string }
): GameStateResponse => {
	console.log('Playing Card in hand index: ' + data.index);

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
		console.log("playing incantation target is: ", data.target)
		card.abilities.forEach((ability) => {
			if (ability.trigger !== 'onPlay') return;
			ability.effects.forEach((effect) => {
				effectQueue.push({
					effect,
					sourceBoard,
					enemyBoard,
					targetID: effect.targetSpec.scope === 'single' ? data.target : undefined
				});
			});
		});
		processEffectQueue();
		checkForDeaths();
	}

	return gameState;
};

export const attack = (_socketID: string, attackData: AttackData): GameStateResponse => {
	
	const attacker = findEntity(attackData.originID)
	if (!attacker) return null;

	const target = findEntity(attackData.targetID);
	if (!target) return null;

	if (attacker.exhausted) return null;
	attacker.exhausted = true;

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

		checkForDeaths();
	}

	console.log('FROM: ' + attackData.originID);
	console.log('TO: ' + attackData.targetID);

	return gameState;
};
