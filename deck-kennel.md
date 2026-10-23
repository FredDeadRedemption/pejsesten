# Kennel (black)

Hero: **The Hermit**, 30 hp. 25 cards, 10 distinct.

A midrange beast deck. 13 Beasts, two ways to buff them, and poison on a body as the
answer to anything too big to fight. It is the faster of the two decks: Blitz lets it take
the board on arrival, and the Deathwish bodies mean clearing that board costs the opponent
more cards than it costs you.

## List

| # | Card | Cost | Body | Text |
|---|------|------|------|------|
| 3 | Black Cat | 1 | 2/1 Beast | Tradeable |
| 2 | Zoo | 1 | — | Give all your beasts +1 +1 (Tradeable) |
| 3 | Dire Hound | 2 | 2/2 Beast | Blitz |
| 3 | Carrion Hound | 2 | 1/3 Beast | Deathwish: draw a card |
| 2 | Assassin | 2 | 1/1 Human | Stealth, Poisonous |
| 2 | Pack Instinct | 2 | omen | Summon three 1/1 rats |
| 3 | Hound Master | 3 | 2/3 Human | Fanfare: give a friendly beast +2 +2 |
| 3 | Plague Rat | 3 | 1/2 Beast | Blitz, Poisonous |
| 2 | Menagerie Keeper | 4 | 2/4 Human | Your beasts have +1 +1 |
| 2 | Dire Matriarch | 6 | 4/5 Beast | Deathwish: summon two 2/2 hounds |

Curve: 5 at one, 10 at two, 6 at three, 2 at four, 2 at six.

## How it plays

Black Cat into Dire Hound is the opening you want: two bodies by turn 2, and Blitz means
the Hound answers whatever they played instead of waiting a turn. Hound Master on a Beast
is the tempo turn - a 3-drop that puts 4/5 of stats on the table across two bodies.

Plague Rat is the removal. Blitz plus Poisonous kills any single minion the turn it lands,
for 3 mana and a 1/2 body. It is walled off by Guard, which is exactly the counterplay
Dawnguard has. Assassin is the answer to that: Stealth ignores Guard entirely, so the two
copies exist to kill a Gargoyle Vigil or a Silverguard Knight the Rat cannot reach.

Menagerie Keeper and Dire Matriarch are the grind. The Keeper's aura lifts every Beast
while it lives, which turns Pack Instinct's rats into real bodies and makes killing the
Keeper the opponent's problem. Note the board cap of 7 - hold Pack Instinct's rats and the
Matriarch's hounds apart or the extra tokens are lost.

Pack Instinct on *friendly minion dies* nearly always fires the turn they clear your
board. On *enemy attacks hero* it punishes a race. On *enemy plays an incantation* it
catches a removal spell.

## Keeps

Keep Black Cat, Dire Hound, Hound Master. Keep Zoo only with two Beasts. Throw back Dire
Matriarch always, and Assassin unless you know you are against Guards.

## Tuning notes

- **Blood Rite** (2, deal 3 damage with Lifesteal) is in the pool but not the list. It is
  the flex slot and the only reach the colour has: swap it in for the second Pack Instinct
  if games keep ending with the opponent on 3 life.
- Plague Rat at 3 mana is priced as removal, not as a body. At 2 mana it would kill any
  minion on curve from turn 2 and the format would not survive it.
- Assassin is a straight port of a known-fair line: 2 mana, 1/1, Stealth and Poisonous.
  Two copies is tech, three would be a removal deck wearing a beast costume.
- 10 cards at two mana is heavy on purpose. The deck wants a body down every turn from 1
  to 4 and has no card draw outside Carrion Hound.
- The deck lost its 4-mana Guard when Rat Cage was reprinted as a 1-mana 0/3 for Sideshow.
  Menagerie Keeper took the slot, so the deck now protects its aura instead of its hero.

## Cards that need engine work

Designed for this deck, not printable today. Each one names the single missing piece.

- **Kennel Call** — 2 incantation. *Draw a Beast from your deck.*
  Needs **filtered draw**. `Effect::Draw` takes an amount and always takes the top card;
  the `FollowUpAbility` conditions can only react to what was drawn, not search for it.
- **Houndmaster's Whistle** — 3 incantation. *Give a friendly beast Blitz and +2 +2.*
  Needs an effect that **grants a keyword** - the same entity-level attribute list
  Dawnguard's Lightbearer wants.
- **Bone Chorus** — 4 incantation. *Summon a beast from your graveyard.*
  Needs **graveyard summoning**. `Effect::Summon` only mints a fresh card from the pool;
  `Board.graveyard` is written and never read.
- **Powder Keg** — 2, 0/4. *Fuse 3: deal 4 damage to all enemy minions.*
  Needs **Fuse**: a per-entity countdown ticked at the start of your turn plus a trigger
  that fires at zero. New field on `MinionEntity` and a new `Trigger` variant.
- **Rabid Bite** — 1 incantation. *Deal 2 damage. Escalate: +1 damage per copy already
  played this game.*
  Needs the same **per-game copies-played counter** as Triad.
