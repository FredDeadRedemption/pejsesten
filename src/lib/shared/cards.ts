import type { Card, CardEntity } from './types';

export const getCardByID = (id: number) => cards.find((card) => card.id === id);

export const getCards = () => cards;

export const deckToCards = (deck: number[]): CardEntity[] => deck.map((id) => getCardByID(id)!).map(
	(card) => {
		if (card.type === 'minion') {
			return {
				...card,
				attack: card.baseAttack,
				defence: card.baseDefence,
				const: card.baseCost,
				exhausted: false
			} as CardEntity;
		} else {
			return {
				...card,
				cost: card.baseCost
			} as CardEntity;
		}
	}
);

export const cards: Card[] = [
	// ── WHITE MINIONS ────────────────────────────────────────
	{
		id: 1,
		color: 'white',
		name: 'Sunforge Paladin',
		description: 'Blessed by the light at birth.',
		baseCost: 2,
		type: 'minion',
		race: 'human',
		baseAttack: 2,
		baseDefence: 3,
		image_url: ''
	},
	{
		id: 2,
		color: 'white',
		name: 'Dawn Priest',
		description: 'Heals with one hand, smites with the other.',
		baseCost: 3,
		type: 'minion',
		race: 'human',
		baseAttack: 2,
		baseDefence: 4,
		image_url: ''
	},
	{
		id: 3,
		color: 'white',
		name: 'Silverguard Knight',
		description: 'His armour has never been tarnished.',
		baseCost: 4,
		type: 'minion',
		race: 'human',
		baseAttack: 3,
		baseDefence: 5,
		image_url: ''
	},
	{
		id: 4,
		color: 'white',
		name: 'Radiant Sentinel',
		description: 'Light pours from every crack in her armour.',
		baseCost: 5,
		type: 'minion',
		race: 'elf',
		baseAttack: 4,
		baseDefence: 6,
		image_url: ''
	},
	{
		id: 5,
		color: 'white',
		name: 'Herald of the Sun',
		description: 'Arrives before the dawn. Leaves after the victory.',
		baseCost: 6,
		type: 'minion',
		race: 'elf',
		baseAttack: 6,
		baseDefence: 5,
		image_url: ''
	},

	// ── BLACK MINIONS ────────────────────────────────────────
	{
		id: 6,
		color: 'black',
		name: 'Duskblade Rogue',
		description: 'Strikes when the candles go out.',
		baseCost: 2,
		type: 'minion',
		race: 'human',
		baseAttack: 3,
		baseDefence: 2,
		image_url: ''
	},
	{
		id: 7,
		color: 'black',
		name: 'Cursed Hexblade',
		description: 'Every wound he deals festers.',
		baseCost: 3,
		type: 'minion',
		race: 'human',
		baseAttack: 4,
		baseDefence: 2,
		image_url: ''
	},
	{
		id: 8,
		color: 'black',
		name: 'Void Stalker',
		description: 'Hunts in the space between thoughts.',
		baseCost: 4,
		type: 'minion',
		race: 'elf',
		baseAttack: 5,
		baseDefence: 3,
		image_url: ''
	},
	{
		id: 9,
		color: 'black',
		name: 'Grave Warden',
		description: 'Chosen to guard what should stay buried.',
		baseCost: 4,
		type: 'minion',
		race: 'human',
		baseAttack: 3,
		baseDefence: 5,
		image_url: ''
	},
	{
		id: 10,
		color: 'black',
		name: 'Soulreaper',
		description: 'Collects what death is owed.',
		baseCost: 6,
		type: 'minion',
		race: 'elf',
		baseAttack: 7,
		baseDefence: 4,
		image_url: ''
	},

	// ── WHITE INCANTATIONS ───────────────────────────────────
	{
		id: 11,
		color: 'white',
		name: 'Divine Shield',
		description: 'Wraps an ally in impenetrable holy light.',
		baseCost: 2,
		type: 'incantation',
		image_url: ''
	},
	{
		id: 12,
		color: 'white',
		name: 'Holy Wrath',
		description: 'Calls down judgement on the unworthy.',
		baseCost: 3,
		type: 'incantation',
		image_url: ''
	},

	// ── BLACK INCANTATIONS ───────────────────────────────────
	{
		id: 13,
		color: 'black',
		name: 'Dark Blessing',
		description: 'Power freely given always has a price.',
		baseCost: 2,
		type: 'incantation',
		image_url: ''
	},
	{
		id: 14,
		color: 'black',
		name: 'Soul Drain',
		description: 'Rips the life force from a target.',
		baseCost: 3,
		type: 'incantation',
		image_url: ''
	}
];
