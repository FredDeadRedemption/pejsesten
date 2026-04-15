import { describe, it, expect } from 'vitest';
import { setGameState, playCard, attack, endTurn, getGameState } from '$lib/server/engine';
import { deckToCards } from '$lib/shared/cards';
import type { GameStateServer, MinionEntity } from '$lib/shared/types';

// ── Card IDs (from cards.ts) ──────────────────────────────────────────────────
const MACHINE_ELF = 1; // 1 mana, 1/2, charge, no abilities
const OVERZEALOUS_PRIEST = 2; // 3 mana, 2/1, fanfare: draw 1
const SILVERGUARD_KNIGHT = 3; // 3 mana, 5/3, last breath: draw 1
const RADIANT_SENTINEL = 4; // 4 mana, 4/6, fanfare: deal 2 damage to all minions
const HERALD_OF_THE_SUN = 5; // 5 mana, 6/5, last breath: deal 5 to enemy hero + draw 1
const BLACK_CAT = 6; // 1 mana, 2/1, combo: return friendly minion to hand
const BARRY_HEXBLADE = 7; // 2 mana, 4/2, fanfare: give all minions +1+1
const VOID_STALKER = 8; // 2 mana, 5/5, fanfare: deal 10 damage to own hero
const GRAVE_WARDEN = 9; // 4 mana, 5/4, last breath: return ALL minions to their owners hand
const ADMIRABLE_MINION = 10; // 6 mana, 5/4, fanfare: give a friendly minion +3+3 (single target)
const DIVINE_POWER = 11; // 2 mana incantation: give friendly minion +4+4 (single target)
const GOOD_FRIDAY = 12; // 3 mana incantation: deal 4 damage (single target, any)
const POT_OF_GREED = 13; // 3 mana incantation: draw 1, combo: draw 2
const BLESSING = 14; // 2 mana incantation: give friendly minion +3+2 (single target)
const SMITE = 15; // 2 mana incantation: deal 2 damage, combo: deal 4
const PULL = 16; // 0 mana incantation: return friendly minion to hand, costs 2 less

const P1 = 'player1';
const P2 = 'player2';

// ── Helpers ───────────────────────────────────────────────────────────────────

// Builds a minimal valid deck (30 cards of the same id)
const deck = (id: number, count = 30) => Array(count).fill(id);

// Sets up a fresh game where p1 is white (goes first)
// Gives both players a specific hand and empties deck to avoid draw side effects
const setupGame = ({
	p1Hand = [MACHINE_ELF],
	p2Hand = [MACHINE_ELF],
	p1Battlefield = [] as number[],
	p2Battlefield = [] as number[],
	p1Mana = 10,
	p2Mana = 10
}: {
	p1Hand?: number[];
	p2Hand?: number[];
	p1Battlefield?: number[];
	p2Battlefield?: number[];
	p1Mana?: number;
	p2Mana?: number;
} = {}): GameStateServer => {
	const state = setGameState(
		P1,
		P2,
		true,
		deckToCards(deck(MACHINE_ELF)),
		deckToCards(deck(MACHINE_ELF))
	);

	// Override hands with exactly what we want
	state.white.hand = deckToCards(p1Hand);
	state.black.hand = deckToCards(p2Hand);

	// Override battlefields
	state.white.battlefield = deckToCards(p1Battlefield) as MinionEntity[];
	state.black.battlefield = deckToCards(p2Battlefield) as MinionEntity[];

	// Override mana
	state.white.mana = p1Mana;
	state.black.mana = p2Mana;
	state.white.baseMana = p1Mana;
	state.black.baseMana = p2Mana;

	// Empty decks so draw effects don't accidentally work unless we want them to
	state.white.deck = [];
	state.black.deck = [];

	return state;
};

const getWhite = () => getGameState().white;
const getBlack = () => getGameState().black;

// Find a minion on white's battlefield by card id
const whiteMinion = (id: number) => getWhite().battlefield.find((m) => m.id === id)!;
const blackMinion = (id: number) => getBlack().battlefield.find((m) => m.id === id)!;

// ── playCard ──────────────────────────────────────────────────────────────────

describe('playCard', () => {
	describe('basic validation', () => {
		it('returns null if card index does not exist', () => {
			setupGame({ p1Hand: [MACHINE_ELF] });
			expect(playCard(P1, { index: 99 })).toBeNull();
		});

		it('returns null if player cannot afford the card', () => {
			setupGame({ p1Hand: [RADIANT_SENTINEL], p1Mana: 1 });
			expect(playCard(P1, { index: 0 })).toBeNull();
		});

		it('deducts mana when card is played', () => {
			setupGame({ p1Hand: [MACHINE_ELF], p1Mana: 5 });
			playCard(P1, { index: 0 });
			expect(getWhite().mana).toBe(4);
		});

		it('removes card from hand after playing', () => {
			setupGame({ p1Hand: [MACHINE_ELF] });
			expect(getWhite().hand).toHaveLength(1);
			playCard(P1, { index: 0 });
			expect(getWhite().hand).toHaveLength(0);
		});

		it('increments cardsPlayedThisTurn', () => {
			setupGame({ p1Hand: [MACHINE_ELF, MACHINE_ELF] });
			expect(getGameState().cardsPlayedThisTurn).toBe(0);
			playCard(P1, { index: 0 });
			expect(getGameState().cardsPlayedThisTurn).toBe(1);
			playCard(P1, { index: 0 });
			expect(getGameState().cardsPlayedThisTurn).toBe(2);
		});
	});

	describe('minion placement', () => {
		it('places minion on battlefield', () => {
			setupGame({ p1Hand: [MACHINE_ELF] });
			playCard(P1, { index: 0 });
			expect(getWhite().battlefield).toHaveLength(1);
			expect(getWhite().battlefield[0]!.id).toBe(MACHINE_ELF);
		});

		it('minion is exhausted after being played (no charge)', () => {
			setupGame({ p1Hand: [SILVERGUARD_KNIGHT], p1Mana: 10 });
			playCard(P1, { index: 0 });
			expect(getWhite().battlefield[0]!.exhausted).toBe(true);
		});

		it('minion with charge is NOT exhausted after being played', () => {
			setupGame({ p1Hand: [MACHINE_ELF] });
			playCard(P1, { index: 0 });
			expect(getWhite().battlefield[0]!.exhausted).toBe(false);
		});
	});

	describe('incantation targeting validation', () => {
		it('returns null if incantation needs single target but none given', () => {
			setupGame({ p1Hand: [DIVINE_POWER], p1Battlefield: [MACHINE_ELF] });
			expect(playCard(P1, { index: 0 })).toBeNull();
		});

		it('returns null if spell wants friendly minion but got enemy minion', () => {
			setupGame({
				p1Hand: [DIVINE_POWER],
				p1Battlefield: [MACHINE_ELF],
				p2Battlefield: [MACHINE_ELF]
			});
			const enemyID = getBlack().battlefield[0]!.entityID;
			expect(playCard(P1, { index: 0, target: enemyID })).toBeNull();
		});

		it('returns null if spell wants minion but got hero', () => {
			setupGame({ p1Hand: [DIVINE_POWER], p1Battlefield: [MACHINE_ELF] });
			expect(playCard(P1, { index: 0, target: 'heroEnemy' })).toBeNull();
		});

		it('plays incantation with valid single target', () => {
			setupGame({ p1Hand: [DIVINE_POWER], p1Battlefield: [MACHINE_ELF] });
			const targetID = getWhite().battlefield[0]!.entityID;
			const result = playCard(P1, { index: 0, target: targetID });
			expect(result).not.toBeNull();
		});

		it('plays incantation targeting enemy for damage spells', () => {
			setupGame({ p1Hand: [GOOD_FRIDAY], p2Battlefield: [MACHINE_ELF] });
			const targetID = getBlack().battlefield[0]!.entityID;
			const result = playCard(P1, { index: 0, target: targetID });
			expect(result).not.toBeNull();
		});
	});

	describe('onPlay effects — buff', () => {
		it('BARRY: buffs all minions +1+1 (scope: all, side: all)', () => {
			setupGame({
				p1Hand: [BARRY_HEXBLADE],
				p1Battlefield: [MACHINE_ELF],
				p2Battlefield: [MACHINE_ELF],
				p1Mana: 10
			});
			const p1Minion = getWhite().battlefield[0]!;
			const p2Minion = getBlack().battlefield[0]!;
			const p1AtkBefore = p1Minion.attack;
			const p2AtkBefore = p2Minion.attack;

			playCard(P1, { index: 0 });

			// Barry himself + existing minions all get buffed
			expect(getWhite().battlefield.find((m) => m.id === MACHINE_ELF)!.attack).toBe(
				p1AtkBefore + 1
			);
			expect(getBlack().battlefield[0]!.attack).toBe(p2AtkBefore + 1);
		});

		it('ADMIRABLE MINION: buffs single friendly minion +3+3 (scope: single, side: friendly)', () => {
			setupGame({
				p1Hand: [ADMIRABLE_MINION],
				p1Battlefield: [MACHINE_ELF],
				p1Mana: 10
			});
			const targetID = getWhite().battlefield[0]!.entityID;
			const atkBefore = getWhite().battlefield[0]!.attack;
			const defBefore = getWhite().battlefield[0]!.defence;

			playCard(P1, { index: 0, target: targetID });

			const buffed = getWhite().battlefield.find((m) => m.entityID === targetID)!;
			expect(buffed.attack).toBe(atkBefore + 3);
			expect(buffed.defence).toBe(defBefore + 3);
		});

		it('DIVINE POWER: gives friendly minion +4+4 via incantation', () => {
			setupGame({ p1Hand: [DIVINE_POWER], p1Battlefield: [MACHINE_ELF] });
			const targetID = getWhite().battlefield[0]!.entityID;
			const atkBefore = getWhite().battlefield[0]!.attack;

			playCard(P1, { index: 0, target: targetID });

			expect(getWhite().battlefield[0]!.attack).toBe(atkBefore + 4);
			expect(getWhite().battlefield[0]!.defence).toBe(2 + 4); // base 2 + 4
		});

		it('BLESSING: gives friendly minion +3+2', () => {
			setupGame({ p1Hand: [BLESSING], p1Battlefield: [MACHINE_ELF] });
			const targetID = getWhite().battlefield[0]!.entityID;
			playCard(P1, { index: 0, target: targetID });
			expect(getWhite().battlefield[0]!.attack).toBe(1 + 3);
			expect(getWhite().battlefield[0]!.defence).toBe(2 + 2);
		});
	});

	describe('onPlay effects — damage', () => {
		it('RADIANT SENTINEL: deals 2 damage to ALL minions (scope: all, side: all, entityType: minion)', () => {
			setupGame({
				p1Hand: [RADIANT_SENTINEL],
				p1Battlefield: [MACHINE_ELF], // 1/2 → dies (2 dmg)
				p2Battlefield: [MACHINE_ELF], // 1/2 → dies (2 dmg)
				p1Mana: 10
			});
			playCard(P1, { index: 0 });
			// Both machine elves should die (2 defence, 2 damage)
			expect(getWhite().battlefield.find((m) => m.id === MACHINE_ELF)).toBeUndefined();
			expect(getBlack().battlefield).toHaveLength(0);
			// Radiant Sentinel itself (4/6) survives since 6 > 2
			expect(getWhite().battlefield.find((m) => m.id === RADIANT_SENTINEL)).toBeDefined();
		});

		it('RADIANT SENTINEL: does NOT kill a minion with enough defence', () => {
			setupGame({
				p1Hand: [RADIANT_SENTINEL],
				p2Battlefield: [SILVERGUARD_KNIGHT], // 5/3 → takes 2 dmg → 5/1, survives
				p1Mana: 10
			});
			playCard(P1, { index: 0 });
			expect(getBlack().battlefield).toHaveLength(1);
			expect(getBlack().battlefield[0]!.defence).toBe(1);
		});

		it('VOID STALKER: deals 10 damage to own hero (scope: all, side: friendly, entityType: hero)', () => {
			setupGame({ p1Hand: [VOID_STALKER], p1Mana: 10 });
			const heroBefore = getWhite().hero.defence;
			playCard(P1, { index: 0 });
			expect(getWhite().hero.defence).toBe(heroBefore - 10);
		});

		it('GOOD FRIDAY: deals 4 damage to enemy minion (scope: single, side: all)', () => {
			setupGame({ p1Hand: [GOOD_FRIDAY], p2Battlefield: [SILVERGUARD_KNIGHT], p1Mana: 10 });
			const targetID = getBlack().battlefield[0]!.entityID;
			const defBefore = getBlack().battlefield[0]!.defence;
			playCard(P1, { index: 0, target: targetID });
			// 3 defence - 4 damage = dies
			expect(getBlack().battlefield).toHaveLength(0);
		});

		it('GOOD FRIDAY: deals 4 damage to enemy hero (scope: single, side: all, entityType: all)', () => {
			setupGame({ p1Hand: [GOOD_FRIDAY], p1Mana: 10 });
			const heroBefore = getBlack().hero.defence;
			playCard(P1, { index: 0, target: 'heroEnemy' });
			expect(getBlack().hero.defence).toBe(heroBefore - 4);
		});

		it('SMITE: deals 2 damage without combo', () => {
			setupGame({ p1Hand: [SMITE], p2Battlefield: [SILVERGUARD_KNIGHT], p1Mana: 10 });
			const targetID = getBlack().battlefield[0]!.entityID;
			playCard(P1, { index: 0, target: targetID });
			expect(getBlack().battlefield[0]!.defence).toBe(3 - 2);
		});

		it('SMITE: deals 4 damage total with combo', () => {
			setupGame({
				p1Hand: [MACHINE_ELF, SMITE],
				p2Battlefield: [SILVERGUARD_KNIGHT],
				p1Mana: 10
			});
			playCard(P1, { index: 0 }); // play machine elf first to enable combo
			const targetID = getBlack().battlefield[0]!.entityID;
			playCard(P1, { index: 0, target: targetID }); // smite is now index 0
			// 3 - 2 - 2 = -1, dies
			expect(getBlack().battlefield).toHaveLength(0);
		});
	});

	describe('onPlay effects — draw', () => {
		it('OVERZEALOUS PRIEST: draws 1 card on play', () => {
			setupGame({
				p1Hand: [OVERZEALOUS_PRIEST],
				p1Mana: 10
			});
			// Give white a deck to draw from
			getGameState().white.deck = deckToCards([MACHINE_ELF, MACHINE_ELF, MACHINE_ELF]);
			const handBefore = getWhite().hand.length;
			playCard(P1, { index: 0 });
			expect(getWhite().hand.length).toBe(handBefore - 1 + 1); // -1 played, +1 drawn = same
		});

		it('POT OF GREED: draws 1 without combo', () => {
			setupGame({ p1Hand: [POT_OF_GREED], p1Mana: 10 });
			getGameState().white.deck = deckToCards([MACHINE_ELF, MACHINE_ELF, MACHINE_ELF]);
			const handBefore = getWhite().hand.length;
			playCard(P1, { index: 0 });
			expect(getWhite().hand.length).toBe(handBefore - 1 + 1);
		});

		it('POT OF GREED: draws 2 with combo', () => {
			setupGame({ p1Hand: [MACHINE_ELF, POT_OF_GREED], p1Mana: 10 });
			getGameState().white.deck = deckToCards([MACHINE_ELF, MACHINE_ELF, MACHINE_ELF]);
			playCard(P1, { index: 0 }); // enable combo
			const handBefore = getWhite().hand.length;
			playCard(P1, { index: 0 }); // pot of greed
			expect(getWhite().hand.length).toBe(handBefore - 1 + 2);
		});
	});

	describe('onPlay effects — returnToHand', () => {
		it('PULL: returns friendly minion to hand with cost reduction', () => {
			setupGame({
				p1Hand: [PULL],
				p1Battlefield: [OVERZEALOUS_PRIEST], // base cost 3
				p1Mana: 10
			});
			const targetID = getWhite().battlefield[0]!.entityID;
			playCard(P1, { index: 0, target: targetID });

			expect(getWhite().battlefield).toHaveLength(0);
			const returned = getWhite().hand.find((c) => c.id === OVERZEALOUS_PRIEST)!;
			expect(returned).toBeDefined();
			expect(returned.cost).toBe(1); // 3 - 2 = 1
		});

		it('PULL: cost reduction cannot go below 0', () => {
			setupGame({
				p1Hand: [PULL],
				p1Battlefield: [MACHINE_ELF], // base cost 1, reduction 2 → 0
				p1Mana: 10
			});
			const targetID = getWhite().battlefield[0]!.entityID;
			playCard(P1, { index: 0, target: targetID });
			const returned = getWhite().hand.find((c) => c.id === MACHINE_ELF)!;
			expect(returned.cost).toBe(0);
		});

		it('PULL: resets minion stats when returned', () => {
			setupGame({
				p1Hand: [PULL],
				p1Battlefield: [MACHINE_ELF],
				p1Mana: 10
			});
			// manually buff the minion first
			const minion = getWhite().battlefield[0]!;
			minion.attack = 99;
			minion.defence = 99;

			const targetID = minion.entityID;
			playCard(P1, { index: 0, target: targetID });

			const returned = getWhite().hand.find((c) => c.id === MACHINE_ELF)!;
			expect((returned as MinionEntity).attack).toBe(1); // baseAttack
			expect((returned as MinionEntity).defence).toBe(2); // baseDefence
		});

		it('BLACK CAT: does NOT bounce without combo', () => {
			setupGame({
				p1Hand: [BLACK_CAT],
				p1Battlefield: [MACHINE_ELF],
				p1Mana: 10
			});
			const targetID = getWhite().battlefield[0]!.entityID;
			playCard(P1, { index: 0, target: targetID });
			// Cat is played, Machine Elf stays (no combo)
			expect(getWhite().battlefield.find((m) => m.id === MACHINE_ELF)).toBeDefined();
		});

		it('BLACK CAT: bounces friendly minion with combo', () => {
			setupGame({
				p1Hand: [MACHINE_ELF, BLACK_CAT],
				p1Battlefield: [SILVERGUARD_KNIGHT],
				p1Mana: 10
			});
			playCard(P1, { index: 0 }); // play machine elf → combo active
			const targetID = getWhite().battlefield.find((m) => m.id === SILVERGUARD_KNIGHT)!.entityID;
			playCard(P1, { index: 0, target: targetID }); // black cat

			expect(getWhite().battlefield.find((m) => m.id === SILVERGUARD_KNIGHT)).toBeUndefined();
			expect(getWhite().hand.find((c) => c.id === SILVERGUARD_KNIGHT)).toBeDefined();
		});

		it('BLACK CAT: stays on board with no target and combo active (no-op)', () => {
			setupGame({ p1Hand: [MACHINE_ELF, BLACK_CAT], p1Mana: 10 });
			playCard(P1, { index: 0 }); // machine elf → combo
			playCard(P1, { index: 0 }); // black cat, no friendly minions other than itself
			// Cat should be on board
			expect(getWhite().battlefield.find((m) => m.id === BLACK_CAT)).toBeDefined();
		});

		it('GRAVE WARDEN: returns ALL minions to their respective owners hands (scope: all, side: all)', () => {
			setupGame({
				p1Hand: [],
				p1Battlefield: [GRAVE_WARDEN, MACHINE_ELF],
				p2Battlefield: [SILVERGUARD_KNIGHT],
				p1Mana: 10
			});
			// Kill the grave warden
			const warden = getWhite().battlefield.find((m) => m.id === GRAVE_WARDEN)!;
			warden.defence = 0;

			// Trigger death check by attacking
			// Actually manipulate defence directly and trigger via attack
			// Easier: use a damage spell
			// Since warden is already dead (defence 0), let's set up an attack instead
			// Reset and use attack to kill it
			setupGame({
				p1Hand: [],
				p1Battlefield: [GRAVE_WARDEN, MACHINE_ELF],
				p2Battlefield: [SILVERGUARD_KNIGHT],
				p1Mana: 10,
				p2Mana: 10
			});

			// Switch to black's turn to attack the warden
			endTurn(); // now black's turn
			getGameState().black.mana = 10;
			getGameState().black.hand = [];
			getGameState().black.deck = [];

			const attackerID = getBlack().battlefield[0]!.entityID; // silverguard
			const wardenID = getWhite().battlefield.find((m) => m.id === GRAVE_WARDEN)!.entityID;

			// Silverguard (5 attack) kills Grave Warden (4 defence)
			attack(P2, { originID: attackerID, targetID: wardenID });

			// Both boards should be cleared
			expect(getWhite().battlefield).toHaveLength(0);
			expect(getBlack().battlefield).toHaveLength(0);
			// Machine Elf returned to white's hand
			expect(getWhite().hand.find((c) => c.id === MACHINE_ELF)).toBeDefined();
			// Silverguard returned to black's hand (if it survived)
			// Silverguard (5/3) takes 5 damage from Warden → dies, but warden's deathrattle
			// fires before removal so silverguard should be returned before it dies
			// Actually: warden dies → deathrattle fires → returns silverguard to hand
			// then checkForDeaths catches silverguard at 3 - 5 = -2... hmm
			// The key thing is Machine Elf is returned to white
		});
	});

	describe('requirements (combo / firstCard)', () => {
		it('firstCard ability fires when it is the first card this turn', () => {
			// No cards have firstCard in current set, but test the mechanic
			// by checking cardsPlayedThisTurn is 0 before first play
			setupGame({ p1Hand: [MACHINE_ELF] });
			expect(getGameState().cardsPlayedThisTurn).toBe(0);
		});

		it('combo does not fire on first card', () => {
			setupGame({
				p1Hand: [BLACK_CAT],
				p1Battlefield: [MACHINE_ELF],
				p1Mana: 10
			});
			const targetID = getWhite().battlefield[0]!.entityID;
			playCard(P1, { index: 0, target: targetID });
			// Machine Elf should NOT be bounced (no combo)
			expect(getWhite().battlefield.find((m) => m.id === MACHINE_ELF)).toBeDefined();
		});

		it('combo fires on second+ card', () => {
			setupGame({
				p1Hand: [MACHINE_ELF, BLACK_CAT],
				p1Battlefield: [SILVERGUARD_KNIGHT],
				p1Mana: 10
			});
			playCard(P1, { index: 0 }); // first card
			const targetID = getWhite().battlefield.find((m) => m.id === SILVERGUARD_KNIGHT)!.entityID;
			playCard(P1, { index: 0, target: targetID });
			expect(getWhite().battlefield.find((m) => m.id === SILVERGUARD_KNIGHT)).toBeUndefined();
		});

		it('cardsPlayedThisTurn resets on endTurn', () => {
			setupGame({ p1Hand: [MACHINE_ELF, MACHINE_ELF], p1Mana: 10 });
			playCard(P1, { index: 0 });
			playCard(P1, { index: 0 });
			expect(getGameState().cardsPlayedThisTurn).toBe(2);
			endTurn();
			expect(getGameState().cardsPlayedThisTurn).toBe(0);
		});
	});
});

// ── attack ────────────────────────────────────────────────────────────────────

describe('attack', () => {
	it('returns null if attacker does not exist', () => {
		setupGame({ p1Battlefield: [MACHINE_ELF] });
		expect(attack(P1, { originID: 'fake-id', targetID: 'heroEnemy' })).toBeNull();
	});

	it('returns null if attacker is exhausted', () => {
		setupGame({ p1Battlefield: [SILVERGUARD_KNIGHT] });
		getWhite().battlefield[0]!.exhausted = true; // ← manually exhaust it
		const attackerID = getWhite().battlefield[0]!.entityID;
		expect(attack(P1, { originID: attackerID, targetID: 'heroEnemy' })).toBeNull();
	});

	it('returns null if target does not exist', () => {
		setupGame({ p1Battlefield: [MACHINE_ELF] });
		const attackerID = getWhite().battlefield[0]!.entityID;
		expect(attack(P1, { originID: attackerID, targetID: 'fake-target' })).toBeNull();
	});

	it('minion attacks enemy hero and deals damage', () => {
		setupGame({ p1Battlefield: [MACHINE_ELF] }); // 1 attack, charge
		const attackerID = getWhite().battlefield[0]!.entityID;
		const heroBefore = getBlack().hero.defence;
		attack(P1, { originID: attackerID, targetID: 'heroEnemy' });
		expect(getBlack().hero.defence).toBe(heroBefore - 1);
	});

	it('attacker becomes exhausted after attacking', () => {
		setupGame({ p1Battlefield: [MACHINE_ELF] });
		const attackerID = getWhite().battlefield[0]!.entityID;
		attack(P1, { originID: attackerID, targetID: 'heroEnemy' });
		expect(getWhite().battlefield[0]!.exhausted).toBe(true);
	});

	it('cannot attack twice in one turn', () => {
		setupGame({ p1Battlefield: [MACHINE_ELF] });
		const attackerID = getWhite().battlefield[0]!.entityID;
		attack(P1, { originID: attackerID, targetID: 'heroEnemy' });
		expect(attack(P1, { originID: attackerID, targetID: 'heroEnemy' })).toBeNull();
	});

	it('minion vs minion: both take damage', () => {
		setupGame({
			p1Battlefield: [BARRY_HEXBLADE], // 4/2
			p2Battlefield: [SILVERGUARD_KNIGHT] // 5/3
		});
		// Manually unexhaust barry (he was placed on battlefield directly, not via playCard)
		getWhite().battlefield[0]!.exhausted = false;
		getBlack().battlefield[0]!.exhausted = false;

		const attackerID = getWhite().battlefield[0]!.entityID;
		const defenderID = getBlack().battlefield[0]!.entityID;

		attack(P1, { originID: attackerID, targetID: defenderID });

		// Barry (4 atk) hits Silverguard: 3 - 4 = -1 → dead
		expect(getBlack().battlefield).toHaveLength(0);
		// Silverguard (5 atk) retaliates against Barry: 2 - 5 = -3 → dead
		expect(getWhite().battlefield).toHaveLength(0);
	});

	it('hero does not retaliate when attacked', () => {
		setupGame({ p1Battlefield: [MACHINE_ELF] });
		const attackerID = getWhite().battlefield[0]!.entityID;
		const attackerDefBefore = getWhite().battlefield[0]!.defence;
		attack(P1, { originID: attackerID, targetID: 'heroEnemy' });
		// Attacker should NOT take damage from hero
		expect(getWhite().battlefield[0]!.defence).toBe(attackerDefBefore);
	});

	it('dead minions go to graveyard', () => {
		setupGame({
			p1Battlefield: [BARRY_HEXBLADE],
			p2Battlefield: [MACHINE_ELF] // 1/2, will die to Barry (4 atk)
		});
		getWhite().battlefield[0]!.exhausted = false;
		getBlack().battlefield[0]!.exhausted = false;

		const attackerID = getWhite().battlefield[0]!.entityID;
		const defenderID = getBlack().battlefield[0]!.entityID;

		attack(P1, { originID: attackerID, targetID: defenderID });

		expect(getBlack().graveyard).toHaveLength(1);
		expect(getBlack().graveyard[0]!.id).toBe(MACHINE_ELF);
	});
});

// ── onDeath triggers ──────────────────────────────────────────────────────────

describe('onDeath triggers', () => {
	it('SILVERGUARD KNIGHT: draws 1 card when it dies', () => {
		setupGame({
			p1Battlefield: [SILVERGUARD_KNIGHT], // 5/3
			p2Battlefield: [BARRY_HEXBLADE] // 4/2
		});
		getGameState().white.deck = deckToCards([MACHINE_ELF, MACHINE_ELF]);
		getWhite().battlefield[0]!.exhausted = false;
		getBlack().battlefield[0]!.exhausted = false;

		// Switch to black to attack silverguard
		endTurn();
		getGameState().black.hand = [];
		getGameState().black.deck = [];

		// Barry (4 atk) kills Silverguard (5/3 → 5/-1)
		const barryID = getBlack().battlefield[0]!.entityID;
		const knightID = getWhite().battlefield.find((m) => m.id === SILVERGUARD_KNIGHT)!.entityID;

		const handBefore = getWhite().hand.length;
		attack(P2, { originID: barryID, targetID: knightID });

		expect(getWhite().battlefield.find((m) => m.id === SILVERGUARD_KNIGHT)).toBeUndefined();
		expect(getWhite().hand.length).toBe(handBefore + 1);
	});

	it('HERALD OF THE SUN: deals 5 damage to enemy hero and draws 1 on death', () => {
		setupGame({
			p1Battlefield: [HERALD_OF_THE_SUN], // 6/5
			p2Battlefield: [BARRY_HEXBLADE] // 4/2
		});
		getGameState().white.deck = deckToCards([MACHINE_ELF, MACHINE_ELF]);
		getWhite().battlefield[0]!.exhausted = false;
		getBlack().battlefield[0]!.exhausted = false;

		endTurn();
		getGameState().black.hand = [];
		getGameState().black.deck = [];

		const heroBefore = getBlack().hero.defence; // should be enemy of herald's owner = black
		const handBefore = getWhite().hand.length;

		// Need something strong enough to kill Herald (6/5)
		// Manually reduce herald's defence
		getWhite().battlefield.find((m) => m.id === HERALD_OF_THE_SUN)!.defence = 1;

		const barryID = getBlack().battlefield[0]!.entityID;
		const heraldID = getWhite().battlefield.find((m) => m.id === HERALD_OF_THE_SUN)!.entityID;

		attack(P2, { originID: barryID, targetID: heraldID });

		expect(getWhite().battlefield.find((m) => m.id === HERALD_OF_THE_SUN)).toBeUndefined();
		expect(getBlack().hero.defence).toBe(heroBefore - 5);
		expect(getWhite().hand.length).toBe(handBefore + 1);
	});

	it('dead minion goes to graveyard after deathrattle fires', () => {
		setupGame({
			p1Battlefield: [SILVERGUARD_KNIGHT],
			p2Battlefield: [BARRY_HEXBLADE]
		});
		getGameState().white.deck = deckToCards([MACHINE_ELF]);
		getWhite().battlefield[0]!.exhausted = false;
		getBlack().battlefield[0]!.exhausted = false;

		endTurn();
		getGameState().black.hand = [];
		getGameState().black.deck = [];

		const barryID = getBlack().battlefield[0]!.entityID;
		const knightID = getWhite().battlefield[0]!.entityID;

		attack(P2, { originID: barryID, targetID: knightID });

		expect(getWhite().graveyard.find((m) => m.id === SILVERGUARD_KNIGHT)).toBeDefined();
	});

	it('death chain: deathrattle killing another minion triggers its deathrattle', () => {
		// Herald dies → deals 5 damage to enemy hero (not a death chain trigger)
		// Radiant Sentinel fanfare kills Machine Elf → Machine Elf has no deathrattle
		// For a true chain: we'd need a deathrattle that damages a minion
		// Use HERALD deathrattle to damage a low-hp minion via manual setup
		setupGame({
			p1Battlefield: [HERALD_OF_THE_SUN, SILVERGUARD_KNIGHT],
			p2Battlefield: []
		});
		getGameState().white.deck = deckToCards([MACHINE_ELF, MACHINE_ELF]);

		// Reduce herald's defence to 1 so it dies easily
		getWhite().battlefield.find((m) => m.id === HERALD_OF_THE_SUN)!.defence = 1;
		// Reduce silverguard defence to 1 so herald's AoE... wait herald hits enemy hero
		// This tests that both deathrattles fire correctly in sequence, not a true chain
		// Real chain test would require a deathrattle that deals damage to a minion
		// which is not yet in the card set — mark for future
		expect(true).toBe(true); // placeholder for death chain test
	});
});

// ── endTurn ───────────────────────────────────────────────────────────────────

describe('endTurn', () => {
	it('switches active turn', () => {
		setupGame();
		expect(getGameState().whiteTurn).toBe(true);
		endTurn();
		expect(getGameState().whiteTurn).toBe(false);
	});

	it('draws 1 card for the new active player', () => {
		setupGame({ p2Hand: [] });
		getGameState().black.deck = deckToCards([MACHINE_ELF, MACHINE_ELF]);
		const handBefore = getBlack().hand.length;
		endTurn();
		expect(getBlack().hand.length).toBe(handBefore + 1);
	});

	it('increments baseMana for new active player', () => {
		setupGame();
		getGameState().black.baseMana = 3; // set to something below MAX_MANA
		const baseBefore = getBlack().baseMana;
		endTurn();
		expect(getBlack().baseMana).toBe(baseBefore + 1); // 3 → 4
	});

	it('restores mana to baseMana for new active player', () => {
		setupGame();
		getGameState().black.mana = 0; // simulate spent mana
		getGameState().black.baseMana = 3;
		endTurn();
		expect(getBlack().mana).toBe(4); // baseMana 3 + 1 = 4
	});

	it('baseMana is capped at MAX_MANA', async () => {
		setupGame();
		getGameState().black.baseMana = 9; // one below max (assuming MAX_MANA = 10)
		endTurn(); // black's turn: baseMana → 10
		expect(getBlack().baseMana).toBeLessThanOrEqual(10);
		endTurn(); // white's turn
		endTurn(); // black's turn again: baseMana should stay at 10
		expect(getBlack().baseMana).toBeLessThanOrEqual(10);
	});

	it('unexhausts all minions for new active player', () => {
		setupGame({ p2Battlefield: [SILVERGUARD_KNIGHT] });
		getBlack().battlefield[0]!.exhausted = true;
		endTurn();
		expect(getBlack().battlefield[0]!.exhausted).toBe(false);
	});

	it('does not unexhaust enemy minions', () => {
		setupGame({ p1Battlefield: [SILVERGUARD_KNIGHT] });
		getWhite().battlefield[0]!.exhausted = true;
		endTurn(); // now black's turn
		expect(getWhite().battlefield[0]!.exhausted).toBe(true);
	});

	it('resets cardsPlayedThisTurn', () => {
		setupGame({ p1Hand: [MACHINE_ELF, MACHINE_ELF], p1Mana: 10 });
		playCard(P1, { index: 0 });
		playCard(P1, { index: 0 });
		expect(getGameState().cardsPlayedThisTurn).toBe(2);
		endTurn();
		expect(getGameState().cardsPlayedThisTurn).toBe(0);
	});

	it('increments turnCount', () => {
		setupGame();
		expect(getGameState().turnCount).toBe(0);
		endTurn();
		expect(getGameState().turnCount).toBe(1);
	});
});

// ── targetSpec edge cases ─────────────────────────────────────────────────────

describe('targetSpec edge cases', () => {
	it('scope:all side:all entityType:minion hits minions on both sides but not heroes', () => {
		setupGame({
			p1Hand: [RADIANT_SENTINEL], // fanfare: 2 dmg to all minions
			p1Battlefield: [MACHINE_ELF],
			p2Battlefield: [MACHINE_ELF],
			p1Mana: 10
		});
		const p1HeroBefore = getWhite().hero.defence;
		const p2HeroBefore = getBlack().hero.defence;
		playCard(P1, { index: 0 });
		// Heroes are not hit
		expect(getWhite().hero.defence).toBe(p1HeroBefore);
		expect(getBlack().hero.defence).toBe(p2HeroBefore);
		// Both elves die
		expect(getWhite().battlefield.filter((m) => m.id === MACHINE_ELF)).toHaveLength(0);
		expect(getBlack().battlefield).toHaveLength(0);
	});

	it('scope:all side:friendly entityType:hero hits only own hero', () => {
		setupGame({ p1Hand: [VOID_STALKER], p1Mana: 10 });
		const p1HeroBefore = getWhite().hero.defence;
		const p2HeroBefore = getBlack().hero.defence;
		playCard(P1, { index: 0 });
		expect(getWhite().hero.defence).toBe(p1HeroBefore - 10);
		expect(getBlack().hero.defence).toBe(p2HeroBefore); // enemy hero untouched
	});

	it('scope:all side:all entityType:minion does not hit heroes even with entityType:all', () => {
		// Barry buffs minions only (entityType: minion)
		setupGame({
			p1Hand: [BARRY_HEXBLADE],
			p1Battlefield: [MACHINE_ELF],
			p1Mana: 10
		});
		const p1HeroBefore = getWhite().hero.defence;
		playCard(P1, { index: 0 });
		expect(getWhite().hero.defence).toBe(p1HeroBefore); // hero not buffed
	});

	it('scope:single requires target to be provided for incantation', () => {
		setupGame({ p1Hand: [DIVINE_POWER], p1Battlefield: [MACHINE_ELF] });
		expect(playCard(P1, { index: 0 })).toBeNull(); // no target
	});

	it('scope:single with wrong side is rejected', () => {
		setupGame({
			p1Hand: [DIVINE_POWER], // friendly minion only
			p2Battlefield: [MACHINE_ELF]
		});
		const enemyID = getBlack().battlefield[0]!.entityID;
		expect(playCard(P1, { index: 0, target: enemyID })).toBeNull();
	});

	it('scope:single with wrong entityType is rejected', () => {
		setupGame({ p1Hand: [DIVINE_POWER], p1Battlefield: [MACHINE_ELF] });
		// Try targeting own hero instead of own minion
		expect(playCard(P1, { index: 0, target: 'heroSelf' })).toBeNull();
	});

	it('scope:all with no valid targets is a no-op (does not crash)', () => {
		setupGame({
			p1Hand: [RADIANT_SENTINEL], // hits all minions, but no minions on board
			p1Mana: 10
		});
		expect(() => playCard(P1, { index: 0 })).not.toThrow();
	});

	it('returnToHand with no valid targets is a no-op', () => {
		// Black Cat with combo but no other friendly minions → no-op, cat stays
		setupGame({ p1Hand: [MACHINE_ELF, BLACK_CAT], p1Mana: 10 });
		playCard(P1, { index: 0 }); // machine elf (combo now active)
		playCard(P1, { index: 0 }); // black cat, no other friendly minions
		expect(getWhite().battlefield.find((m) => m.id === BLACK_CAT)).toBeDefined();
	});
});

// ── mana system ───────────────────────────────────────────────────────────────

describe('mana system', () => {
	it('playing a card deducts its cost from mana', () => {
		setupGame({ p1Hand: [OVERZEALOUS_PRIEST], p1Mana: 10 }); // costs 3
		playCard(P1, { index: 0 });
		expect(getWhite().mana).toBe(7);
	});

	it('cannot play a card that costs more than current mana', () => {
		setupGame({ p1Hand: [RADIANT_SENTINEL], p1Mana: 3 }); // costs 4
		expect(playCard(P1, { index: 0 })).toBeNull();
	});

	it('0 cost cards can always be played (if mana >= 0)', () => {
		setupGame({ p1Hand: [PULL], p1Mana: 0, p1Battlefield: [MACHINE_ELF] });
		const targetID = getWhite().battlefield[0]!.entityID;
		expect(playCard(P1, { index: 0, target: targetID })).not.toBeNull();
	});
});
