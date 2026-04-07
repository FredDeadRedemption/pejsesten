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

export type Condition =
	| { type: 'targetRace'; race: Race }
	| { type: 'heroHealthBelow'; heroSide: 'self' | 'enemy'; value: number }
	| {
			type: 'boardSize';
			side: 'friendly' | 'enemy';
			comparison: 'more' | 'less' | 'equal';
			value: number;
	  };

export type Proc =
	| { type: 'combo' } // played a card before this one this turn
	| { type: 'firstCard' }; // first card played this turn

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
	  }
	| {
      readonly type: 'returnToHand';
      readonly targetSpec: TargetSpec;
      readonly costReduction?: number;
    };

export type Ability = {
	trigger: Trigger;
	conditions: Condition[];
	proc?: Proc;
	effects: Effect[];
};

type Attributes = 'charge'; // can attack the turn it is played

// Base Card
type CardBase = {
	id: number; // also functions as runtime entity ID
	readonly name: string;
	readonly description: string | null;
	readonly color: Color;
	readonly baseCost: number;
	readonly image_url: string;
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

// Minion Entity
export type MinionEntity = MinionCard & {
	entityID: string;
	attack: number;
	defence: number;
	cost: number;
	exhausted: boolean;
};

// Encantation Entity
export type IncantationEntity = IncantationCard & {
	entityID: string;
	cost: number;
};

// !!! IF ADDING REMOVING ANYTHING HERE REMEMBER TO ALSO
// REWORK THE deckToCards FUNCTION IN cards.ts !!!
// Card Entity
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
