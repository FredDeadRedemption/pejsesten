import type { Card, CardEntity, IncantationEntity, MinionEntity } from './types';

const getCardByID = (id: number) => cards.find((card) => card.id === id);

export const getCards = () => cards;

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

const cards: Card[] = [
	// ── WHITE MINIONS ────────────────────────────────────────
	{
		id: 1,
		color: 'white',
		name: 'Macine Elf',
		description:
			'<strong>Blitz</strong>. Appears in your peripheral vision. Gone when you look directly.',
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
				conditions: [],
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
		description: '<strong>Last breath:</strong> Draw a card',
		baseCost: 3,
		type: 'minion',
		races: ['human'],
		baseAttack: 5,
		baseDefence: 3,
		image_url: '',
		abilities: [
			{
				trigger: "onDeath",
				conditions: [],
				effects: [
					{
						type: "draw",
						drawAmount: 1,
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
		description: 'Light pours from every crack in her armour. Doctors are concerned.',
		baseCost: 4,
		type: 'minion',
		races: ['elf'],
		baseAttack: 4,
		baseDefence: 6,
		image_url: '',
		abilities: [],
		attributes: []
	},
	{
		id: 5,
		color: 'white',
		name: 'Herald of the Sun',
		description: 'Arrives before the dawn. Leaves before doing the dishes.',
		baseCost: 5,
		type: 'minion',
		races: ['elf'],
		baseAttack: 6,
		baseDefence: 5,
		image_url: '',
		abilities: [],
		attributes: []
	},

	// ── BLACK MINIONS ────────────────────────────────────────
	{
		id: 6,
		color: 'black',
		name: 'Black Cat',
		description:
			'<strong>Blitz</strong>. <strong>Fanfare:</strong> Return a friendly minion from the battlefield to your hand',
		baseCost: 1,
		type: 'minion',
		races: ['beast'],
		baseAttack: 2,
		baseDefence: 1,
		image_url: 'Black Cat.webp',
		abilities: [
			{
				trigger: 'onPlay',
				conditions: [],
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
		attributes: ['charge']
	},
	{
		id: 7,
		color: 'black',
		name: 'Barry the Hexblade',
		description: 'Every wound he deals festers. His name does not.',
		baseCost: 2,
		type: 'minion',
		races: ['human'],
		baseAttack: 4,
		baseDefence: 2,
		image_url: '',
		abilities: [],
		attributes: []
	},
	{
		id: 8,
		color: 'black',
		name: 'Void Stalker',
		description: 'Hunts in the space between thoughts. Probably behind you right now.',
		baseCost: 3,
		type: 'minion',
		races: ['elf'],
		baseAttack: 3,
		baseDefence: 5,
		image_url: '',
		abilities: [],
		attributes: []
	},
	{
		id: 9,
		color: 'black',
		name: 'Grave Warden',
		description: 'He was told to guard the grave. He did not ask whose.',
		baseCost: 4,
		type: 'minion',
		races: ['human'],
		baseAttack: 5,
		baseDefence: 4,
		image_url: '',
		abilities: [],
		attributes: []
	},
	{
		id: 10,
		color: 'black',
		name: 'Soulreaper',
		description: "Collecting souls since 1987. It's been a good year.",
		baseCost: 6,
		type: 'minion',
		races: ['elf'],
		baseAttack: 7,
		baseDefence: 6,
		image_url: '',
		abilities: [],
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
				conditions: [],
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
				conditions: [],
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
				conditions: [],
				effects: [{ type: 'draw', drawAmount: 1 }]
			},
			{
				trigger: 'onPlay',
				conditions: [],
				proc: { type: 'combo' }, // only fires on combo
				effects: [{ type: 'draw', drawAmount: 1 }] // extra 1 on top = 2 total
			}
		]
	},

	// ── BLACK INCANTATIONS ───────────────────────────────────
	{
		id: 14,
		color: 'black',
		name: 'Dark Blessing',
		description: 'Give a friendly minion +3 +2',
		baseCost: 2,
		type: 'incantation',
		image_url: '',
		abilities: [
			{
				trigger: 'onPlay',
				conditions: [],
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
		color: 'black',
		name: 'Smite',
		description: 'Deal 2 damage <strong>Combo:</strong> Deal 4 damage instead.',
		baseCost: 2,
		type: 'incantation',
		image_url: 'Smite.webp',
		abilities: [
			{
				trigger: 'onPlay',
				conditions: [],
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
				conditions: [],
				proc: { type: 'combo' }, // only fires on combo
				effects: [
					{
						type: 'damage',
						damage: 2,
						targetSpec: {
							scope: 'single',
							side: 'enemy',
							entityType: 'all'
						}
					}
				] // extra 2 on top = 4 total
			}
		]
	},
	{
		id: 16,
		color: 'black',
		name: 'Pull',
		description: 'Return a friendly minions to your hand it costs (2) less.',
		baseCost: 0,
		type: 'incantation',
		image_url: 'Pull.webp',
		abilities: [
			{
				trigger: 'onPlay',
				conditions: [],
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
	}
];
