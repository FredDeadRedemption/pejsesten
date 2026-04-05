import type { Card } from './types';

export const getCardByID = (id: number) => cards.find((card) => card.id === id);

export const getCards = () => cards;

export const deckToCards = (deck: number[]) => deck.map((id) => getCardByID(id)!);

export const cards: Card[] = [
	// ── WHITE MINIONS ────────────────────────────────────────
	{
		id: 1,
		color: 'white',
		name: 'Sunforge Paladin',
		description: 'Blessed by the light at birth.',
		cost: 2,
		type: 'minion',
		race: 'human',
		attack: 2,
		defence: 3,
		image_url: ''
	},
	{
		id: 2,
		color: 'white',
		name: 'Dawn Priest',
		description: 'Heals with one hand, smites with the other.',
		cost: 3,
		type: 'minion',
		race: 'human',
		attack: 2,
		defence: 4,
		image_url: ''
	},
	{
		id: 3,
		color: 'white',
		name: 'Silverguard Knight',
		description: 'His armour has never been tarnished.',
		cost: 4,
		type: 'minion',
		race: 'human',
		attack: 3,
		defence: 5,
		image_url: ''
	},
	{
		id: 4,
		color: 'white',
		name: 'Radiant Sentinel',
		description: 'Light pours from every crack in her armour.',
		cost: 5,
		type: 'minion',
		race: 'elf',
		attack: 4,
		defence: 6,
		image_url: ''
	},
	{
		id: 5,
		color: 'white',
		name: 'Herald of the Sun',
		description: 'Arrives before the dawn. Leaves after the victory.',
		cost: 6,
		type: 'minion',
		race: 'elf',
		attack: 6,
		defence: 5,
		image_url: ''
	},

	// ── BLACK MINIONS ────────────────────────────────────────
	{
		id: 6,
		color: 'black',
		name: 'Duskblade Rogue',
		description: 'Strikes when the candles go out.',
		cost: 2,
		type: 'minion',
		race: 'human',
		attack: 3,
		defence: 2,
		image_url: ''
	},
	{
		id: 7,
		color: 'black',
		name: 'Cursed Hexblade',
		description: 'Every wound he deals festers.',
		cost: 3,
		type: 'minion',
		race: 'human',
		attack: 4,
		defence: 2,
		image_url: ''
	},
	{
		id: 8,
		color: 'black',
		name: 'Void Stalker',
		description: 'Hunts in the space between thoughts.',
		cost: 4,
		type: 'minion',
		race: 'elf',
		attack: 5,
		defence: 3,
		image_url: ''
	},
	{
		id: 9,
		color: 'black',
		name: 'Grave Warden',
		description: 'Chosen to guard what should stay buried.',
		cost: 4,
		type: 'minion',
		race: 'human',
		attack: 3,
		defence: 5,
		image_url: ''
	},
	{
		id: 10,
		color: 'black',
		name: 'Soulreaper',
		description: 'Collects what death is owed.',
		cost: 6,
		type: 'minion',
		race: 'elf',
		attack: 7,
		defence: 4,
		image_url: ''
	},

	// ── WHITE INCANTATIONS ───────────────────────────────────
	{
		id: 11,
		color: 'white',
		name: 'Divine Shield',
		description: 'Wraps an ally in impenetrable holy light.',
		cost: 2,
		type: 'incantation',
		image_url: ''
	},
	{
		id: 12,
		color: 'white',
		name: 'Holy Wrath',
		description: 'Calls down judgement on the unworthy.',
		cost: 3,
		type: 'incantation',
		image_url: ''
	},

	// ── BLACK INCANTATIONS ───────────────────────────────────
	{
		id: 13,
		color: 'black',
		name: 'Dark Blessing',
		description: 'Power freely given always has a price.',
		cost: 2,
		type: 'incantation',
		image_url: ''
	},
	{
		id: 14,
		color: 'black',
		name: 'Soul Drain',
		description: 'Rips the life force from a target.',
		cost: 3,
		type: 'incantation',
		image_url: ''
	}
];
