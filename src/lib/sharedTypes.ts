// this is types that is shared with the backend only!!! :D
type Board = {
  deck: number[], 
  hand: number[], 
  graveyard: number[], // Mortem
  battlefield: number[], // Bellum
  land: number[], // Terra
}

export type GameStateServer = {
  white: Board,
  black: Board,
  whitePlayerID: string; // used for validation turn
  blackPlayerID: string; // used for validation turn
  whiteTurn: boolean, // used for processing the actual game 
  turnCount: number,
}

export type GameState = {
  self: Board,
  enemy: Board,
  whitePlayerID: string; // used for validation turn
  blackPlayerID: string; // used for validation turn
  whiteTurn: boolean, // used for processing the actual game 
  turnCount: number,
}

export type PlayerMetaData = {
  username: string,
  choosenDeck: number[],
  avatar: string,
}

export function parseGameStateServerToGameState(
  gameStateServer: GameStateServer,
  yourTurn: boolean
): GameState {
  return {
    self: yourTurn ? gameStateServer.white : gameStateServer.black,
    enemy: yourTurn ? gameStateServer.black : gameStateServer.white,
    whitePlayerID: gameStateServer.whitePlayerID,
    blackPlayerID: gameStateServer.blackPlayerID,
    whiteTurn: gameStateServer.whiteTurn,
    turnCount: gameStateServer.turnCount,
  };
}
