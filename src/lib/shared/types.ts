type Color = 'white' | 'black';

type Race = 'human' | 'elf' | 'beast';

export type Trigger =
	| 'onPlay' // when the card is played
	| 'onDeath' // when the card dies
	| 'onTurnStart' // at the start of the turn
	| 'onTurnEnd' // at the end of the turn
	| 'onAttack' // when the card attacks
	| 'onAttacked' // when the card is attacked
	| 'onDamage' // when the card takes damage
	| 'onHeal' // when the card is healed
	| 'onSummon' // when the card is summoned to the battlefield
	| 'onDiscard' // when the card is discarded from hand
	| 'onDraw'; // when the card is drawn from the deck

export type TargetSpec = {
	readonly scope: 'single' | 'all';
	readonly side: 'friendly' | 'enemy' | 'all';
	readonly entityType: 'minion' | 'hero' | 'all';
};

export type Requirement =
	| { type: 'combo' } // played a card before this one this turn
	| { type: 'quickdraw' } // played same turn as drawn
	| { type: "cardsInHand" };

export type FollowUpRequirement =
  | { type: 'isRace'; race: Race }

export type ScaledBy = 'minionsOnBoard';

export type ScaledAmount = {
  scalar: number;
  scaledBy?: ScaledBy;
};

export type FollowUp = {
	requirements: FollowUpRequirement[];
	type: 'discount';
	scaledAmount: ScaledAmount;
};

export type Effect =
	| {
			readonly type: 'buff';
			readonly targetSpec: TargetSpec;
			attack: number;
			defence: number;
	  }
	| {
			readonly type: 'damage';
			readonly targetSpec: TargetSpec;
			damage: number;
	  }
	| {
			readonly type: 'draw';
			drawAmount: number;
			followUp?: FollowUp;
	  }
	| {
			readonly type: 'returnToHand';
			readonly targetSpec: TargetSpec;
			readonly costReduction?: number;
	  }
	| {
			readonly type: 'destroy';
			readonly targetSpec: TargetSpec;
	  };

export type Ability = {
	trigger: Trigger;
	requirements?: Requirement[];
	effects: Effect[];
};

type Attributes = 'charge'; // can attack the turn it is played

// Base Card
type CardBase = {
	readonly id: number; // static identifier
	readonly name: string;
	readonly description: string | null;
	readonly color: Color;
	readonly baseCost: number;
	readonly image_url: string;
	readonly tradeable?: true;
	abilities: Ability[];
};

// Base Minion Card
export type MinionCard = CardBase & {
	readonly type: 'minion';
	readonly baseAttack: number;
	readonly baseDefence: number;
	readonly races: Race[];
	attributes: Attributes[];
};

// Base Incantation Card
export type IncantationCard = CardBase & {
	readonly type: 'incantation';
};

// Metadata mostly for ui
export type Card = MinionCard | IncantationCard;

// !!! IF ADDING REMOVING ANYTHING HERE REMEMBER TO ALSO
// REWORK THE deckToCards FUNCTION IN cards.ts !!!
// Base for all entities on the board/in-play
export type EntityBase = {
	entityID: string;
	cost: number;
	turnsInHand: number;
	justDrawn: boolean;
};

// !!! IF ADDING REMOVING ANYTHING HERE REMEMBER TO ALSO
// REWORK THE deckToCards FUNCTION IN cards.ts !!!
// Minion-specific runtime props (not on the card definition)
type MinionEntityProps = {
	attack: number;
	defence: number;
	exhausted: boolean;
};

// Card Entity
export type MinionEntity = MinionCard & EntityBase & MinionEntityProps;
export type IncantationEntity = IncantationCard & EntityBase;

export type CardEntity = MinionEntity | IncantationEntity;

export type Hero = {
	attack: number;
	defence: number;
};

export type Board = {
	deck: CardEntity[];
	hand: CardEntity[];
	graveyard: MinionEntity[];
	battlefield: MinionEntity[];
	hero: Hero;
	baseMana: number;
	mana: number;
};

export type GameStateServer = {
	white: Board;
	black: Board;
	whitePlayerID: string; // used for validation turn
	blackPlayerID: string; // used for validation turn
	whiteTurn: boolean; // used for processing the actual game
	turnCount: number;
	cardsPlayedThisTurn: number;
};

export type GameStateClient = {
	self: Board;
	enemy: Board;
	whitePlayerID: string; // used for validation turn
	blackPlayerID: string; // used for validation turn
	yourTurn: boolean; // used for processing the actual game
	turnCount: number;
	cardsPlayedThisTurn: number;
};

export type PlayerMetaData = {
	username: string;
	choosenDeck: number[];
	avatar: string;
};

export type AttackData = {
	originID: string;
	targetID: string | 'heroEnemy' | 'heroSelf';
};
