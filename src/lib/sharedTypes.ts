import type { Database } from '$lib/database.types'; 
type Card = Database['public']['Tables']['cards']['Row'];

// this is types that is shared with the backend only!!! :D
type Board = {
  deck: Card[], 
  hand: Card[], 
  graveyard: Card[], // Mortem
  battlefield: Card[], // Bellum
  land: Card[], // Terra
}

export type GameState = {
  self: Board,
  enemy: Board,
  whitePlayerID: string; // used for validation turn
  blackPlayerID: string; // used for validation turn
  selfHP: number,
  enemyHP: number,
  yourTurn: boolean, // used for processing the actual game 
  turnCount: number,
}

export type PlayerMetaData = {
  username: string,
  choosenDeck: number[],
  avatar: string,
}

export type AttackData = {
  origin: number,
  target: number,
  face: boolean
}
