# pejsesten

A card game built around knowing rather than drawing. Low randomness, small decks, and a resource that buys certainty instead of board.

Two players, one hero each, 25 card decks. Mana buys the board, embers buy information. Everything either player can do is on the table, including the cards in the other deck once the match starts.

---

## Philosophy

- No (mana) **ramping**, nothing remotely *rampesque*.
- No extra turns.
- No excessively mana cheats, smaller cheats kept for combo-style decks.
- No random targets unless spell is being **replayed**.
- Discourage RnG - encourage **disruption** and **hard counters**.
- Reward player interaction with mechanics like **Counter** and **Secret**. 

---

## Docs

| Doc | What is in it |
|---|---|
| [docs/MECHANICS.md](docs/MECHANICS.md) | Every mechanic, built or not, with its Hearthstone or Magic name, what it does, and a status symbol |
| [docs/CONCEPTS.md](docs/CONCEPTS.md) | The designs not built yet: embers and actions, Wager, Sigils, Escalate, Fuse, Triad, Bequeath |
| [docs/GAMEMODES.md](docs/GAMEMODES.md) | Single Match and the Crucible roster format |
| [docs/DECKBUILDING.md](docs/DECKBUILDING.md) | Deck size, copy limits, and why they are what they are |
| [architecture.md](architecture.md) | Workspace layout, how the Rust client runs native and in the browser, build and deploy |
| `deck-*.md` | The decks that exist today: Dawnguard, Gunslinger, Kennel |

---

## Running it

```sh
cargo dev       # server plus native client
cargo dev web   # wasm build, then open http://localhost:3000
```

See [architecture.md](architecture.md) for what those do and how the container is built.
