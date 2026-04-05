type Colors = "white" | "black"

// Base Card
type CardBase = {
  readonly id: number
  readonly name: string
  readonly description: string | null
  readonly color: Colors
  readonly baseCost: number
  readonly image_url: string
}

type Races = "human" | "elf"

// Base Minion Card
export type MinionCard = CardBase & {
  readonly type: "minion"
  readonly baseAttack: number
  readonly baseDefence: number
  readonly race: Races
}

// Base Incantation Card
export type IncantationCard = CardBase & {
  readonly type: "incantation"
}

// Metadata mostly for ui
export type Card = MinionCard | IncantationCard

// Minion Entity
export type MinionEntity = MinionCard & {
  attack: number
  defence: number
  cost: number
  exhausted: boolean
}

// Encantation Entity
export type IncantationEntity = IncantationCard & {
  cost: number
}

// Card Entity
export type CardEntity = MinionEntity | IncantationEntity

export type Board = {
  deck: CardEntity[], 
  hand: CardEntity[], 
  graveyard: MinionEntity[], 
  battlefield: MinionEntity[],
  hp: number,
  mana: number,
}

export type GameStateServer = {
  white: Board,
  black: Board,
  whitePlayerID: string; // used for validation turn
  blackPlayerID: string; // used for validation turn
  whiteTurn: boolean, // used for processing the actual game 
  turnCount: number,
}

export type GameStateClient = {
  self: Board,
  enemy: Board,
  whitePlayerID: string; // used for validation turn
  blackPlayerID: string; // used for validation turn
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