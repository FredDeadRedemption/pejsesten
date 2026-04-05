import type { Card } from "./types";

export const getCardByID = (id: number) => cards.find(card => card.id === id);

export const getCards = () => cards;

export const deckToCards = (deck: number[]) => deck.map(id => getCardByID(id)!);

export const cards: Card[] = [
  // ── HUMANS ──────────────────────────────────────────────
  { id: 1, name: "Village Guard", description: "A stalwart defender of the realm.", cost: 1, type: "creature", race: "human", attack: 1, defence: 3, image_url: "/cards/village-guard.png" },
  { id: 2, name: "Peasant Farmer", description: "Strength in numbers.", cost: 1, type: "creature", race: "human", attack: 1, defence: 2, image_url: "/cards/peasant-farmer.png" },
  { id: 3, name: "Town Crier", description: "Rallies allies with a booming voice.", cost: 1, type: "creature", race: "human", attack: 2, defence: 1, image_url: "/cards/town-crier.png" },
  { id: 4, name: "Militia Recruit", description: "Fresh from training, eager to prove worth.", cost: 2, type: "creature", race: "human", attack: 2, defence: 2, image_url: "/cards/militia-recruit.png" },
  { id: 5, name: "Shield Maiden", description: "Her shield has never been broken.", cost: 2, type: "creature", race: "human", attack: 1, defence: 4, image_url: "/cards/shield-maiden.png" },
  { id: 6, name: "Crossbow Marksman", description: "Precise. Patient. Deadly.", cost: 2, type: "creature", race: "human", attack: 3, defence: 1, image_url: "/cards/crossbow-marksman.png" },
  { id: 7, name: "Footsoldier", description: "Trained for the front lines.", cost: 2, type: "creature", race: "human", attack: 2, defence: 3, image_url: "/cards/footsoldier.png" },
  { id: 8, name: "Sellsword", description: "Fights for whoever pays most.", cost: 3, type: "creature", race: "human", attack: 4, defence: 2, image_url: "/cards/sellsword.png" },
  { id: 9, name: "Hedge Knight", description: "A knight without a lord.", cost: 3, type: "creature", race: "human", attack: 3, defence: 3, image_url: "/cards/hedge-knight.png" },
  { id: 10, name: "Royal Squire", description: "Aspires to knighthood.", cost: 3, type: "creature", race: "human", attack: 2, defence: 4, image_url: "/cards/royal-squire.png" },
  { id: 11, name: "Battle Priest", description: "Blesses allies before battle.", cost: 3, type: "creature", race: "human", attack: 3, defence: 2, image_url: "/cards/battle-priest.png" },
  { id: 12, name: "City Watch Captain", description: "Commands respect in every district.", cost: 4, type: "creature", race: "human", attack: 3, defence: 4, image_url: "/cards/city-watch-captain.png" },
  { id: 13, name: "Inquisitor", description: "Questions first, then strikes.", cost: 4, type: "creature", race: "human", attack: 4, defence: 3, image_url: "/cards/inquisitor.png" },
  { id: 14, name: "Veteran Swordsman", description: "Every scar tells a story.", cost: 4, type: "creature", race: "human", attack: 5, defence: 2, image_url: "/cards/veteran-swordsman.png" },
  { id: 15, name: "War Cleric", description: "Smites enemies with holy fury.", cost: 4, type: "creature", race: "human", attack: 4, defence: 4, image_url: "/cards/war-cleric.png" },
  { id: 16, name: "Knight Commander", description: "Orders the charge.", cost: 5, type: "creature", race: "human", attack: 5, defence: 4, image_url: "/cards/knight-commander.png" },
  { id: 17, name: "Royal Champion", description: "The king's chosen blade.", cost: 5, type: "creature", race: "human", attack: 6, defence: 3, image_url: "/cards/royal-champion.png" },
  { id: 18, name: "Siege Engineer", description: "Breaks walls, breaks wills.", cost: 5, type: "creature", race: "human", attack: 4, defence: 5, image_url: "/cards/siege-engineer.png" },
  { id: 19, name: "Grand Paladin", description: "Light made flesh.", cost: 6, type: "creature", race: "human", attack: 6, defence: 5, image_url: "/cards/grand-paladin.png" },
  { id: 20, name: "High General", description: "Undefeated in 30 campaigns.", cost: 6, type: "creature", race: "human", attack: 5, defence: 6, image_url: "/cards/high-general.png" },
  { id: 21, name: "King's Executioner", description: "The last face many ever see.", cost: 7, type: "creature", race: "human", attack: 8, defence: 4, image_url: "/cards/kings-executioner.png" },
  { id: 22, name: "Warlord of the North", description: "Conquered ten kingdoms before thirty.", cost: 8, type: "creature", race: "human", attack: 8, defence: 7, image_url: "/cards/warlord-north.png" },

  // ── ELVES ───────────────────────────────────────────────
  { id: 23, name: "Woodland Scout", description: "Silent as falling leaves.", cost: 1, type: "creature", race: "elf", attack: 2, defence: 1, image_url: "/cards/woodland-scout.png" },
  { id: 24, name: "Faerie Trickster", description: "Illusions and misdirection.", cost: 1, type: "creature", race: "elf", attack: 1, defence: 2, image_url: "/cards/faerie-trickster.png" },
  { id: 25, name: "Elven Page", description: "Young, fast, underestimated.", cost: 1, type: "creature", race: "elf", attack: 1, defence: 1, image_url: "/cards/elven-page.png" },
  { id: 26, name: "Moonlit Archer", description: "Shoots between the stars.", cost: 2, type: "creature", race: "elf", attack: 3, defence: 1, image_url: "/cards/moonlit-archer.png" },
  { id: 27, name: "Grove Warden", description: "Bound to the ancient trees.", cost: 2, type: "creature", race: "elf", attack: 2, defence: 3, image_url: "/cards/grove-warden.png" },
  { id: 28, name: "Sylvan Duelist", description: "Two blades, one heartbeat.", cost: 2, type: "creature", race: "elf", attack: 3, defence: 2, image_url: "/cards/sylvan-duelist.png" },
  { id: 29, name: "River Dancer", description: "Flows around every attack.", cost: 3, type: "creature", race: "elf", attack: 2, defence: 4, image_url: "/cards/river-dancer.png" },
  { id: 30, name: "Starweave Mage", description: "Draws power from constellations.", cost: 3, type: "creature", race: "elf", attack: 4, defence: 2, image_url: "/cards/starweave-mage.png" },
  { id: 31, name: "Thornblade Ranger", description: "The forest bends to her will.", cost: 3, type: "creature", race: "elf", attack: 3, defence: 3, image_url: "/cards/thornblade-ranger.png" },
  { id: 32, name: "Dawnseeker", description: "Wakes before the sun.", cost: 3, type: "creature", race: "elf", attack: 3, defence: 4, image_url: "/cards/dawnseeker.png" },
  { id: 33, name: "Veilstep Assassin", description: "Strikes from the shadow between shadows.", cost: 4, type: "creature", race: "elf", attack: 5, defence: 2, image_url: "/cards/veilstep-assassin.png" },
  { id: 34, name: "High Elf Lancer", description: "Ancient lineage, ancient fury.", cost: 4, type: "creature", race: "elf", attack: 4, defence: 3, image_url: "/cards/high-elf-lancer.png" },
  { id: 35, name: "Glade Champion", description: "Winner of ten thousand duels.", cost: 4, type: "creature", race: "elf", attack: 4, defence: 4, image_url: "/cards/glade-champion.png" },
  { id: 36, name: "Oracle of Leaves", description: "Reads the future in falling foliage.", cost: 4, type: "creature", race: "elf", attack: 3, defence: 5, image_url: "/cards/oracle-of-leaves.png" },
  { id: 37, name: "Moonfire Arcanist", description: "Channels lunar energy into destruction.", cost: 5, type: "creature", race: "elf", attack: 6, defence: 3, image_url: "/cards/moonfire-arcanist.png" },
  { id: 38, name: "Eternal Sentinel", description: "Has stood watch for a thousand years.", cost: 5, type: "creature", race: "elf", attack: 4, defence: 6, image_url: "/cards/eternal-sentinel.png" },
  { id: 39, name: "Songweaver", description: "Her melody unmakes steel.", cost: 5, type: "creature", race: "elf", attack: 5, defence: 5, image_url: "/cards/songweaver.png" },
  { id: 40, name: "Elder of the Grove", description: "Trees bow when she walks.", cost: 6, type: "creature", race: "elf", attack: 5, defence: 7, image_url: "/cards/elder-of-the-grove.png" },
  { id: 41, name: "Starfall Invoker", description: "Calls meteors down at a whisper.", cost: 6, type: "creature", race: "elf", attack: 7, defence: 4, image_url: "/cards/starfall-invoker.png" },
  { id: 42, name: "Archon of the Wood", description: "Apex of elven warrior tradition.", cost: 7, type: "creature", race: "elf", attack: 7, defence: 6, image_url: "/cards/archon-of-the-wood.png" },
  { id: 43, name: "The Ageless Queen", description: "She remembers when these mountains were plains.", cost: 8, type: "creature", race: "elf", attack: 8, defence: 8, image_url: "/cards/ageless-queen.png" },

  // ── DWARVES ─────────────────────────────────────────────
  { id: 44, name: "Mine Runner", description: "Fastest in the tunnels.", cost: 1, type: "creature", race: "dwarf", attack: 1, defence: 2, image_url: "/cards/mine-runner.png" },
  { id: 45, name: "Anvil Apprentice", description: "Still learning the craft.", cost: 1, type: "creature", race: "dwarf", attack: 2, defence: 1, image_url: "/cards/anvil-apprentice.png" },
  { id: 46, name: "Tunnel Fighter", description: "Prefers tight quarters.", cost: 2, type: "creature", race: "dwarf", attack: 2, defence: 3, image_url: "/cards/tunnel-fighter.png" },
  { id: 47, name: "Clan Beserker", description: "Anger fuels every swing.", cost: 2, type: "creature", race: "dwarf", attack: 4, defence: 1, image_url: "/cards/clan-berserker.png" },
  { id: 48, name: "Runecarver", description: "Etches power into stone and steel.", cost: 3, type: "creature", race: "dwarf", attack: 2, defence: 5, image_url: "/cards/runecarver.png" },
  { id: 49, name: "Iron Shield Bearer", description: "A wall with legs.", cost: 3, type: "creature", race: "dwarf", attack: 1, defence: 6, image_url: "/cards/iron-shield-bearer.png" },
  { id: 50, name: "Forgemaster", description: "Hammers that never miss.", cost: 3, type: "creature", race: "dwarf", attack: 4, defence: 3, image_url: "/cards/forgemaster.png" },
  { id: 51, name: "Stoneguard", description: "Hewn from the mountain itself.", cost: 4, type: "creature", race: "dwarf", attack: 3, defence: 6, image_url: "/cards/stoneguard.png" },
  { id: 52, name: "Axe Thrower", description: "Never misses at range.", cost: 4, type: "creature", race: "dwarf", attack: 5, defence: 3, image_url: "/cards/axe-thrower.png" },
  { id: 53, name: "Deep Delver", description: "Found things in the dark best left unfound.", cost: 4, type: "creature", race: "dwarf", attack: 4, defence: 4, image_url: "/cards/deep-delver.png" },
  { id: 54, name: "King Under the Mountain", description: "His word is law beneath the stone.", cost: 6, type: "creature", race: "dwarf", attack: 6, defence: 7, image_url: "/cards/king-under-mountain.png" },

  // ── ORCS ────────────────────────────────────────────────
  { id: 55, name: "Orc Runt", description: "Small for an orc. Still terrifying.", cost: 1, type: "creature", race: "orc", attack: 2, defence: 1, image_url: "/cards/orc-runt.png" },
  { id: 56, name: "War Drummer", description: "The beat drives warriors to madness.", cost: 2, type: "creature", race: "orc", attack: 2, defence: 2, image_url: "/cards/war-drummer.png" },
  { id: 57, name: "Bone Crusher", description: "The name is literal.", cost: 2, type: "creature", race: "orc", attack: 3, defence: 2, image_url: "/cards/bone-crusher.png" },
  { id: 58, name: "Warchief's Guard", description: "Dies before his chief.", cost: 3, type: "creature", race: "orc", attack: 3, defence: 4, image_url: "/cards/warchiefs-guard.png" },
  { id: 59, name: "Pit Fighter", description: "Never lost a fight in the pits.", cost: 3, type: "creature", race: "orc", attack: 5, defence: 2, image_url: "/cards/pit-fighter.png" },
  { id: 60, name: "Troll Kin", description: "Half orc. Half something worse.", cost: 4, type: "creature", race: "orc", attack: 4, defence: 5, image_url: "/cards/troll-kin.png" },
  { id: 61, name: "Orcish Marauder", description: "Leaves nothing standing.", cost: 4, type: "creature", race: "orc", attack: 6, defence: 3, image_url: "/cards/orcish-marauder.png" },
  { id: 62, name: "Gore Shaman", description: "Paints war masks with enemy blood.", cost: 5, type: "creature", race: "orc", attack: 5, defence: 4, image_url: "/cards/gore-shaman.png" },
  { id: 63, name: "Warchief Grommak", description: "United the clans under one banner.", cost: 7, type: "creature", race: "orc", attack: 8, defence: 6, image_url: "/cards/warchief-grommak.png" },

  // ── UNDEAD ──────────────────────────────────────────────
  { id: 64, name: "Risen Peasant", description: "Death gave it purpose it never had in life.", cost: 1, type: "creature", race: "undead", attack: 1, defence: 2, image_url: "/cards/risen-peasant.png" },
  { id: 65, name: "Shambling Corpse", description: "Slow, relentless, unkillable.", cost: 1, type: "creature", race: "undead", attack: 1, defence: 3, image_url: "/cards/shambling-corpse.png" },
  { id: 66, name: "Grave Robber", description: "Takes from those who no longer need it.", cost: 2, type: "creature", race: "undead", attack: 2, defence: 2, image_url: "/cards/grave-robber.png" },
  { id: 67, name: "Skeleton Archer", description: "Bone fingers, perfect aim.", cost: 2, type: "creature", race: "undead", attack: 3, defence: 1, image_url: "/cards/skeleton-archer.png" },
  { id: 68, name: "Plague Bearer", description: "You don't want to know what it carries.", cost: 3, type: "creature", race: "undead", attack: 2, defence: 4, image_url: "/cards/plague-bearer.png" },
  { id: 69, name: "Wight Knight", description: "A fallen knight, risen in darkness.", cost: 3, type: "creature", race: "undead", attack: 4, defence: 3, image_url: "/cards/wight-knight.png" },
  { id: 70, name: "Banshee", description: "Her scream ends battles before they begin.", cost: 4, type: "creature", race: "undead", attack: 3, defence: 4, image_url: "/cards/banshee.png" },
  { id: 71, name: "Death Knight", description: "Serves the dark throne eternally.", cost: 5, type: "creature", race: "undead", attack: 6, defence: 5, image_url: "/cards/death-knight.png" },
  { id: 72, name: "Lich", description: "Traded mortality for power.", cost: 6, type: "creature", race: "undead", attack: 6, defence: 6, image_url: "/cards/lich.png" },
  { id: 73, name: "The Undying King", description: "Has died seven times. Always returns.", cost: 8, type: "creature", race: "undead", attack: 9, defence: 7, image_url: "/cards/undying-king.png" },

  // ── DRAGONS ─────────────────────────────────────────────
  { id: 74, name: "Whelpling", description: "Small flame, big attitude.", cost: 2, type: "creature", race: "dragon", attack: 2, defence: 2, image_url: "/cards/whelpling.png" },
  { id: 75, name: "Young Drake", description: "Growing into its power.", cost: 3, type: "creature", race: "dragon", attack: 4, defence: 2, image_url: "/cards/young-drake.png" },
  { id: 76, name: "Ember Drake", description: "Trails fire wherever it flies.", cost: 4, type: "creature", race: "dragon", attack: 5, defence: 3, image_url: "/cards/ember-drake.png" },
  { id: 77, name: "Stone Wyrm", description: "Ancient. Patient. Territorial.", cost: 5, type: "creature", race: "dragon", attack: 5, defence: 6, image_url: "/cards/stone-wyrm.png" },
  { id: 78, name: "Frostbreath Serpent", description: "Its breath turns armies to statues.", cost: 6, type: "creature", race: "dragon", attack: 6, defence: 6, image_url: "/cards/frostbreath-serpent.png" },
  { id: 79, name: "Elder Dragon", description: "Old enough to remember the first war.", cost: 7, type: "creature", race: "dragon", attack: 8, defence: 7, image_url: "/cards/elder-dragon.png" },
  { id: 80, name: "Draconlord Vaelthar", description: "The sky darkens when he spreads his wings.", cost: 9, type: "creature", race: "dragon", attack: 10, defence: 9, image_url: "/cards/draconlord-vaelthar.png" },

  // ── BEASTS ──────────────────────────────────────────────
  { id: 81, name: "Swamp Rat", description: "Feral and relentless.", cost: 1, type: "creature", race: "beast", attack: 1, defence: 1, image_url: "/cards/swamp-rat.png" },
  { id: 82, name: "Cave Spider", description: "Venom does the work.", cost: 1, type: "creature", race: "beast", attack: 2, defence: 1, image_url: "/cards/cave-spider.png" },
  { id: 83, name: "War Boar", description: "Charges without hesitation.", cost: 2, type: "creature", race: "beast", attack: 3, defence: 2, image_url: "/cards/war-boar.png" },
  { id: 84, name: "Stone Bear", description: "Shrugs off arrows like raindrops.", cost: 3, type: "creature", race: "beast", attack: 3, defence: 5, image_url: "/cards/stone-bear.png" },
  { id: 85, name: "Razorwing Hawk", description: "Talons that cut through armour.", cost: 3, type: "creature", race: "beast", attack: 4, defence: 2, image_url: "/cards/razorwing-hawk.png" },

  // ── SPELLS ──────────────────────────────────────────────
  { id: 86, name: "Fireball", description: "Deals damage to a target.", cost: 3, type: "spell", image_url: "/cards/fireball.png" },
  { id: 87, name: "Lightning Bolt", description: "Fast and precise.", cost: 2, type: "spell", image_url: "/cards/lightning-bolt.png" },
  { id: 88, name: "Healing Light", description: "Restores health to your champion.", cost: 2, type: "spell", image_url: "/cards/healing-light.png" },
  { id: 89, name: "Stone Skin", description: "Hardens a creature's defence.", cost: 1, type: "spell", image_url: "/cards/stone-skin.png" },
  { id: 90, name: "Battle Cry", description: "Emboldens an ally.", cost: 2, type: "spell", image_url: "/cards/battle-cry.png" },
  { id: 91, name: "Smite", description: "Holy damage, no questions asked.", cost: 3, type: "spell", image_url: "/cards/smite.png" },
  { id: 92, name: "Shadow Step", description: "Move a creature out of harm's way.", cost: 2, type: "spell", image_url: "/cards/shadow-step.png" },
  { id: 93, name: "Frost Nova", description: "Freezes an enemy in place.", cost: 4, type: "spell", image_url: "/cards/frost-nova.png" },
  { id: 94, name: "Arcane Surge", description: "Raw magical energy, uncontrolled.", cost: 5, type: "spell", image_url: "/cards/arcane-surge.png" },
  { id: 95, name: "Raise Dead", description: "Return a creature from the graveyard.", cost: 4, type: "spell", image_url: "/cards/raise-dead.png" },
  { id: 96, name: "Counterspell", description: "Deny the enemy their play.", cost: 3, type: "spell", image_url: "/cards/counterspell.png" },
  { id: 97, name: "Draw Two", description: "Fortune favours the prepared.", cost: 2, type: "spell", image_url: "/cards/draw-two.png" },
  { id: 98, name: "Earthquake", description: "Damages all creatures on the board.", cost: 6, type: "spell", image_url: "/cards/earthquake.png" },
  { id: 99, name: "Holy Nova", description: "Purging light in all directions.", cost: 5, type: "spell", image_url: "/cards/holy-nova.png" },
  { id: 100, name: "Dark Pact", description: "Power at a terrible cost.", cost: 1, type: "spell", image_url: "/cards/dark-pact.png" },
];