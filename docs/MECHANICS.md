# Mechanics

Every mechanic this game has or might have: the name used here, what it is called in
Hearthstone, Magic or elsewhere, what it does, and whether it is built.

Built behaviour is read from `shared/src/types.rs`, `server/src/engine.rs` and
`server/src/settings.rs`. The larger unbuilt designs have their own write-ups in
[CONCEPTS.md](CONCEPTS.md).

| Symbol | Meaning |
|---|---|
| ✅ | built and playable |
| ⬜ | wanted, not built |
| 🟡 | maybe, undecided |
| ❌ | ruled out |

---

## Card types

| | Name | Equivalent | Code | What it is |
|---|---|---|---|---|
| ✅ | Minion | Minion, Creature | `Card::Minion` | Body with attack and defence, sits on the battlefield, can attack |
| ✅ | Incantation | Spell, Instant/Sorcery | `Card::Incantation` | Resolves on play, goes nowhere afterwards |
| ✅ | Omen | Secret, Trap | `Card::Omen` | Played face up with a hidden armed trigger, fires on the opponent's turn |
| ✅ | Hero | Hero | `Card::Hero` | Picked at deckbuild, never drawn, only holds hp. No hero power |
| ✅ | Token | Token | `is_token: true` | Summoned only, never in a deck list |
| ⬜ | Secret | Secret | - | Card and trigger both hidden. An omen hides only which of three triggers was armed |
| ❌ | Location | Location | - | No location type, so Reopen has nothing to refresh |

Zones per player: `deck`, `hand`, `battlefield`, `omens`, `graveyard`, plus the hero slot.
Nothing is ever exiled; destroyed minions go to the graveyard.

Colours are `White`, `Black`, `Green`, deck identity only, no rules attached yet.
Races are `Human`, `Elf`, `Beast`, used by condition filters and auras.

---

## Resources and limits

Prod values, with the dev profile in brackets where it differs (`server/src/settings.rs`).

| | Thing | Value | Note |
|---|---|---|---|
| ✅ | Mana | start 1 (dev 10), +1 per turn, cap 10 | Refills to the new base each turn. No ramp, ever. Second player starts one lower and catches up on their first turn |
| ✅ | Embers | start 1, +1 per turn, cap 5 | Tracked and shown, but nothing spends them yet |
| ⬜ | Actions | 1 to 3 embers each | The menu embers are meant to pay for. See Concepts |
| ✅ | Hand | start 3 (dev 7), cap 9 | Second player gets one extra card instead of a coin. Cards that arrive at a full hand are dropped, not sent to the graveyard |
| ✅ | Board | cap 5 minions | Playing over the cap is rejected, a summon over the cap is skipped |
| ✅ | Omens | cap 3 armed | Playing over the cap is rejected |
| ✅ | Mulligan | on | Replacements are drawn before the returned cards are shuffled back |
| ⬜ | Corpse | - | Second resource gained when your own minions die, spent alongside mana |

---

## Keywords on minions

`MinionAttribute` in `shared/src/types.rs`, resolved in `engine.rs`.

| | Name | Equivalent | Code | Effect |
|---|---|---|---|---|
| ✅ | Blitz | Charge / Rush, Haste | `Charge` | Arrives unexhausted, so it can attack the turn it lands. It still cannot hit the enemy hero until it has survived a turn, so it currently plays as Rush |
| ✅ | Guard | Taunt, Lure | `Guard` | While a non-stealthed Guard is on a board, attacks into that board must pick a Guard. A stealthed attacker ignores Guards entirely |
| ✅ | Ward | Divine Shield, Totem Armour | `Ward` | Absorbs the next damage event whole, then breaks. Applies to combat and to effect damage. Named Ward here, which is a different keyword in Magic |
| ✅ | Lifesteal | Lifesteal, Lifelink | `Lifesteal` | Damage this minion deals in combat heals your hero for the same amount. Only if the hit landed, so a broken Ward heals nothing |
| ✅ | Poisonous | Poisonous, Deathtouch | `Poisonous` | Any damage this minion lands in combat marks the target doomed, and doomed minions die in the next death check |
| ✅ | Stealth | Stealth | `Stealth` | The enemy cannot attack it or pick it as a target, and it is skipped when checking for Guards. Attacking breaks it. Auto-targeted effects and friendly targeting still reach it |
| ✅ | Tradeable | Tradeable | `Tradeable` | 1 mana: card to the bottom of your deck, draw one. Also exists on incantations |
| ⬜ | Rush | Rush | - | Attack minions on arrival but not the hero. Already how Blitz resolves, so this is the split of one keyword into two |
| ⬜ | Freeze | Freeze | - | The minion misses its next attack |
| ⬜ | Can't attack | Can't Attack | - | A body that only defends |
| 🟡 | Reborn | Reborn | - | The first death returns it with 1 defence. Not loved |
| 🟡 | Elusive | Elusive | - | Cannot be targeted by spells |
| 🟡 | Immune | Immune | - | Takes no damage. Only ever for heroes, if at all |
| 🟡 | Spell damage | Spell Damage | - | Your incantations deal extra damage |
| 🟡 | Temporary | Temporary | - | Leaves at the end of the turn |
| 🟡 | Permanent attribute | Permanent attribute | - | Intent unclear, parked |
| ❌ | Windfury | Windfury | - | Attacks twice per turn. Not wanted |
| ❌ | Mega-Windfury | Mega-Windfury | - | Attacks four times per turn. Even less wanted |
| ❌ | Unlimited attacks | - | - | No attack cap at all. Too strong to price |
| ❌ | Magnetic | Magnetic | - | Fuses onto another minion, carrying its stats and keywords over. Too close to Blitz stacking |
| ❌ | Forgetful | Forgetful | - | Half the time the attack hits something else. Randomness |
| ❌ | Titan | Titan | - | Three one-shot abilities before it may attack. Maybe worth a look once everything else is in |
| ❌ | Starship | Starship | - | Pieces assemble into one permanent body. Sigils cover assembly better |

---

## Triggers

`Trigger` in `shared/src/types.rs`. When an ability fires.

| | Name | Equivalent | Code | Fires when |
|---|---|---|---|---|
| ✅ | Fanfare | Battlecry, ETB | `OnPlay` | The card is played from hand, after the body lands. Also fires on a replay |
| ✅ | Deathwish | Deathrattle, Dies trigger | `OnDeath` | The minion leaves the battlefield for the graveyard. Queued and resolved before the next death check |
| ✅ | Quickdraw watcher | "Whenever you cast a spell", Magecraft | `OnQuickdraw` | Once per card played that fired a Quickdraw, on the friendly board |
| ✅ | Omen watcher | Secret payoff | `OnOmenFired` | After one of your omens resolves. Checked on both boards, whoever owned the omen |
| ⬜ | Start of game | Start of Game | - | Before the first turn |
| ⬜ | Finale | Finale | - | The card is played with no mana left over |
| 🟡 | Spellburst | Spellburst | - | One shot, the first time you play an incantation |
| 🟡 | Frenzy | Frenzy | - | The first time the minion survives damage |
| 🟡 | Overheal | Overheal | - | The minion is healed past its max defence |
| 🟡 | Honorable kill | Honorable Kill | - | The attack killed with exactly lethal damage |
| 🟡 | Overkill | Overkill | - | The attack dealt more than lethal damage |
| 🟡 | Enrage | Enrage | - | While the minion is damaged |
| ❌ | Inspire | Inspire | - | After using a hero power. No hero powers, and it was weak |
| ❌ | Casts when drawn | Casts When Drawn | - | Fires itself out of the deck. Randomness |
| ❌ | Summoned when drawn | Summoned When Drawn | - | Same, with a body |

---

## Conditions on abilities

`Requirement`. An ability whose requirements are unmet does not fire; the rest of the card
still resolves. Filters (`Condition`) are `IsRace { race }` and `HasAttribute { attribute }`.

| | Name | Equivalent | Code | True when |
|---|---|---|---|---|
| ✅ | Combo | Combo | `Combo` | You already played a card this turn |
| ✅ | Quickdraw | nearest is Outcast | `Quickdraw` | The card reached your hand this turn, by any route: turn draw, trade, bounce, or fetched by an effect |
| ✅ | Is holding | "if you're holding a Dragon" | `IsHolding { filters }` | A minion card in your hand matches the filters |
| ✅ | Controls another | "if you control a ..." | `ControlsOther { filters }` | Another friendly minion matches the filters |
| ✅ | Counter at least | "if you've cast 3 spells" | `CounterAtLeast { counter, amount }` | A board tally reached the amount. Only tally so far: `QuickdrawsPlayed` |
| ✅ | Arrival played | - | `ArrivalPlayedThisTurn` | You already played a card this turn that had arrived this turn |
| 🟡 | Outcast | Outcast | - | The card is the leftmost or rightmost in your hand. Possibly not as clever as it looks |
| 🟡 | Manathirst | Manathirst | - | You have at least X mana crystals |
| 🟡 | Kindred | Kindred | - | A card of the same race was played last turn |
| 🟡 | Corrupt | Corrupt | - | Upgrades in hand once you play a more expensive card |
| 🟡 | Infuse | Infuse | - | Upgrades in hand once X friendly minions have died |
| 🟡 | Joust | Joust | - | Reveal a minion from each deck, the costlier one wins. Fun, but random |

---

## Effects

`Effect`. Every built effect carries a `TargetSpec`: mode (`Targeted`, `Auto`, `SelfOnly`),
side (`Friendly`, `Enemy`, `All`), entity type (`Minion`, `Hero`, `All`), and filters.

| | Name | Equivalent | Code | Effect |
|---|---|---|---|---|
| ✅ | Buff | Buff, +1/+1 counters | `Buff` | Permanent attack and defence change, stored apart from base stats |
| ✅ | Damage | Damage | `Damage` | Adds damage to the target. Optional `lifesteal` heals your hero for the total dealt |
| ✅ | Heal | Heal, Lifegain | `Heal` | Removes damage taken, never above max defence |
| ✅ | Draw | Draw | `Draw` | Draws cards, with an optional follow-up applied to what was drawn |
| ✅ | Return to hand | Bounce / Sap, Unsummon | `ReturnToHand` | Returns a minion to its owner's hand, fully reset, with an optional flat cost reduction |
| ✅ | Destroy | Destroy | `Destroy` | Marks the target doomed, so it dies in the next death check whatever its defence. Heroes are immune |
| ✅ | Summon | Summon, Create token | `Summon` | Puts n copies of a minion card id on your board, exhausted unless it has Blitz. Stops at the board cap |
| ✅ | Reverse deck | - | `ReverseDeck` | Flips the draw order of your own deck |
| ✅ | Generate omen | "add a random Secret" | `AddRandomOmen` | Adds a random collectible omen to your hand |
| ✅ | Replay | Tess Greymane | `ReplayQuickdraws` | Recasts every Quickdraw card you landed this game, free, from nowhere. Targets are rolled at random, the one place random targeting is allowed. A replayed minion with no board room still fires its ability |
| ⬜ | Silence | Silence | - | Wipes a minion's text and buffs |
| ⬜ | Discover | Discover | - | Pick one of three offered cards |
| ⬜ | Choose one | Choose One | - | Pick one of the card's modes on play |
| ⬜ | Choose multiple | - | - | Pick more than one mode, presumably for a price |
| ⬜ | Counter | Counterspell | - | Cancel what the opponent is doing |
| ⬜ | Revive | Resurrect, Reanimate | - | Return a minion from the graveyard to the board |
| ⬜ | Copy | Copy, Clone | - | Duplicate a card or a minion |
| ⬜ | Upgradable | - | - | A card that improves while you hold it |
| ⬜ | Twinspell | Twinspell | - | Casting leaves a copy without Twinspell in hand |
| ⬜ | Quest | Quest | - | Track a goal, pay out when it completes. Sigils are the planned shape, see CONCEPTS.md |
| ⬜ | Questline | Questline | - | A quest in three stages |
| ⬜ | Miniaturize | Miniaturize | - | Play it and get a 1 cost 1/1 version in hand |
| ⬜ | Forge | Forge | - | Pay mana from hand to upgrade the card |
| ⬜ | Change cost type | - | - | A card paid for with something other than mana |
| 🟡 | Adapt | Adapt | - | Choose one of three upgrades from a fixed list |
| 🟡 | Dredge | Dredge | - | Look at the bottom three of your deck, put one on top |
| 🟡 | Excavate | Excavate | - | Adds a treasure to hand, better each time |
| 🟡 | Echo | Echo | - | Replayable all turn, the copy expires at end of turn |
| 🟡 | Recruit | Recruit | - | Summon a minion straight out of your deck |
| 🟡 | Shatter | Shatter | - | On arrival in hand, splits into two halves of itself |
| 🟡 | Spare part | Spare Part | - | Generates cheap filler spells |
| 🟡 | Sidequest | Sidequest | - | A small quest that is not your whole deck plan |
| 🟡 | Passive | Passive | - | A standing effect, floated as the replacement for hero powers |
| 🟡 | Equip | Equip, Weapon | - | Gives the hero a weapon |
| 🟡 | Discard | Discard | - | Throw a card out of hand |
| 🟡 | Force attack | - | - | Make a minion attack. Not much liked |
| 🟡 | Put into battlefield | Put into play, Cheat into play | - | Place a card on the board without paying for it |
| 🟡 | Put into hand | Tutor | - | Fetch a specific card into hand |
| 🟡 | Replace | - | - | Swap one card for another |
| 🟡 | Swap effects | - | - | Trade abilities between two cards. Not stat swapping, which is out |
| 🟡 | Transform | Transform, Polymorph | - | Turn a minion into something else |
| ❌ | Rewind | Rewind | - | Undo a card and replay it for a different random outcome. Needs randomness to be worth anything |
| ❌ | Jade Golem | Jade Golem | - | A golem one size bigger every time. Ramp by another name |
| ❌ | Lackey | Lackey | - | Generated 1/1s with small fanfares. Good idea, wrong flavour |
| ❌ | Invoke | Invoke | - | Upgrade your hero power by playing invokers. Good idea, but no hero powers |
| ❌ | Imbue | Imbue | - | Hero power upgrades. No hero power system |
| ❌ | Herald | Herald | - | Summons limbs of one big body |
| ❌ | Gigantify | Gigantify | - | Play it and get an 8 cost 8/8 version. Miniaturize is the half we want |
| ❌ | Transform when drawn | Transform When Drawn | - | Changes in your hand as you draw it. No point |
| ❌ | Keep enchantment | - | - | Buffs survive a bounce or a transform. Hard to implement cleanly |
| ❌ | Recurring | - | - | Duplicate of effects already listed |
| ❌ | Reopen | Reopen | - | Refresh a used location. There are no locations |

Follow-ups on a draw (`FollowUpEffect`), filtered by `card_must_match`:

| | Name | Equivalent | Code | Effect |
|---|---|---|---|---|
| ✅ | Cost (n) less | "costs (2) less" | `Discount { scaled_amount }` | Flat or scaled discount written onto the drawn card |
| ✅ | Add a copy | Copy | `Copy { copy_amount }` | Puts n copies of the drawn card in hand, each a fresh entity |

---

## Passives and auras

`PassiveEffect`. A passive is live while a minion carrying it is on the board; it never
resolves on its own and has no trigger.

| | Name | Equivalent | Code | Effect |
|---|---|---|---|---|
| ✅ | Aura | Lord / anthem, "Your other X have +1/+1" | `Aura { filters, attack, defence }` | Adds stats to every friendly minion matching the filters, itself included if it matches. Recomputed, never stored, so it vanishes with the source |
| ✅ | Quickdraw doubling | Brann Bronzebeard | `QuickdrawTriggersTwice` | Quickdraw abilities resolve twice. The tally still counts one |

Derived stats are rebuilt from base plus buffs plus auras minus damage, in `refresh_stats`,
which only runs inside `check_for_deaths`.

---

## Cost modification

| | Name | Equivalent | Code | Effect |
|---|---|---|---|---|
| ✅ | Discount | "costs (n) less" | `discount` on the card entity | A flat reduction written onto one card in hand, by a draw follow-up or by a bounce |
| ✅ | Cost scaling | Arcane Giant, Affinity | `cost_scaling: ScaledAmount` | Printed on the card, recomputed in hand as the scale moves. Scales: `MinionsOnBoard`, `Counter { QuickdrawsPlayed }` |
| ❌ | Overload | Overload | - | Borrow mana now, lose it next turn. Too close to ramp |

Final hand cost is `base_cost - discount - scaled`, floored at 0, rebuilt in `refresh_costs`
whenever the hand or a scale moves.

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

| | Rule | Equivalent | Behaviour |
|---|---|---|---|
| ✅ | Exhaustion | Summoning sickness | A minion arrives exhausted unless it has Blitz, and refreshes at the start of your turn. Attacking exhausts it |
| ✅ | Rush window | Rush | A minion cannot attack the enemy hero until it has survived to one of your turns |
| ✅ | Retaliation | Combat damage | Attacker and defender damage each other at once. A 0 attack defender deals nothing and does not break the attacker's Ward |
| ✅ | Guard check | Taunt | Attacks must pick a Guard while a visible one exists on the defending board |
| ✅ | Death check | State-based actions | After every action: refresh stats, remove anything at 0 defence or doomed, queue its Deathwishes, repeat until nothing dies |
| ✅ | Trade | Tradeable | 1 mana, card to the bottom of the deck, draw one. Needs mana and a non-empty deck |
| ✅ | Mulligan | Mulligan | Pick any number of cards, replacements are drawn first, then the returned cards are shuffled back |

---

## Deckbuilding

| | Name | Equivalent | Code | Effect |
|---|---|---|---|---|
| ⬜ | Deck size | - | `MAX_DECK_CARDS` | The design is 25 cards and up to 3 copies. The builder caps at 50 today and does not limit copies |
| 🟡 | Fabled | Fabled | - | Adding the card also puts two uncollectible partners in the deck |
| 🟡 | Tourist | Tourist | - | Lets the deck hold cards from another colour |
| 🟡 | Deckbuilding effect | - | - | Any card that changes what a legal deck is |

---

## Concepts

Larger designs, none of them built. Written up in [CONCEPTS.md](CONCEPTS.md).

| | Name | Equivalent | Effect |
|---|---|---|---|
| ⬜ | Actions and embers | - | A free Bury plus two chosen actions per deck, 1 to 3 embers each, allowed to touch decks, hands and graveyards only, never the board or health. Pool: Bury, Swap, Foresee, Seal, Sift, Unearth, Reclaim, Draw, Exile, Snuff, Pry |
| ⬜ | Wager | - | Face up card naming something the opponent must not do, and the price if they do it. Nothing hidden: they choose, and the card pays either way |
| ⬜ | Sigils | Quest / Questline | Three or four face-up cards that assemble on the board for a large payoff. Fetched with actions, not drawn |
| ⬜ | Escalate | - | Each copy of the card you play this game is stronger than the last |
| ⬜ | Fuse | Saga, Countdown | A minion with a public countdown that drops each of your turns and fires at zero |
| ⬜ | Triad | - | One payoff when you play the third copy of a card in a game |
| ⬜ | Bequeath | - | On death, the next minion you play inherits its keywords and buffs. Experimental, ceiling unknown |
