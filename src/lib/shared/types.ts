type CardBase = {
  id: number
  name: string
  description: string | null
  cost: number
  image_url: string
}

// Subtypes
export type CreatureCard = CardBase & {
  type: "creature"
  attack: number
  defence: number
  race: "human" | "elf" | "dwarf" | "undead" | "dragon" | "beast" | "orc"
}

export type SpellCard = CardBase & {
  type: "spell"
}

export type IncantationCard = CardBase & {
  type: "incantation"
}

export type Card = CreatureCard | SpellCard | IncantationCard

export type Board = {
  deck: Card[], 
  hand: Card[], 
  graveyard: Card[], 
  battlefield: CreatureCard[],
  hp: number,
  mana: number,
}

type state = "draw" | "play" | "attack" | "inactive";

export type GameState = {
  white: Board,
  black: Board,
  whitePlayerID: string; // used for validation turn
  blackPlayerID: string; // used for validation turn
  whiteTurn: boolean, // used for processing the actual game 
  state: state,
  turnCount: number,
}

export type GameStateClient = {
  self: Board,
  enemy: Board,
  whitePlayerID: string; // used for validation turn
  blackPlayerID: string; // used for validation turn
  yourTurn: boolean, // used for processing the actual game 
  state: state, // used for processing the actual game
  stateEnemy: state,
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