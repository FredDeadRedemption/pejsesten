# Card Game Engine — Developer Reference

This document covers everything about how the engine works: the type system, game state, sockets, the effect pipeline, and how to write new cards.

---

## Table of Contents

1. [Project Structure](#project-structure)
2. [Game State](#game-state)
3. [Type System](#type-system)
4. [Triggers](#triggers)
5. [Requirements](#requirements)
6. [Effects & TargetSpec](#effects--targetspec)
7. [The Effect Pipeline](#the-effect-pipeline)
8. [Death Checking](#death-checking)
9. [Sockets & Actions](#sockets--actions)
10. [Writing New Cards](#writing-new-cards)
11. [Common Mistakes](#common-mistakes)

---

## Project Structure

```
src/
  lib/
    server/
      engine.ts       — core game logic (playCard, attack, endTurn, effect queue)
      settings.ts     — constants (STARTING_HP, STARTING_HAND_SIZE)
      lib.ts          — helpers (getSourceBoard, getEnemyBoard, switchTurn)
    shared/
      types.ts        — all shared TypeScript types
      cards.ts        — card definitions + deckToCards()
```

---

## Game State

The entire game lives in a single `GameStateServer` object on the server. It is never stored in a database — it lives in memory for the duration of the game.

```typescript
GameStateServer = {
  white: Board,           // white player's board
  black: Board,           // black player's board
  whitePlayerID: string,  // socket ID of the white player
  blackPlayerID: string,  // socket ID of the black player
  whiteTurn: boolean,     // whose turn it is
  turnCount: number,      // increments every endTurn
  cardsPlayedThisTurn: number  // resets each turn, used for combo checks
}
```

Each player has a `Board`:

```typescript
Board = {
  deck: CardEntity[],        // remaining cards
  hand: CardEntity[],        // cards in hand
  graveyard: MinionEntity[], // dead minions
  battlefield: MinionEntity[], // live minions
  hero: Hero,                // { attack, defence }
  baseMana: number,          // increases by 1 each turn
  mana: number               // current spendable mana (restored to baseMana each turn)
}
```

`baseMana` is the "permanent" mana ceiling that grows each turn. `mana` is what the player actually has to spend — effects can add to `mana` without touching `baseMana`, and it resets to `baseMana` at the start of the next turn.

**Active player** — `getSourceBoard(gameState)` returns the board of whoever's turn it is. `getEnemyBoard(gameState)` returns the opponent's board.

---

## Type System

### Cards vs Entities

Cards are the base definition (static data). Entities are cards at runtime with added fields.

```
MinionCard  →  MinionEntity   (adds entityID, attack, defence, cost, exhausted)
IncantationCard → IncantationEntity  (adds entityID, cost)
```

`entityID` is a `crypto.randomUUID()` assigned when the deck is built via `deckToCards()`. This is what the engine uses to find entities at runtime — never use `id` (the card number) for lookups.

### Card Types

```typescript
type Card = MinionCard | IncantationCard
```

Minions stay on the battlefield after being played. Incantations (spells) fire their effects and are consumed — they never touch the battlefield.

### Hero

```typescript
Hero = { attack: number, defence: number }
```

Heroes don't have an `entityID`. They are referenced by the special strings `'heroSelf'` and `'heroEnemy'` in attack data and effect targeting.

---

## Triggers

Triggers define WHEN an ability fires. They are declared on each `Ability` object.

| Trigger | When it fires |
|---|---|
| `onPlay` | When the card is played from hand |
| `onDeath` | When the minion's defence reaches 0 |
| `onDraw` | When a card is drawn (enqueueTrigger only, not per-card yet) |
| `onTurnStart` | (defined, not yet implemented) |
| `onTurnEnd` | (defined, not yet implemented) |
| `onAttack` | (defined, not yet implemented) |
| `onAttacked` | (defined, not yet implemented) |
| `onDamage` | (defined, not yet implemented) |
| `onHeal` | (defined, not yet implemented) |
| `onSummon` | (defined, not yet implemented) |
| `onDiscard` | (defined, not yet implemented) |

**onPlay** — fires for both minions and incantations. For minions, the minion is already on the battlefield when its onPlay effects fire, so AoE damage effects will hit it.

**onDeath** — fires before the minion is removed from the battlefield. The minion is still in the array when effects resolve, but it is excluded from its own `returnToHand` pool via `selfID` filtering.

---

## Requirements

Requirements are an optional array on an `Ability`. ALL requirements must pass for the ability to fire. If `requirements` is omitted, the ability always fires.

```typescript
type Requirement =
  | { type: 'combo' }      // at least one card was played before this one this turn
  | { type: 'firstCard' }  // this is the first card played this turn
```

Requirements are checked with `.every()` so all must pass. The check happens before any effects are enqueued.

**Pattern for conditional abilities** — give the card two abilities with the same trigger. The first has no requirements (always fires). The second has a requirement. This is cleaner than branching inside a single ability.

```typescript
// Pot of Greed: draw 1, Combo: draw 2 total
abilities: [
  { trigger: 'onPlay', effects: [{ type: 'draw', drawAmount: 1 }] },
  { trigger: 'onPlay', requirements: [{ type: 'combo' }], effects: [{ type: 'draw', drawAmount: 1 }] }
]
```

---

## Effects & TargetSpec

An effect is the atomic unit of "something that happens". Each ability has an array of effects.

### Effect Types

**`buff`** — adds attack and defence to targets.
```typescript
{ type: 'buff', attack: 2, defence: 3, targetSpec: ... }
```

**`damage`** — reduces defence of targets.
```typescript
{ type: 'damage', damage: 4, targetSpec: ... }
```

**`draw`** — draws cards into the active player's hand. No targetSpec.
```typescript
{ type: 'draw', drawAmount: 2 }
```

**`returnToHand`** — removes a minion from the battlefield and puts it back in its owner's hand. Resets the minion's attack, defence, cost, and exhausted state. Optional `costReduction` permanently reduces the card's cost (floored at 0).
```typescript
{ type: 'returnToHand', costReduction: 2, targetSpec: ... }
```

---

### TargetSpec

This is the most important thing to understand. `targetSpec` has three fields:

```typescript
TargetSpec = {
  scope: 'single' | 'all',
  side: 'friendly' | 'enemy' | 'all',
  entityType: 'minion' | 'hero' | 'all'
}
```

#### `scope` — who picks the target?

`scope` does NOT mean "one target" vs "many targets". It means **who resolves the targeting**.

- `'single'` — the **player** must click a target before playing the card. The engine validates their choice against `side` and `entityType`. Use this only when there is a genuine choice between multiple possible targets.

- `'all'` — the **engine** resolves targets automatically from the pool defined by `side` and `entityType`. No player input. Use this for AoE effects AND for effects where there is only one possible target (like "deal damage to the enemy hero" — there is no choice, so the engine should handle it automatically).

#### `side`

- `'friendly'` — the active player's side
- `'enemy'` — the opponent's side
- `'all'` — both sides

#### `entityType`

- `'minion'` — only minions on the battlefield
- `'hero'` — only the hero
- `'all'` — minions and hero

#### TargetSpec Examples

| Description | scope | side | entityType |
|---|---|---|---|
| Deal 4 damage to any target (player chooses) | `single` | `all` | `all` |
| Deal 2 damage to ALL minions | `all` | `all` | `minion` |
| Deal 5 damage to the enemy hero | `all` | `enemy` | `hero` |
| Give a friendly minion +3+3 (player chooses) | `single` | `friendly` | `minion` |
| Give ALL friendly minions +1+1 | `all` | `friendly` | `minion` |
| Return a friendly minion to hand (player chooses) | `single` | `friendly` | `minion` |
| Return ALL minions to hand | `all` | `all` | `minion` |
| Deal 10 damage to your own hero | `all` | `friendly` | `hero` |

---

## The Effect Pipeline

This is the core of the engine. Effects are never applied immediately — they go through a queue.

### Flow

```
playCard / endTurn / checkForDeaths
    │
    ├─ enqueue effects → effectQueue[]
    │
    └─ processEffectQueue()
           │
           └─ applyEffect() for each queued effect
```

### `effectQueue`

Each item in the queue is a `QueuedEffect`:

```typescript
type QueuedEffect = {
  effect: Effect,
  targetID?: string,     // pre-selected player target (for scope: 'single')
  sourceBoard: Board,    // board of the player who owns this effect
  enemyBoard: Board,     // opponent's board
  selfID?: string        // entityID of the minion that created this effect
                         // used to exclude self from returnToHand pool
}
```

`sourceBoard` and `enemyBoard` are captured at enqueue time, not resolved at apply time. This means if boards change between enqueue and apply (which can happen in death chains), the effect still fires from the correct player's perspective.

### `applyEffect`

Resolves targets and applies the effect. For effects with a `targetSpec`:
- If `targetID` is set (player picked a target), use that entity directly.
- Otherwise call `resolveTargets()` to build the pool from `side` and `entityType`.
- Then filter out `selfID` from the pool (so a minion's own onPlay effect doesn't target itself with `returnToHand`).

If the pool is empty after filtering, the effect is silently skipped (no-op).

---

## Death Checking

`checkForDeaths()` is called after every action that can deal damage: `playCard`, `attack`, and recursively after death effects resolve.

### Flow

```
checkForDeaths()
  │
  ├─ scan both battlefields for minions with defence <= 0
  ├─ for each dead minion:
  │    ├─ check its onDeath abilities (with requirements)
  │    ├─ enqueue their effects
  │    └─ move minion to graveyard
  │
  ├─ if any onDeath effects were enqueued → processEffectQueue()
  └─ if any minions died → checkForDeaths()  (recursive, handles death chains)
```

The recursion handles cases where a deathrattle kills another minion, which might trigger another deathrattle, and so on.

`anyDied` and `anyOnDeathTriggers` are tracked separately so `processEffectQueue` is only called if there are actually effects to process, and the recursive call only happens if something actually died.

---

## Sockets & Actions

The server exposes three game actions via socket events. Each validates input and returns the updated `GameStateServer`, or `null` if the action was invalid (client should ignore `null` responses).

### `playCard`

```typescript
playCard(socketID, { index: number, target?: string })
```

- `index` — position in the player's hand
- `target` — optional entityID string (`'heroSelf'`, `'heroEnemy'`, or a minion's `entityID`)

**Validation:**
- Card must exist at that hand index.
- For incantations with `scope: 'single'` effects, a valid target must be provided.
- Target is validated against every effect's `targetSpec` — if any effect disagrees with the chosen target, the whole play is rejected.

**Flow:** peek card → validate → splice from hand → push minion to battlefield (if minion) → enqueue onPlay effects → processEffectQueue → checkForDeaths → increment `cardsPlayedThisTurn`.

### `attack`

```typescript
attack(socketID, { originID: string, targetID: string })
```

- `originID` — entityID of the attacking minion
- `targetID` — entityID of the target, or `'heroEnemy'`

**Validation:** attacker must exist, must be a minion, must not be exhausted.

**Flow:** mark attacker exhausted → deal damage → retaliate if target is a minion → checkForDeaths.

### `endTurn`

No parameters. Increments `turnCount`, switches active player, draws a card, increments `baseMana`, restores `mana` to `baseMana`, unexhausts all minions, resets `cardsPlayedThisTurn`, fires `onDraw` triggers.

---

## Writing New Cards

Add new cards to the `cards` array in `cards.ts`. The comment block at the top of that file has a full reference. Here is the practical process:

### Step 1 — Pick an id

Check the existing cards and pick the next unused number. ids must be unique.

### Step 2 — Decide the card type

Is it a `'minion'` or an `'incantation'`?

### Step 3 — Write the abilities

For each thing the card does, ask:
- WHEN does it fire? → `trigger`
- Does it need a special condition to fire? → `requirements`
- WHAT does it do? → `effects`

For each effect, ask:
- What kind of effect? → `type`
- Does the player need to click something, or does the engine resolve it? → `scope`
- Whose side? → `side`
- Minions, heroes, or both? → `entityType`

### Step 4 — Write the description

Use the keyword formatting conventions:
- `<strong>Fanfare:</strong>` for onPlay effects
- `<strong>Last Breath:</strong>` for onDeath effects
- `<strong>Combo:</strong>` for abilities with `requirements: [{ type: 'combo' }]`
- `<strong>Blitz</strong>` for the `charge` attribute

### Step 5 — Add to a test deck and verify

Add the card's id to a deck and play it in a local game. Check the server console for any errors.

### Full Examples

**Simple Fanfare minion:**
```typescript
{
  id: 20,
  color: 'white',
  name: 'Ironclad Defender',
  description: '<strong>Fanfare:</strong> Give all friendly minions +0 +2',
  baseCost: 3,
  type: 'minion',
  races: ['human'],
  baseAttack: 2,
  baseDefence: 3,
  image_url: '',
  attributes: [],
  abilities: [
    {
      trigger: 'onPlay',
      effects: [
        {
          type: 'buff',
          attack: 0,
          defence: 2,
          targetSpec: { scope: 'all', side: 'friendly', entityType: 'minion' }
          // scope: 'all' because there's no choice — buff ALL of them
        }
      ]
    }
  ]
}
```

**Combo incantation:**
```typescript
{
  id: 21,
  color: 'black',
  name: 'Shadow Strike',
  description: 'Deal 2 damage. <strong>Combo:</strong> Deal 4 damage instead.',
  baseCost: 2,
  type: 'incantation',
  image_url: '',
  abilities: [
    {
      trigger: 'onPlay',
      // no requirements = always fires
      effects: [{ type: 'damage', damage: 2, targetSpec: { scope: 'single', side: 'all', entityType: 'all' } }]
    },
    {
      trigger: 'onPlay',
      requirements: [{ type: 'combo' }],
      // fires only on combo, adds 2 more on top = 4 total
      effects: [{ type: 'damage', damage: 2, targetSpec: { scope: 'single', side: 'all', entityType: 'all' } }]
    }
  ]
}
```

**Deathrattle minion:**
```typescript
{
  id: 22,
  color: 'white',
  name: 'Martyr Knight',
  description: '<strong>Last Breath:</strong> Deal 3 damage to the enemy hero.',
  baseCost: 4,
  type: 'minion',
  races: ['human'],
  baseAttack: 3,
  baseDefence: 4,
  image_url: '',
  attributes: [],
  abilities: [
    {
      trigger: 'onDeath',
      effects: [
        {
          type: 'damage',
          damage: 3,
          targetSpec: { scope: 'all', side: 'enemy', entityType: 'hero' }
          // scope: 'all' — only one enemy hero, no player choice needed
        }
      ]
    }
  ]
}
```

---

## Common Mistakes

**Using `scope: 'single'` when there is no real player choice.**
If there is only one possible target (enemy hero, your own hero, all minions), use `scope: 'all'`. `scope: 'single'` means "pause and wait for the player to click something" — if there's nothing to choose between, that's wrong.

**Using `scope: 'all'` when the player should choose.**
"Give a friendly minion +3+3" — there are multiple friendly minions and the player picks one. Use `scope: 'single'`.

**Validating only some effects on a multi-effect ability.**
The incantation target validator checks ALL effects. If your incantation has two effects with different `side` values (e.g. one `'friendly'`, one `'enemy'`), they will conflict and the play will always be rejected regardless of target. Use separate abilities instead.

**Forgetting that AoE onPlay effects hit the source minion.**
When a minion plays and its onPlay fires, it is already on the battlefield. An effect with `side: 'all', entityType: 'minion'` will include the minion itself. This is intentional — Radiant Sentinel deals 2 damage to all minions including itself.

**`returnToHand` on a minion with no other friendly minions.**
If `scope: 'single'` and `target` is undefined, `resolveTargets` is called as fallback. `selfID` filtering removes the source minion from the pool. If no other friendly minions exist, the pool is empty and the effect is a no-op. The minion stays on the board. This is correct behavior.

**Duplicate ids.**
Every card needs a unique `id`. Check the existing array before picking one.