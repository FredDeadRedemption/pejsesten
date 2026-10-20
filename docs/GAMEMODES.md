# Game modes

Every mode this game has or might have: the name used here, what it is called in
Hearthstone, Magic or elsewhere, and whether it is built.

| Symbol | Meaning |
|---|---|
| ✅ | built and playable |
| ⬜ | wanted, not built |
| ❓ | maybe, undecided |
| ❌ | ruled out |

| | Name | Equivalent | Decks | Bans | Match | What it is |
|---|---|---|---|---|---|---|
| ✅ | Single Match | ladder Bo1, Standard | 1 | - | 1 game | One deck, one game. The default, for casual and ladder play |
| ⬜ | Crucible 3 | Conquest Bo3, HS Masters | 3 | 1 each | Bo3, 2 wins | Win a game with each of your 2 surviving decks. A win retires that deck |
| ❓ | Crucible 4 | Conquest Bo5, old HCT | 4 | 1 each | Bo5, 3 wins | Same rules, one more deck. Longer and a heavier build cost |

No sideboarding in any mode. The roster is the sideboard.

---

## Single Match

- One deck, one game.
- The deck is picked in the lobby and sent with the queue request.

## Crucible

A deck roster with bans, played as conquest: a deck that wins is retired, so every deck on
your roster has to be able to close a game.

- Bring 3 decks (Crucible 3) or 4 (Crucible 4).
- Both rosters are revealed at the start of the match.
- Each player bans 1 of the opponent's decks.
- To win the match, win one game with **each** of your remaining decks.
- After a win, that deck is retired and you must switch.
- After a loss, you may switch decks or keep playing the same one.
- Crucible 3 ends at 2 wins, so it runs 2 or 3 games. Crucible 4 ends at 3 wins, so 3 to 5.

Rewards range over one-deck mastery, and the loser always gets to adapt while the winner is
forced off their working deck. 25 card decks are what make a roster this size reasonable to
build and to bring.

Crucible 3 is the intended competitive mode. Crucible 4 is the same system with a 4th deck;
it is parked because a 4 deck roster is a lot to ask of a new player, and 5 games is a long
match. Nothing in the rules has to change to turn it on.

Status: only Single Match exists. Crucible is design, not code - there is no mode enum, no
roster, and no ban step. See [DECKBUILDING.md](DECKBUILDING.md) for deck rules.
