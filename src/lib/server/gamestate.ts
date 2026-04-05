import { switchTurn } from '$lib/server/lib';
import type { AttackData, CardEntity, GameStateServer } from '$lib/shared/types';

let gameState: GameStateServer;

export type GameStateResponse = GameStateServer | null;

export const getGameState = (): GameStateServer => gameState;

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
        card.exhausted = true; // minions enter the battlefield exhausted
		battlefield.push(card);
	}

	return gameState;
};

export const attack = (_socketID: string, attackData: AttackData): GameStateResponse => {
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

    if (attacker.exhausted) return null;
    attacker.exhausted = true;

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