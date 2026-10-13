# Dawnguard (white)

Hero: **The Magician**, 30 hp. 25 cards, 10 distinct.

A midrange board deck. Cheap sticky bodies, a Guard at every rung of the curve, and two
ways to lift the whole board. Lifesteal pays for the race so the deck can afford to trade
early and still be ahead on life at turn 8. Removal is minion-only, so the plan is always
the board, never the face.

## List

| # | Card | Cost | Body | Text |
|---|------|------|------|------|
| 3 | Shieldmaiden | 1 | 1/1 Human | Ward |
| 2 | Smite | 1 | — | Deal 2 damage. Combo: deal 4 instead |
| 3 | Dawn Guard | 2 | 1/4 Human | Guard |
| 3 | Machine Elf | 2 | 2/2 Elf | Tradeable, Lifesteal |
| 2 | Grove Vigil | 2 | omen | Give all your minions +1 +1 |
| 3 | Standard Bearer | 3 | 2/4 Human | Fanfare: give a friendly minion +1 +1 |
| 2 | Good Friday | 3 | — | Deal 4 damage to a minion |
| 3 | Silverguard Knight | 4 | 3/4 Human | Guard. Deathwish: draw a card |
| 2 | Herald of the Sun | 5 | 5/5 Elf | Lifesteal |
| 2 | Gargoyle Vigil | 6 | 5/7 | Guard |

Curve: 5 at one, 8 at two, 5 at three, 3 at four, 2 at five, 2 at six.

## How it plays

Turns 1-3 you put down a body every turn and let the opponent come to you: Shieldmaiden
eats a hit for free, Dawn Guard's 4 defence survives two small attacks, Machine Elf turns
any trade into life. Turns 4-6 Silverguard Knight is the core card - a Guard that replaces
itself, so removing it costs the opponent a card and gains you one. From turn 5 the two
top-end bodies are bigger than anything that can trade with them cleanly.

Grove Vigil is the swing. Armed on *enemy ends turn* it is a guaranteed +1 +1 on a wide
board; armed on *enemy plays a minion* it pre-pays for a trade; armed on *enemy attacks a
minion* it saves a body mid-combat. The opponent sees the card text and still has to guess
which of the three it is, which is the whole point of the mechanic.

Smite and Good Friday are only for minions you cannot beat in combat. Hold them.

## Keeps

Keep any 1 or 2 drop. Keep Standard Bearer with a 1-drop. Throw back Gargoyle Vigil and
Herald of the Sun unless the hand already has two cheap bodies. The deck has 11 cards
costing 1 or 2, so a hand of three sees one about 80% of the time.

## Tuning notes

- **Divine Power** (4, give a friendly minion +4 +4) is in the pool but not the list. It is
  the obvious flex slot: swap it in over the second Good Friday if the board stalls too
  often, out if you keep drawing it with no board.
- Dawn Guard at 1/4 is a point of stats over curve, allowed because 1 attack cannot trade.
  If it feels unkillable rather than annoying, cut it to 1/3.
- Shieldmaiden at 1/1 Ward is the one card deliberately over curve at the 1 slot, same
  reasoning the format uses for any one-mana keyword body.
- Only 5 Elves and no Elf payoff: the race is flavour here. The tribal payoff lives in
  Kennel, on Beasts.

## Cards that need engine work

Designed for this deck, not printable today. Each one names the single missing piece.

- **Lightbearer** — 2, 2/2 Human. *Fanfare: give a friendly minion Ward.*
  Needs an effect that **grants a keyword**. Keywords are read off `MinionCard.attributes`
  (the Guard check in attack validation reads the card, not the entity), so the entity
  needs its own attribute list before anything can hand one out.
- **Reliquary Guard** — 3, 1/4 Human, Guard. *Fanfare: silence an enemy minion.*
  Needs **Silence**: an effect that strips attributes, abilities and buffs from an entity.
- **Martyr's Light** — 4, 3/3 Human. *Deathwish: give a friendly minion +3 +3.*
  Needs **targets on non-OnPlay triggers**. Deathwish effects are queued with
  `target_id: None`, so a `TargetMode::Targeted` spec resolves to nothing. Either a
  deferred choice step, or an auto rule (highest attack, leftmost).
- **Hearthwarden** — 3, 3/3 Human. *Triad: summon a 6/6.*
  Needs a **per-game count of copies played** of a given card id, on the server state.
  Same counter unlocks Escalate.
- **Hold the Line** — 2 incantation. *Wager: if you attack my hero this turn, I draw 2.*
  Needs the **Wager** mechanic: a face-up permanent that watches an enemy action during
  the enemy's turn. The omen plumbing is most of it; what is missing is a face-up card that
  shows its trigger instead of hiding it.
