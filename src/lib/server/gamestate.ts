import { switchTurn } from '$lib/server/lib';
import type { AttackData, GameStateServer } from '$lib/shared/types';
import { activeGame } from '$lib/server/socket';
import { ioHandle as io } from '$lib/server/socket';
import type { Card } from '$lib/shared/types';

let gameState: GameStateServer;

type flags = {
	hasDrawn: boolean;
	exhaustedIndexes: number[];
};

export type GameStateResponse = GameStateServer | null;

/*
 * EXPORTED FUNCTIONS
 */

// FLAGS
const flags: flags = {
	hasDrawn: false,
	exhaustedIndexes: []
};

const resetFlags = () => {
	flags.hasDrawn = false;
	flags.exhaustedIndexes = [];
};

export const getGameState = (): GameStateServer => gameState;

export const setGameState = (
	player1ID: string,
	player2ID: string,
	isPlayer1White: boolean,
	player1Deck: Card[],
	player2Deck: Card[]
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
	resetFlags();
	console.log(`Game started! First turn: ${gameState.whitePlayerID}`);
	return gameState;
};

export const endTurn = (): GameStateResponse => {

	resetFlags(); // reset flags for the next turn
	gameState.turnCount++;

    switchTurn(gameState);

	gameState.whiteTurn
		? (gameState.white.hand.push(...gameState.white.deck.draw(1)))
		: (gameState.black.hand.push(...gameState.black.deck.draw(1)));

	return gameState;
};

export const playCard = (_socketID: string, index: number): GameStateResponse => {
	console.log('PLAYING CARD IN HAND: ' + index);

	const hand = gameState.whiteTurn ? gameState.white.hand : gameState.black.hand;
	const battlefield = gameState.whiteTurn
		? gameState.white.battlefield
		: gameState.black.battlefield;

	const [card] = hand.splice(index, 1);

	if (card === null) {
		console.log('Trying to play NULL card!');
		return null;
	}

	if (card.type === 'minion') {
		battlefield.push(card);
	}

	return gameState;
};

export const attack = (_socketID: string, attackData: AttackData): GameStateResponse => {
	if (flags.exhaustedIndexes.includes(attackData.origin)) return null; // early return if already attacked this turn
	flags.exhaustedIndexes.push(attackData.origin); // fill flags for the attacked card

	const originBattlefield = gameState.whiteTurn
		? gameState.white.battlefield
		: gameState.black.battlefield;
	const targetBattlefield = gameState.whiteTurn
		? gameState.black.battlefield
		: gameState.white.battlefield;
	const originGraveyard = gameState.whiteTurn
		? gameState.white.graveyard
		: gameState.black.graveyard;
	const targetGraveyard = gameState.whiteTurn
		? gameState.black.graveyard
		: gameState.white.graveyard;

	const attacker = originBattlefield[attackData.origin];
	const target = targetBattlefield[attackData.target];

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
			const [deadCard] = targetBattlefield.splice(attackData.target, 1);

			targetGraveyard.push(deadCard);
		}
		if (attacker.defence <= 0) {
			const [deadCard] = originBattlefield.splice(attackData.origin, 1);

			originGraveyard.push(deadCard);
		}
	}

	console.log('FROM: ' + attackData.origin);
	console.log('TO: ' + attackData.target);

	return gameState;
};
