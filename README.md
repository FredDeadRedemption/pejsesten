# pejsesten

A card game built around knowing rather than drawing. Low randomness, small decks, and a resource that buys certainty instead of board.

---

## Philosophy

- No (mana) **ramping**, nothing remotely *rampesque* like jade golems.
- No extra turns.
- No excessively big mana cheats, keep it for combo style decks.
- No stacking effects on mana cheats.
- No random targets unless spell is being **replayed**.
- Discourage RnG. 
- Encourage **disruption** and **hard counters**.
- Reward player interaction with mechanics like **Counter** and **Secret**. 

---

## Deckbuilding

- 25 cards per deck.
- Up to 3 copies of any card.

Smaller decks with 3-ofs are the main anti-RnG lever: a specific card shows up by turn 7 about 65% of the time, against 42% for a 30 card deck with 2-ofs. The plan you built is the plan you draw, so losses land on decisions instead of the shuffle.

Watch for: at 3 copies a legal deck is only 9 distinct cards, so the metagame narrows faster. Drop to 2 copies if it gets stale.

---

## Game Modes

### Single Match

- One deck, one game.
- The default mode. Casual and ladder play.

### Crucible

Best of 3 with a deck roster. The competitive mode.

- Bring 3 decks.
- Both rosters are revealed at the start of the match.
- Each player bans 1 of the opponent's 3 decks.
- To win the match, win one game with **each** of your 2 remaining decks.
- After a win, that deck is retired and you must switch.
- After a loss, you may switch decks or keep playing the same one.
- Match ends at 2 wins, so it runs 2 or 3 games.

No sideboarding. The roster is the sideboard.

Rewards range over one-deck mastery, and the loser always gets to adapt while the winner is forced off their working deck. 25 card decks are what make a 3 deck roster reasonable to build and to bring.

---

## Embers and Actions

An **action** is a move any player can make on their turn, at any time, paid for with **embers**. Actions are not cards and are never drawn. The menu is identical for both players, so any difference in outcome is decision quality.

### Economy

- Start with 1 ember, gain 1 at the start of your turn, cap 5.
- The cap is 5 because that is exactly enough to fire a 2 cost and a 3 cost action in the same turn. A double action turn costs three turns of restraint, and two 3 cost slots can never double up at all.
- Ember totals are public.

### Loadout

- **Bury** is in every loadout for free.
- Pick **2** more actions when you build the deck.
- Loadouts are revealed at the start of the match, with the deck lists.
- In Crucible each of your 3 decks has its own loadout.

### The Rule: deck control only

Actions may only touch decks, hands and graveyards. Never the board, never health.

Board effects belong on cards. If a ping, a heal, a token or armour is always available to everyone, every cheap card that does those things becomes unprintable and the menu quietly becomes the real game. Deck manipulation is something cards do badly, so there is no overlap: mana buys board, ember buys certainty.

Aggro is not left out by this. Digging for gas and burying dead late game cards is less flashy than a ping, not less useful.

### Pool

Your own deck:

- ⬜ Bury (1) — put a card from hand on the bottom of your deck *(free in every loadout)*
- ⬜ Swap (1) — exchange a card in hand for the top card of your deck
- ⬜ Foresee (1) — look at your top card, leave it or bottom it
- ⬜ Seal (2) — put a card from hand on top of your deck
- ⬜ Sift (2) — look at the top 3, bury any number of them
- ⬜ Unearth (2) — draw the bottom card of your deck
- ⬜ Reclaim (3) — put a minion from your graveyard on the bottom of your deck
- ⬜ Draw (3, once per turn) — draw a card

Opponent's deck:

- ⬜ Exile (1) — remove a card in either graveyard from the game
- ⬜ Snuff (3) — your opponent's next draw comes from the bottom of their deck
- ⬜ Pry (3) — your opponent buries a card of their choice

Bury and Unearth are the pair the set is built around: the bottom of your deck becomes a stash you stocked yourself, and the skill is remembering the order.

Watch Draw. It is card advantage wearing a deck control costume. Keep it at 3 and once per turn, and if it turns up in nearly every loadout it is underpriced rather than popular.

---

## Concepts

### Wager

A card played face up that names something the opponent must not do, and what it costs them if they do it.

> **Wager:** If you attack my hero this turn, draw 2 and your opponent discards 1.

Everything is public: the trigger, the effect, the price. Where a Secret hides the trigger and an Omen hides which trigger was chosen, a Wager hides nothing at all. The opponent is not guessing, they are choosing, and the card wins either way — they either pay the effect or they give up the line they wanted.

The whole skill is pricing. Too cheap to avoid and the card does nothing; too painful and they simply eat it. The interesting band is where the answer is genuinely hard.

Needs no response step in the engine: it resolves on the active player's own action, so it fits the current turn model as it stands.

### Sigils

A quest archetype. Three or four sigil cards that are played face up and stay on the board; when the last one lands, a large payoff fires.

The sigils are not drawn, they are fetched. Four singletons in a 25 card deck, seeing 13 cards by turn 10, is roughly a 6% chance of holding all four — nobody would build that deck in a normal game. Sift, Unearth, Seal, Draw and Reclaim are what make the archetype playable at all, so it only exists because of the action system.

Sigils must sit face up on the board, never assembled in hand. Visible progress means the opponent can count the pieces, knows what the last one means, and gets to choose between racing it and attacking it. Assembly in hand is a solitaire check nobody can interact with.

This is also the answer to quests: a sigil on the board **is** the questline, progress bar included. One mechanic, not two.

Sizing: four distinct singletons is brutal even with digging. Either 3 pieces, or 4 pieces at 2 copies each.

The counterplay is not removal, it is Exile, Snuff and Pry. Against a sigil deck you attack the deck, not the board.

### Escalate

Each copy of this card you play in a game is stronger than the last.

> **Bolt** — Deal 2 damage. **Escalate:** +1 damage for each copy already played this game.

This keyword barely functions in Hearthstone and works here, for a reason already built into the deckbuilding rules. At 25 cards with 3 copies you expect to have seen about 1.6 copies of a given card by turn 10; at 30 cards with 2 copies it is about 0.9. Drawing your second and third copy is normal here and rare there.

It also rewards running a full playset instead of splashing singletons, which pushes decks toward a focused plan — the same direction as everything else in the game.

Watch for: escalating removal is the dangerous case, since the third copy answers anything. Escalate on damage wants a ceiling; escalate on bodies and value effects does not.

### Fuse

A minion enters with a visible countdown that drops by 1 at the start of each of your turns. At zero it fires a large effect.

> **Powder Keg** — 0/4. **Fuse 3:** deal 4 damage to all enemy minions.

The number is public from the moment it lands, so nobody is guessing. The opponent knows exactly how many turns they have and chooses between killing it, racing it, or bouncing it back to hand to reset the timer. That is a real decision every turn it sits there, and it costs no hidden information to create.

Fuse minions should be weak bodies. The tension comes from them being answerable, not from them being hard to kill.

Open question: whether anything may speed a fuse up or slow one down. Effects that shift a timer are the obvious place for a combo deck to break it, so price them like combo pieces.

---

### Bequeath *(experimental)*

When this minion dies, the next minion you play this game inherits its keywords and buffs.

> **Torchbearer** — 2/2 Guard, Ward. **Bequeath.**

You choose the heir by sequencing, so the value chain is entirely in your hands and nothing is random. Hearthstone has no mechanic that passes anything forward from a dead minion.

Flagged experimental because the ceiling is unknown: stacking several Bequeath deaths into one recipient could hand a single body four keywords, and it is not obvious yet whether inherited buffs should stack, overwrite, or be capped at one donor. Needs play before it is trusted.

---

## Evergreen Keywords

### In

- ✅ Battlecry --> Fanfare
- ✅ Charge --> Blitz (currently resolves as Rush: attacks minions on arrival, not the hero)
- ⬜ Choose One
- ✅ Combo
- ⬜ Corpse
- ✅ Deathrattle --> Deathwish
- ⬜ Discover
- ✅ Divine Shield --> Ward
- ⬜ Freeze
- ✅ Lifesteal
- ✅ Poisonous
- ⬜ Rush (already folded into Blitz, see above)
- ⬜ Secret (would love secrets)
- ⬜ Silence
- ✅ Stealth
- ✅ Taunt --> Guard
- ✅ Tradeable

### Maybe

- ⬜ Outcast (is it really that sophisticated?)
- ⬜ Overheal
- ⬜ Reborn (hmm don't like that much)
- ⬜ Spell Damage

### Out

- ⬜ Overload (too ramp like, don't like it)
- ⬜ Windfury (not cool)

---

## Abilities

### In

- ⬜ Choose multiple
- ⬜ Counter
- ⬜ Finale
- ⬜ Forge
- ⬜ Miniaturize (yes please)
- ⬜ Quest (hard to implement, but is cool as fuck)
- ⬜ Questline
- ✅ Quickdraw
- ✅ Return to hand
- ✅ Add copy to hand
- ✅ Cost (n) less
- ✅ Is Holding (effect fires only while a matching card is in hand)
- ⬜ Revive
- ⬜ Start of Game
- ⬜ Twinspell

### Maybe

- ⬜ Adapt (would be cool, maybe just as bonus effect)
- ⬜ Corrupt
- ⬜ Dredge
- ⬜ Echo
- ⬜ Excavate
- ⬜ Fabled
- ⬜ Frenzy
- ⬜ Honorable Kill
- ⬜ Immune (maaayybe, but ONLY for heroes)
- ⬜ Infuse
- ⬜ Kindred
- ⬜ Manathirst
- ⬜ Overkill
- ⬜ Passive (might be cool instead of hero powers)
- ⬜ Recruit
- ⬜ Shatter
- ⬜ Sidequest
- ⬜ Spare Part
- ⬜ Spellburst
- ⬜ Temporary
- ⬜ Tourist

### Out

- ⬜ Casts When Drawn (for meget rng)
- ⬜ Gigantify (just don't like it)
- ⬜ Herald
- ⬜ Imbue (not sure I will have hero power system yet)
- ⬜ Inspire (this was shit)
- ⬜ Invoke (no galakrond, cool idea tho)
- ⬜ Jade Golem (too boring, too ramplike)
- ⬜ Lackey (cool idea, dont like lackeys tho)
- ⬜ Magnetic (too much charge like)
- ⬜ Mega-Windfury (nooo)
- ⬜ Rewind (no random effects means no rewind)
- ⬜ Starship (hate this)
- ⬜ Summoned When Drawn (too much rng)
- ⬜ Titan (don't like this, maybe check out if implementing everything else)

---

## Attributes

### In

- ✅ Buff
- ⬜ Can't attack
- ⬜ Cast spell
- ⬜ Change cost type
- ⬜ Copy
- ✅ Damage
- ✅ Destroy
- ✅ Draw
- ✅ Heal
- ✅ Summon
- ⬜ Upgradable

### Maybe

- ⬜ Deckbuilding effect
- ⬜ Discard effect
- ⬜ Elusive (maybe yea)
- ⬜ Enrage
- ⬜ Equip
- ⬜ Force attack (hmm don't particularly like this)
- ⬜ Generate
- ⬜ Joust (this is so fun, a bit rng)
- ⬜ Permanent attribute (what?)
- ⬜ Put into battlefield
- ⬜ Put into hand
- ⬜ Replace
- ⬜ Swap effects (maybe, not to be confused with 'Swap' that does something like swap attack and health — don't want that one)
- ⬜ Transform

### Out

- ⬜ Forgetful (no rng)
- ⬜ Keep enchantment (maybe, hard to implement)
- ⬜ Recurring (not sure, seems like a duplicate of earlier effects)
- ⬜ Reopen (don't have locations)
- ⬜ Transform when drawn (what is the point)
- ⬜ Unlimited attacks (sounds too op)