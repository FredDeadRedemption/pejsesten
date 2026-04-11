import type { Card, CardEntity, IncantationEntity, MinionEntity } from './types';

const getCardByID = (id: number) => cards.find((card) => card.id === id);

export const getCards = () => cards;

// Transforms deck ids from static cards into cards entities with state variables
// Transform Deck: [1, 2, 3] to (actual)Deck: [CardEntity, CardEntity, CardEntity]
export const deckToCards = (deck: number[]): CardEntity[] =>
	deck
		.map((id) => getCardByID(id)!)
		.map((card) => {
			if (card.type === 'minion') {
				return {
					...card,
					entityID: crypto.randomUUID(),
					attack: card.baseAttack,
					defence: card.baseDefence,
					cost: card.baseCost,
					exhausted: false
				} as MinionEntity;
			} else {
				return {
					...card,
					entityID: crypto.randomUUID(),
					cost: card.baseCost
				} as IncantationEntity;
			}
		});
		
// =============================================================================
// CARD WRITING GUIDE
// =============================================================================
//
// Every card is a plain object matching the Card type (MinionCard | IncantationCard).
// Cards live in the cards array at the bottom of this file.
// After adding a card, add its id to a deck in your test client to try it out.
//
// =============================================================================
// CARD SKELETON
// =============================================================================
//
// MINION:
// {
//   id: <unique number>,
//   color: 'white' | 'black',
//   name: 'Card Name',
//   description: 'Human-readable description of what it does',
//   baseCost: <number>,
//   type: 'minion',
//   races: ['human' | 'elf' | 'beast'],   // can be multiple
//   baseAttack: <number>,
//   baseDefence: <number>,
//   image_url: 'filename.webp',            // or '' if no image yet
//   attributes: [],                        // see ATTRIBUTES section
//   abilities: [],                         // see ABILITIES section
// }
//
// INCANTATION (spell):
// {
//   id: <unique number>,
//   color: 'white' | 'black',
//   name: 'Card Name',
//   description: 'Human-readable description',
//   baseCost: <number>,
//   type: 'incantation',
//   image_url: 'filename.webp',
//   abilities: [],
// }
//
// =============================================================================
// ATTRIBUTES  (minions only)
// =============================================================================
//
// attributes: ['charge']
//
//   charge  — minion can attack the same turn it is played (not exhausted on summon)
//             display as <strong>Blitz</strong> in description
//
// =============================================================================
// ABILITIES
// =============================================================================
//
// An ability is an object with:
//   trigger      — WHEN it fires (see TRIGGERS)
//   requirements — optional array of conditions that must ALL be true for the
//                  ability to fire (see REQUIREMENTS)
//   effects      — array of things that happen when the ability fires (see EFFECTS)
//
// {
//   trigger: 'onPlay',
//   requirements: [{ type: 'combo' }],   // optional
//   effects: [ ... ]
// }
//
// A card can have MULTIPLE abilities with the SAME trigger. They are processed
// in order. This is how combo cards work — one ability always fires, a second
// ability only fires when the combo requirement is met.
//
// Example — Pot of Greed (draw 1, combo: draw 2 total):
//   abilities: [
//     { trigger: 'onPlay', effects: [{ type: 'draw', drawAmount: 1 }] },
//     { trigger: 'onPlay', requirements: [{ type: 'combo' }], effects: [{ type: 'draw', drawAmount: 1 }] }
//   ]
// The base ability always draws 1. The combo ability draws 1 more on top = 2 total.
//
// =============================================================================
// TRIGGERS
// =============================================================================
//
// 'onPlay'   — fires when the card is played from hand. Use this for Fanfare effects.
//              ⚠ incantations are consumed after firing — they don't stay on board
//
// 'onDeath'  — fires when a minion's defence reaches 0. Use this for Last Breath effects.
//              the minion is still on the board when its effects fire, then removed.
//
// NOT YET IMPLEMENTED (defined in types for future use):
//   onTurnStart, onTurnEnd, onAttack, onAttacked, onDamage, onHeal, onSummon, onDiscard, onDraw
//
// =============================================================================
// REQUIREMENTS
// =============================================================================
//
// requirements is an optional array. ALL requirements must pass for the ability to fire.
// If requirements is omitted (or empty), the ability always fires.
//
// { type: 'combo' }      — at least one other card was played this turn before this one
//                          display as <strong>Combo:</strong> in description
//
// { type: 'firstCard' }  — this is the first card played this turn
//
// =============================================================================
// EFFECTS
// =============================================================================
//
// ── buff ──────────────────────────────────────────────────────────────────────
// Adds attack and defence to targets.
//
// {
//   type: 'buff',
//   attack: 2,
//   defence: 3,
//   targetSpec: { scope: 'single', side: 'friendly', entityType: 'minion' }
// }
//
// ── damage ────────────────────────────────────────────────────────────────────
// Deals damage (reduces defence) to targets.
//
// {
//   type: 'damage',
//   damage: 4,
//   targetSpec: { scope: 'single', side: 'all', entityType: 'all' }
// }
//
// ── draw ──────────────────────────────────────────────────────────────────────
// Draws cards from the active player's deck into their hand. No targetSpec needed.
//
// { type: 'draw', drawAmount: 2 }
//
// ── returnToHand ──────────────────────────────────────────────────────────────
// Returns a minion from the battlefield to its owner's hand.
// Resets its attack, defence, and exhausted state.
// costReduction is optional — reduces the card's cost when returned.
//
// {
//   type: 'returnToHand',
//   costReduction: 2,    // optional
//   targetSpec: { scope: 'single', side: 'friendly', entityType: 'minion' }
// }
//
// =============================================================================
// TARGETSPEC — THE MOST IMPORTANT THING TO GET RIGHT
// =============================================================================
//
// targetSpec has three fields: scope, side, entityType
//
// ── scope ─────────────────────────────────────────────────────────────────────
//
//   ⚠ scope does NOT mean "how many targets". It means "who picks the target".
//
//   'single' — the PLAYER must click a target before the card can be played.
//               the engine validates their choice against side and entityType.
//               use this when there is a real choice (e.g. "deal 4 damage to any target")
//
//   'all'    — the ENGINE resolves targets automatically from the pool defined by
//               side and entityType. No player input required.
//               use this when there is no real choice (e.g. "deal 2 damage to ALL minions",
//               OR "deal 5 damage to the enemy hero" — only one possible target)
//
//   RULE OF THUMB: if there is only ever one possible target (your hero, enemy hero,
//   all minions, all friendlies), always use 'all'. Only use 'single' when the player
//   genuinely needs to choose between multiple options.
//
// ── side ──────────────────────────────────────────────────────────────────────
//
//   'friendly' — targets on the active player's side
//   'enemy'    — targets on the opponent's side
//   'all'      — targets on both sides
//
// ── entityType ────────────────────────────────────────────────────────────────
//
//   'minion'   — only minions on the battlefield
//   'hero'     — only the hero
//   'all'      — both minions and hero
//
// ── EXAMPLES ──────────────────────────────────────────────────────────────────
//
//   "Deal 4 damage to any target" (player chooses):
//   { scope: 'single', side: 'all', entityType: 'all' }
//
//   "Deal 2 damage to ALL minions" (no choice):
//   { scope: 'all', side: 'all', entityType: 'minion' }
//
//   "Deal 5 damage to the enemy hero" (no choice, only one target):
//   { scope: 'all', side: 'enemy', entityType: 'hero' }      ← use 'all' NOT 'single'!
//
//   "Give a friendly minion +3 +3" (player chooses which one):
//   { scope: 'single', side: 'friendly', entityType: 'minion' }
//
//   "Give ALL friendly minions +1 +1" (no choice):
//   { scope: 'all', side: 'friendly', entityType: 'minion' }
//
//   "Return a friendly minion to hand" (player chooses):
//   { scope: 'single', side: 'friendly', entityType: 'minion' }
//
//   "Return ALL minions to hand" (no choice):
//   { scope: 'all', side: 'all', entityType: 'minion' }
//
// =============================================================================
// DESCRIPTION FORMATTING CONVENTIONS
// =============================================================================
//
//   <strong>Fanfare:</strong>     — onPlay ability (always fires)
//   <strong>Deathwish:</strong> — onDeath ability
//   <strong>Combo:</strong>       — ability with requirements: [{ type: 'combo' }]
//   <strong>Blitz</strong>        — charge attribute
//
// =============================================================================
// COMMON MISTAKES
// =============================================================================
//
//   ✗ Using scope: 'single' for "deal damage to the enemy hero"
//     → There's only one enemy hero, no choice needed. Use scope: 'all'.
//
//   ✗ Using scope: 'all' for "give a friendly minion +3 +3"
//     → Player needs to choose which one. Use scope: 'single'.
//
//   ✗ Forgetting that a minion with returnToHand and scope: 'single'
//     will be a no-op if played with no other friendly minions on board
//     (the source minion is excluded from its own returnToHand pool).
//     This is intentional — it just stays on the board.
//
//   ✗ Using the same id as an existing card — ids must be unique.
//
// =============================================================================

const cards: Card[] = [
	// ── WHITE MINIONS ────────────────────────────────────────
	{
		id: 1,
		color: 'white',
		name: 'Macine Elf',
		description: '<strong>Blitz</strong>',
		baseCost: 1,
		type: 'minion',
		races: ['elf'],
		baseAttack: 1,
		baseDefence: 2,
		image_url: 'Machine Elf.webp',
		abilities: [],
		attributes: ['charge']
	},
	{
		id: 2,
		color: 'white',
		name: 'Overzealous Priest',
		description: '<strong>Fanfare:</strong> Draw a card',
		baseCost: 3,
		type: 'minion',
		races: ['human'],
		baseAttack: 2,
		baseDefence: 1,
		image_url: '',
		abilities: [
			{
				trigger: 'onPlay',
				effects: [
					{
						type: 'draw',
						drawAmount: 1
					}
				]
			}
		],
		attributes: []
	},
	{
		id: 3,
		color: 'white',
		name: 'Silverguard Knight',
		description: '<strong>Deathwish:</strong> Draw a card',
		baseCost: 3,
		type: 'minion',
		races: ['human'],
		baseAttack: 5,
		baseDefence: 3,
		image_url: '',
		abilities: [
			{
				trigger: 'onDeath',
				effects: [
					{
						type: 'draw',
						drawAmount: 1
					}
				]
			}
		],
		attributes: []
	},
	{
		id: 4,
		color: 'white',
		name: 'Radiant Sentinel',
		description: '<strong>Fanfare:</strong> Deal 2 damage to all minions',
		baseCost: 4,
		type: 'minion',
		races: ['elf'],
		baseAttack: 4,
		baseDefence: 6,
		image_url: '',
		abilities: [
			{
				trigger: 'onPlay',
				effects: [
					{
						type: 'damage',
						damage: 2,
						targetSpec: {
							scope: 'all',
							side: 'all',
							entityType: 'minion'
						}
					}
				]
			}
		],
		attributes: []
	},
	{
		id: 5,
		color: 'white',
		name: 'Herald of the Sun',
		description: '<strong>Deathwish:</strong> Deal 5 damage to the enemy hero, draw a card',
		baseCost: 5,
		type: 'minion',
		races: ['elf'],
		baseAttack: 6,
		baseDefence: 5,
		image_url: '',
		abilities: [
			{
				trigger: 'onDeath',
				effects: [
					{
						type: 'damage',
						damage: 5,
						targetSpec: {
							scope: 'all',
							side: 'enemy',
							entityType: 'hero'
						}
					},
					{
						type: 'draw',
						drawAmount: 1
					}
				]
			}
		],
		attributes: []
	},

	// ── BLACK MINIONS ────────────────────────────────────────
	{
		id: 6,
		color: 'white',
		name: 'Black Cat',
		description:
			'<strong>Combo:</strong> Return a friendly minion from the battlefield to your hand',
		baseCost: 1,
		type: 'minion',
		races: ['beast'],
		baseAttack: 2,
		baseDefence: 1,
		image_url: 'Black Cat.webp',
		abilities: [
			{
				trigger: 'onPlay',
				requirements: [
					{
						type: 'combo'
					}
				],
				effects: [
					{
						type: 'returnToHand',
						targetSpec: {
							scope: 'single',
							side: 'friendly',
							entityType: 'minion'
						}
					}
				]
			}
		],
		attributes: []
	},
	{
		id: 7,
		color: 'white',
		name: 'Barry the Hexblade',
		description: '<strong>Fanfare:</strong> Give all minions +1 +1',
		baseCost: 2,
		type: 'minion',
		races: ['human'],
		baseAttack: 4,
		baseDefence: 2,
		image_url: '',
		abilities: [
			{
				trigger: 'onPlay',
				effects: [
					{
						type: 'buff',
						attack: 1,
						defence: 1,
						targetSpec: {
							scope: 'all',
							side: 'all',
							entityType: 'minion'
						}
					}
				]
			}
		],
		attributes: []
	},
	{
		id: 8,
		color: 'white',
		name: 'Void Stalker',
		description: '<strong>Fanfare:</strong> Deal 10 damage to your own hero',
		baseCost: 2,
		type: 'minion',
		races: ['elf'],
		baseAttack: 5,
		baseDefence: 5,
		image_url: '',
		abilities: [
			{
				trigger: 'onPlay',
				effects: [
					{
						type: 'damage',
						damage: 10,
						targetSpec: {
							scope: 'all', // scope all means that the player doesnt get to choose
							side: 'friendly',
							entityType: 'hero'
						}
					}
				]
			}
		],
		attributes: []
	},
	{
		id: 9,
		color: 'white',
		name: 'Grave Warden',
		description: '<strong>Deathwish:</strong> Return all minions to their owners hand',
		baseCost: 4,
		type: 'minion',
		races: ['human'],
		baseAttack: 5,
		baseDefence: 4,
		image_url: '',
		abilities: [
			{
				trigger: 'onDeath',
				effects: [
					{
						type: 'returnToHand',
						targetSpec: {
							scope: 'all',
							side: 'all',
							entityType: 'minion'
						}
					}
				]
			}
		],
		attributes: []
	},
	{
		id: 10,
		color: 'white',
		name: 'Admirable Minion',
		description: '<strong>Fanfare:</strong> Give a friendly minion +3 +3',
		baseCost: 6,
		type: 'minion',
		races: ['elf'],
		baseAttack: 5,
		baseDefence: 4,
		image_url: '',
		abilities: [
			{
				trigger: 'onPlay',
				effects: [
					{
						type: 'buff',
						attack: 3,
						defence: 3,
						targetSpec: {
							scope: 'single',
							side: 'friendly',
							entityType: 'minion'
						}
					}
				]
			}
		],
		attributes: []
	},

	// ── WHITE INCANTATIONS ───────────────────────────────────
	{
		id: 11,
		color: 'white',
		name: 'Divine Power',
		description: 'Give a friendly minion +4 +4',
		baseCost: 2,
		type: 'incantation',
		image_url: 'Divine Power.webp',
		abilities: [
			{
				trigger: 'onPlay',
				effects: [
					{
						type: 'buff',
						attack: 4,
						defence: 4,
						targetSpec: {
							scope: 'single',
							side: 'friendly',
							entityType: 'minion'
						}
					}
				]
			}
		]
	},
	{
		id: 12,
		color: 'white',
		name: 'Good Friday',
		description: 'Deal 4 damage',
		baseCost: 3,
		type: 'incantation',
		image_url: 'Good Friday.webp',
		abilities: [
			{
				trigger: 'onPlay',
				effects: [
					{
						type: 'damage',
						damage: 4,
						targetSpec: {
							scope: 'single',
							side: 'all',
							entityType: 'all'
						}
					}
				]
			}
		]
	},
	{
		id: 13,
		color: 'white',
		name: 'Pot of Greed',
		description: 'Draw 1 card <strong>Combo:</strong> Draw 2 cards instead',
		baseCost: 3,
		type: 'incantation',
		image_url: 'Pot of Greed.webp',
		abilities: [
			{
				trigger: 'onPlay',
				effects: [{ type: 'draw', drawAmount: 1 }]
			},
			{
				trigger: 'onPlay',
				requirements: [
					{
						type: 'combo'
					}
				], // only fires on combo
				effects: [{ type: 'draw', drawAmount: 1 }] // extra 1 on top = 2 total
			}
		]
	},

	// ── BLACK INCANTATIONS ───────────────────────────────────
	{
		id: 14,
		color: 'white',
		name: 'Blessing',
		description: 'Give a friendly minion +3 +2',
		baseCost: 2,
		type: 'incantation',
		image_url: '',
		abilities: [
			{
				trigger: 'onPlay',
				effects: [
					{
						type: 'buff',
						attack: 3,
						defence: 2,
						targetSpec: {
							scope: 'single',
							side: 'friendly',
							entityType: 'minion'
						}
					}
				]
			}
		]
	},
	{
		id: 15,
		color: 'white',
		name: 'Smite',
		description: 'Deal 2 damage <strong>Combo:</strong> Deal 4 damage instead.',
		baseCost: 2,
		type: 'incantation',
		image_url: 'Smite.webp',
		abilities: [
			{
				trigger: 'onPlay',
				effects: [
					{
						type: 'damage',
						damage: 2,
						targetSpec: {
							scope: 'single',
							side: 'all',
							entityType: 'all'
						}
					}
				]
			},
			{
				trigger: 'onPlay',
				requirements: [
					{
						type: 'combo'
					}
				], // only fires on combo
				effects: [
					{
						type: 'damage',
						damage: 2,
						targetSpec: {
							scope: 'single',
							side: 'all',
							entityType: 'all'
						}
					}
				] // extra 2 on top = 4 total
			}
		]
	},
	{
		id: 16,
		color: 'white',
		name: 'Pull',
		description: 'Return a friendly minions to your hand it costs (2) less.',
		baseCost: 0,
		type: 'incantation',
		image_url: 'Pull.webp',
		abilities: [
			{
				trigger: 'onPlay',
				effects: [
					{
						type: 'returnToHand',
						costReduction: 2,
						targetSpec: {
							scope: 'single',
							side: 'friendly',
							entityType: 'minion'
						}
					}
				]
			}
		]
	},
	{
		id: 17,
		color: 'white',
		name: 'Destruction',
		description: 'Destroy a minion',
		baseCost: 5,
		type: 'incantation',
		image_url: '',
		abilities: [
			{
				trigger: 'onPlay',
				effects: [
					{
						type: "destroy",
						targetSpec: {
							scope: "single",
							side: "all",
							entityType: "minion"
						}
					}
				]
			}
		]
	}
];
