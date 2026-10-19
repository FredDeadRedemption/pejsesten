# Mechanics

Every mechanic the engine implements: the name used here, what it is called in Hearthstone,
Magic or elsewhere, and what it actually does in code. Planned mechanics are listed at the
end with no code column. Source of truth: `shared/src/types.rs`, `server/src/engine.rs`,
`server/src/settings.rs`.

Design intent and the full wish list live in `README.md`; this file is what is built.

---

## Card types

| Name | Equivalent | Code | What it is |
|---|---|---|---|
| Minion | Minion, Creature | `Card::Minion` | Body with attack and defence, sits on the battlefield, can attack |
| Incantation | Spell, Instant/Sorcery | `Card::Incantation` | Resolves on play, goes nowhere afterwards |
| Omen | Secret, Trap | `Card::Omen` | Played face up with a hidden armed trigger, fires on the opponent's turn |
| Hero | Hero, Planeswalker-ish | `Card::Hero` | Picked at deckbuild, never drawn, only holds hp. No hero power |
| Token | Token | `is_token: true` | Summoned only, never in a deck list |

Zones per player: `deck`, `hand`, `battlefield`, `omens`, `graveyard`, plus the hero slot.
Nothing is ever exiled; destroyed minions go to the graveyard.

### Colours and races

Colours are `White`, `Black`, `Green` (deck identity only, no rules attached yet).
Races are `Human`, `Elf`, `Beast`, used by condition filters and auras.

---

## Resources and limits

Prod values, with the dev profile in brackets where it differs (`server/src/settings.rs`).

| Thing | Value | Note |
|---|---|---|
| Starting mana | 1 (dev 10) | Second player starts one lower and catches up on their first turn |
| Mana per turn | +1, cap 10 | Refills to the new base each turn. No ramp, ever |
| Starting hand | 3 (dev 7) | Second player gets one extra card instead of a coin |
| Hand cap | 9 | Cards that arrive at a full hand are dropped, not discarded to the graveyard |
| Board cap | 5 minions | Playing over the cap is rejected; a summon over the cap is skipped |
| Omen cap | 3 armed | Playing over the cap is rejected |
| Embers | start 1, +1 per turn, cap 5 | Tracked and shown, but nothing spends them yet (see Actions, below) |
| Mulligan | on | Both hands, replacements drawn before the returned cards are shuffled back |

---

## Keywords on minions

`MinionAttribute` in `shared/src/types.rs`, resolved in `engine.rs`.

| Name | Equivalent | Code | Effect |
|---|---|---|---|
| Blitz | Charge / Rush, Haste | `Charge` | Arrives unexhausted, so it can attack the turn it lands. It still cannot hit the enemy hero until it has survived a turn, so it currently plays as Rush |
| Guard | Taunt, Lure | `Guard` | While a non-stealthed Guard is on a board, attacks into that board must pick a Guard. A stealthed attacker ignores Guards entirely |
| Ward | Divine Shield, Totem Armour | `Ward` | Absorbs the next damage event whole, then breaks. Applies to combat and to effect damage. Named Ward here, which is a different keyword in MTG |
| Lifesteal | Lifesteal, Lifelink | `Lifesteal` | Damage dealt by this minion's attack heals your hero for the same amount. Only counts if the hit landed, so a broken Ward heals nothing |
| Poisonous | Poisonous, Deathtouch | `Poisonous` | Any damage this minion lands in combat marks the target doomed, and doomed minions die in the next death check |
| Stealth | Stealth | `Stealth` | The enemy cannot attack it or pick it as a target, and it is skipped when checking for Guards. Attacking breaks it. Auto-targeted effects and friendly targeting still reach it |
| Tradeable | Tradeable | `Tradeable` | 1 mana: put the card on the bottom of your deck and draw one. Also exists for incantations (`IncantationAttribute::Tradeable`) |

Lifesteal also exists as a flag on an effect (`Effect::Damage { lifesteal: true }`), used by
incantations that drain.

---

## Triggers

`Trigger` in `shared/src/types.rs`.

| Name | Equivalent | Code | Fires when |
|---|---|---|---|
| Fanfare | Battlecry, ETB | `OnPlay` | The card is played from hand, after the body lands. Also fires on a replay |
| Deathwish | Deathrattle, Dies trigger | `OnDeath` | The minion leaves the battlefield for the graveyard. Queued and resolved before the next death check |
| Quickdraw watcher | "Whenever you cast a spell", Magecraft | `OnQuickdraw` | Once per card played that fired a Quickdraw, on the friendly board |
| Omen watcher | Secret payoff | `OnOmenFired` | After one of your omens resolves. Checked on both boards, whoever owned the omen |

---

## Conditions on abilities

`Requirement`. An ability whose requirements are unmet simply does not fire; the rest of the
card still resolves.

| Name | Equivalent | Code | True when |
|---|---|---|---|
| Combo | Combo | `Combo` | You already played a card this turn |
| Quickdraw | no direct equivalent; nearest is Outcast | `Quickdraw` | The card reached your hand this turn, by any route: turn draw, trade, bounce, or fetched by an effect |
| Is Holding | "if you're holding a Dragon" | `IsHolding { filters }` | A minion card in your hand matches the filters |
| Controls another | "if you control a ..." | `ControlsOther { filters }` | Another friendly minion matches the filters |
| Counter at least | "if you've cast 3 spells" | `CounterAtLeast { counter, amount }` | A board tally has reached the amount. Only tally so far: `QuickdrawsPlayed` |
| Arrival played | new | `ArrivalPlayedThisTurn` | You already played a card this turn that had arrived this turn |

Filters (`Condition`) are `IsRace { race }` and `HasAttribute { attribute }`.

---

## Effects

`Effect`. Every effect carries a `TargetSpec`: mode (`Targeted`, `Auto`, `SelfOnly`), side
(`Friendly`, `Enemy`, `All`), entity type (`Minion`, `Hero`, `All`), and filters.

| Name | Equivalent | Code | Effect |
|---|---|---|---|
| Buff | Buff, +1/+1 counters | `Buff` | Permanent attack and defence change, stored separately from base stats |
| Damage | Damage | `Damage` | Adds damage to the target. Optional `lifesteal` heals your hero for the total dealt |
| Heal | Heal, Lifegain | `Heal` | Removes damage taken, never above max defence |
| Draw | Draw | `Draw` | Draws cards, with an optional follow-up applied to what was drawn |
| Return to hand | Bounce / Sap, Unsummon | `ReturnToHand` | Returns a minion to its owner's hand, fully reset, with an optional flat cost reduction |
| Destroy | Destroy | `Destroy` | Marks the target doomed; it dies in the next death check regardless of defence. Heroes are immune |
| Summon | Summon, Create token | `Summon` | Puts n copies of a minion card id on your board. Arrives exhausted unless it has Blitz. Stops at the board cap |
| Reverse deck | new | `ReverseDeck` | Flips the draw order of your own deck |
| Generate omen | "add a random Secret" | `AddRandomOmen` | Adds a random collectible omen to your hand |
| Replay | Tess Greymane | `ReplayQuickdraws` | Recasts every Quickdraw card you landed this game, free, from nowhere. Targets are rolled at random, the one place random targeting is allowed. A replayed minion with no board room still fires its ability |

Follow-ups on a draw (`FollowUpEffect`), filtered by `card_must_match`:

| Name | Equivalent | Code | Effect |
|---|---|---|---|
| Cost (n) less | "costs (2) less" | `Discount { scaled_amount }` | Flat or scaled discount written onto the drawn card |
| Add a copy | Copy | `Copy { copy_amount }` | Puts n copies of the drawn card in hand, each a fresh entity |

---

## Passives and auras

`PassiveEffect`. A passive is live while a minion carrying it is on the board; it never
resolves on its own and has no trigger.

| Name | Equivalent | Code | Effect |
|---|---|---|---|
| Aura | Lord / anthem, "Your other X have +1/+1" | `Aura { filters, attack, defence }` | Adds stats to every friendly minion matching the filters, itself included if it matches. Recomputed, never stored, so it vanishes with the source |
| Quickdraw doubling | Brann Bronzebeard | `QuickdrawTriggersTwice` | Quickdraw abilities resolve twice. The tally still counts one |

Derived stats are rebuilt from base plus buffs plus auras minus damage, in
`refresh_stats`, which only runs inside `check_for_deaths`.

---

## Cost modification

| Name | Equivalent | Code | Effect |
|---|---|---|---|
| Discount | "costs (n) less" | `discount` on the card entity | A flat reduction written onto one card in hand, by a draw follow-up or by a bounce |
| Cost scaling | Arcane Giant, Affinity | `cost_scaling: ScaledAmount` | Printed on the card, recomputed in hand as the scale moves. Scales: `MinionsOnBoard`, `Counter { QuickdrawsPlayed }` |

Final hand cost is `base_cost - discount - scaled`, floored at 0, rebuilt in
`refresh_costs` whenever the hand or a scale moves.

---

## Omens

An omen is played face up for its cost, and the player picks one of its three printed
triggers. The opponent sees the card and its effect, never which trigger was armed
(`hide_armed_triggers`). It fires at most once and is consumed. Every omen effect must be
auto-targeted, because it resolves on the opponent's turn with nobody to pick a target.
An omen firing cannot arm or fire another omen, so death cascades cannot chain the zone.

| Trigger | Fires when |
|---|---|
| `EnemyPlaysMinion` | After the enemy plays a minion |
| `EnemyPlaysIncantation` | After the enemy plays an incantation |
| `EnemyAttacksHero` | After your hero is attacked |
| `EnemyAttacksMinion` | After one of your minions is attacked |
| `FriendlyMinionDies` | After one of your minions dies |
| `EnemyEndsTurn` | When the enemy ends their turn |
| `EnemyEndsTurnWithoutAttacking` | When the enemy ends their turn without attacking |
| `EnemyBoardReachesThree` | After the enemy has three or more minions |

---

## Combat and turn rules

| Rule | Equivalent | Behaviour |
|---|---|---|
| Exhaustion | Summoning sickness | A minion arrives exhausted unless it has Blitz, and refreshes at the start of your turn. Attacking exhausts it |
| Rush window | Rush | A minion cannot attack the enemy hero until it has survived to one of your turns (`turns_on_board > 0`) |
| Retaliation | Combat damage | Attacker and defender damage each other simultaneously. A 0 attack defender deals nothing and does not break the attacker's Ward |
| Guard check | Taunt | Attacks must pick a Guard while a visible one exists on the defending board |
| Death check | State-based actions | After every action: refresh stats, remove anything at 0 defence or doomed, queue its Deathwishes, repeat until nothing dies |
| Trade | Tradeable | 1 mana, card to the bottom of the deck, draw one. Needs mana and a non-empty deck |
| Mulligan | Mulligan | Pick any number of cards, replacements are drawn first, then the returned cards are shuffled back |

---

## Planned, not implemented

From `README.md`. Nothing below exists in code.

### Actions and embers

Embers accrue already, but there is no action menu and nothing to spend them on. The design:
a free **Bury** plus two chosen actions per deck, costing 1 to 3 embers, allowed to touch
decks, hands and graveyards only, never the board or health. Pool: Bury, Swap, Foresee,
Seal, Sift, Unearth, Reclaim, Draw, Exile, Snuff, Pry.

### Concepts

| Name | Equivalent | Intent |
|---|---|---|
| Wager | no equivalent | Face up card naming something the opponent must not do, and the price if they do. Nothing hidden: they choose, and the card pays either way |
| Sigils | Quest / Questline | Three or four face-up cards that assemble on the board for a large payoff. Fetched with actions, not drawn |
| Escalate | Escalate-like, see also Spellburst | Each copy of the card you play this game is stronger than the last |
| Fuse | Countdown / Saga | A minion with a public countdown that drops each of your turns and fires at zero |
| Triad | Quest folded into one card | A single payoff when you play the third copy of a card in a game |
| Bequeath *(experimental)* | no equivalent | On death, the next minion you play inherits its keywords and buffs |

### Keyword backlog

Wanted: Choose One, Corpse, Discover, Freeze, Rush as its own keyword split from Blitz,
Secret, Silence, Counter, Finale, Forge, Miniaturize, Quest, Questline, Revive,
Start of Game, Twinspell, Choose multiple, Copy, Upgradable, Can't attack, Change cost type.

Under consideration and ruled out: see the Evergreen Keywords, Abilities and Attributes
lists in `README.md`, which carry the reasoning per keyword.
