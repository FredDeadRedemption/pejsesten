import { switchTurn } from "$lib/server/lib";
import type { AttackData, GameState } from "$lib/shared/types";
import { activeGame } from "$lib/server/socket";
import { io } from "../../hooks.server";
import type { Card } from "$lib/shared/types";

let gameState: GameState;

type flags = {
    hasDrawn: boolean,
    exhaustedIndexes: number[],
}

export type GameStateResponse = GameState | null;

/*
* EXPORTED FUNCTIONS
*/

// FLAGS
const flags: flags = {
    hasDrawn: false,
    exhaustedIndexes: [],
}

const resetFlags = () => {
    flags.hasDrawn = false;
    flags.exhaustedIndexes = [];
};


export const getGameState = (): GameState => gameState;

export const setGameState = (player1ID: string, player2ID: string, isPlayer1White: boolean, player1Deck: Card[], player2Deck: Card[]) => {
    const shuffleDeck = (deck: Card[]) => {
        for (let i = deck.length - 1; i > 0; i--) {
            const j = Math.floor(Math.random() * (i + 1));
            [deck[i], deck[j]] = [deck[j], deck[i]];
        }
        return deck;
    };

    // Shuffle both decks
    const shuffledPlayer1Deck = shuffleDeck([...player1Deck]);
    const shuffledPlayer2Deck = shuffleDeck([...player2Deck]);

    const whiteDeck = isPlayer1White ? shuffledPlayer1Deck : shuffledPlayer2Deck;
    const blackDeck = isPlayer1White ? shuffledPlayer2Deck : shuffledPlayer1Deck;

    // Draw 5 random cards from each deck
    let startingHandWhite: Card[] = whiteDeck.splice(0, 5);
    let startingHandBlack: Card[] = blackDeck.splice(0, 5);

    const socket1 = io.sockets.sockets.get(player1ID);
    const socket2 = io.sockets.sockets.get(player2ID);
    if (socket1 && socket2) {
        socket1.emit("cardDrawn", isPlayer1White ? startingHandWhite[0] : startingHandBlack[0]);
        socket2.emit("cardDrawn", isPlayer1White ? startingHandBlack[0] : startingHandWhite[0]);
        socket1.emit("cardDrawn", isPlayer1White ? startingHandWhite[1] : startingHandBlack[1]);
        socket2.emit("cardDrawn", isPlayer1White ? startingHandBlack[1] : startingHandWhite[1]);
        socket1.emit("cardDrawn", isPlayer1White ? startingHandWhite[2] : startingHandBlack[2]);
        socket2.emit("cardDrawn", isPlayer1White ? startingHandBlack[2] : startingHandWhite[2]);
        socket1.emit("cardDrawn", isPlayer1White ? startingHandWhite[3] : startingHandBlack[3]);
        socket2.emit("cardDrawn", isPlayer1White ? startingHandBlack[3] : startingHandWhite[3]);
        socket1.emit("cardDrawn", isPlayer1White ? startingHandWhite[4] : startingHandBlack[4]);
        socket2.emit("cardDrawn", isPlayer1White ? startingHandBlack[4] : startingHandWhite[4]);
        socket1.emit("cardDrawn", isPlayer1White ? startingHandWhite[5] : startingHandBlack[5]);
        socket2.emit("cardDrawn", isPlayer1White ? startingHandBlack[5] : startingHandWhite[5]);
    } else {
        console.error(`Socket with ID ${player1ID} or ${player2ID} not found.`);
    }

    // write this to restore starting with 5 cards in hand
    // deck: whiteDeck, hand: startingHandWhite
    // deck: blackDeck, hand: startingHandBlack
    
    gameState = {
        white: { deck: whiteDeck, hand: startingHandWhite, graveyard: [], battlefield: [], hp: 30, mana: 1 },
        black: { deck: blackDeck, hand: startingHandBlack, graveyard: [], battlefield: [], hp: 30, mana: 1 },
        whitePlayerID: isPlayer1White ? player1ID : player2ID,
        blackPlayerID: isPlayer1White ? player2ID : player1ID,
        state: "draw",
        whiteTurn: true, 
        turnCount: 0,
    };
    resetFlags();
    console.log(`Game started! First turn: ${gameState.whitePlayerID}`);
    return gameState;
}


export const endTurn = (socketID: string): GameStateResponse => {

    if(gameState.state === "attack"){
        resetFlags(); // reset flags for the next turn
        gameState.turnCount++;
    
        
        const socket = io.sockets.sockets.get(socketID);
        // if (!socket) console.log("socket is null during emit mana")
        // socket && socket.emit("mana", manas);

        switchTurn(gameState)
    
        gameState.state = "draw"; // set state to draw phase

        return gameState;
    }

    if(gameState.state === "play") gameState.state = "attack";
    if(gameState.state === "draw") gameState.state = "play";

    return gameState;
} 

export const drawCard = (socketID: string): GameStateResponse => {

    console.log("DRAWING CARD: hasDRAWN: " + flags.hasDrawn)
    console.log("whiteTurn: " + gameState.whiteTurn)

    if(flags.hasDrawn) return null;
    flags.hasDrawn = true; // set flag to prevent drawing more than once per turn

    const deck = gameState.whiteTurn ? gameState.white.deck : gameState.black.deck;
    const hand = gameState.whiteTurn ? gameState.white.hand : gameState.black.hand;

    const randomIndex = Math.floor(Math.random() * deck.length);
    const [card] = deck.splice(randomIndex, 1);

    let succes: boolean = false;

    if (card) {
        succes = true; // set success to true if card was drawn
        hand.push(card);
    } else {
        // Handle empty deck (deal damage to player!!!)
        console.log("Deck is empty!");
    }

    if (succes) {
        const socket = io.sockets.sockets.get(socketID);
        if (socket) {
            socket.emit("cardDrawn", card);
        } else {
            console.error(`Socket with ID ${socketID} not found.`);
        }
    }

    return gameState
}

export const playCard = (socketID: string, index: number): GameStateResponse => {

    console.log("PLAYING CARD IN HAND: " + index)

    const hand = gameState.whiteTurn ? gameState.white.hand : gameState.black.hand;
    const battlefield = gameState.whiteTurn ? gameState.white.battlefield : gameState.black.battlefield;

    const [card] = hand.splice(index, 1);

    if (card === null) {
        console.log("Trying to play NULL card!") 
        return null
    }

    if(card.type === "creature"){

        battlefield.push(card);
    }


    const otherSocketID = gameState.whiteTurn ? gameState.blackPlayerID : gameState.whitePlayerID;
    const socket = io.sockets.sockets.get(otherSocketID);
    if (socket) {
        socket.emit("playCard", card);
    } else {
        console.error(`Socket with ID ${socketID} not found.`);
    }

    return gameState
}

export const attack = (socketID: string, attackData: AttackData): GameStateResponse => {

    if(flags.exhaustedIndexes.includes(attackData.origin)) return null; // early return if already attacked this turn
    flags.exhaustedIndexes.push(attackData.origin); // fill flags for the attacked card

    const originBattlefield = gameState.whiteTurn ? gameState.white.battlefield : gameState.black.battlefield;
    const targetBattlefield = gameState.whiteTurn ? gameState.black.battlefield : gameState.white.battlefield;
    const originGraveyard = gameState.whiteTurn ? gameState.white.graveyard : gameState.black.graveyard;
    const targetGraveyard = gameState.whiteTurn ? gameState.black.graveyard : gameState.white.graveyard;

    const attacker = originBattlefield[attackData.origin];
    const target = targetBattlefield[attackData.target];

    let targetDied: boolean = false;
    let attackerDied: boolean = false;

    let attackerAttack: number;
    let targetAttack: number;

    if(attackData.face){ // early return if attacking face - noooooooooo ;_;

        console.log("ATTACKING CARD: " + attacker.attack)

        // mutate directly as primitives are always passed by value not refrence
        gameState.whiteTurn ? gameState.black.hp -= attacker.attack : gameState.white.hp -= attacker.attack;
        
        targetAttack = 0;

        
    } else {

        target.defence -= attacker.attack; 
        attacker.defence -= target.attack;

        targetAttack = target.attack;

        // check if attacked card died, if so move to graveyard
        if(target.defence <= 0){
            const [deadCard] = targetBattlefield.splice(attackData.target, 1);

            targetDied = true; 

            targetGraveyard.push(deadCard);
        }
        if(attacker.defence <= 0){
            const [deadCard] = originBattlefield.splice(attackData.origin, 1);

            attackerDied = true; 

            originGraveyard.push(deadCard);
        }
    }

    const socket = io.sockets.sockets.get(socketID);
    if (socket) {
        attackerAttack = attacker.attack;
        
        activeGame?.socket1.emit("attacked", { targetAttack, attackerAttack, targetDied, attackerDied, attackData });
        activeGame?.socket2.emit("attacked", { targetAttack, attackerAttack, targetDied, attackerDied, attackData });
        
    } else {
        console.error(`Socket with ID ${socketID} not found.`);
    }

    
    console.log("FROM: " + attackData.origin);
    console.log("TO: " + attackData.target);

    return gameState
}



