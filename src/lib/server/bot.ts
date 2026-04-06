// src/lib/server/bot.ts

import type { GameStateServer } from '$lib/shared/types';
import { attack, endTurn, playCard } from './engine';

export const makeBotMove = (gameState: GameStateServer, botIsWhite: boolean) => {
  if (gameState.whiteTurn !== botIsWhite) {
    console.error('makeBotMove called but it is not the bot turn!');
    return null;
  }

  const botBoard = botIsWhite ? gameState.white : gameState.black;
  const enemyBoard = botIsWhite ? gameState.black : gameState.white;

  // play any card it can afford
  const playableCard = botBoard.hand.find((c) => c.cost <= botBoard.mana);
  if (playableCard) {
    const index = botBoard.hand.indexOf(playableCard);
    playCard('bot', { index });
  }

  // attack with every non-exhausted minion using entityID
  botBoard.battlefield.forEach((minion) => {
    if (minion.exhausted) return;
    if (enemyBoard.battlefield.length > 0) {
      const randomTarget = enemyBoard.battlefield[Math.floor(Math.random() * enemyBoard.battlefield.length)];
      attack('bot', { originID: minion.entityID, targetID: randomTarget.entityID });
    } else {
      attack('bot', { originID: minion.entityID, targetID: 'faceEnemy' });
    }
  });

  endTurn();
  return gameState;
};
