# Philosophy

Why the game is shaped the way it is. The rules live in
[MECHANICS.md](MECHANICS.md); the unbuilt designs in [CONCEPTS.md](CONCEPTS.md). This file
is the reasoning that decides which of those three slots a new card goes in.

---

## The problem

Nobody acts on the opponent's turn. That is deliberate and it is not changing: no response
windows, no instant speed, no priority passing. But it is also the root of a well known
failure chain, and the chain is worth spelling out because every rule below exists to break
one of its links.

Where nothing can be done on the other player's turn, every interaction happens on the turn
of the person doing it. Interaction then only ever means *I act and you watch*. Removal
becomes the single lever that exists, so removal has to be strong. Once removal is strong
the board stops being a place you can commit to, and the only win conditions that survive
are the ones removal cannot touch. Those win conditions are invisible until they resolve,
which is exactly what feels unfair to play against.

## The rule

**The defender makes a choice.**

Not a response - a choice. Made in advance, or forced on them in the moment. The acting
player still holds the turn, but what happens during it is not decided by them alone. Every
reactive mechanic in the game is a different shape of that same idea.

## Which slot a card goes in

Sort by **who owns the decision**, never by what the card does.

| Slot | Visibility | Lifetime | Who decides | What belongs here |
|---|---|---|---|---|
| **Omen** | trigger hidden | one shot | the opponent guesses | value |
| **Wager** | fully public | one shot | the opponent chooses | denial |
| **Aura on a minion** | public | standing | kill the body | taxes |
| **Aura on a rite** | public | three turns | wait it out, or race it | taxes |

The first three are forms of one card type, the incantation; the fourth is a minion. One
card type for everything that is not a body means a card watching incantations watches all
of them, and a counter written as a wager would catch an aura too.

**Omens** hide which of three triggers was armed. The opponent sees the card and plays
around it or walks into it. Because they cannot be certain, an omen must leave them their
play: you gain something, they still get to do the thing they wanted. *Copy the next
incantation they play* is an omen. *Counter it* is not.

**Wagers** hide nothing - trigger, effect and price are all face up. That is what lets them
deny. The opponent reads the card and decides whether the line is worth the cost, and the
card pays either way: they drop the play, or they pay for it. Anything that stops the
opponent doing something goes here, because a denial they cannot see is the *I act and you
watch* failure in miniature.

**Auras** fire every single time and offer no decision at all, so the defender's choice has
to come from somewhere else: either the aura sits on a body and the answer is killing it, or
it sits in the rites row with a public clock and the answer is outlasting it. What is ruled
out is the third case, a standing effect with neither a body nor a clock, which is an
unanswerable permanent and the same shape as an uninteractable win condition.

The clock also decides what the two kinds of aura are for. The one on a body never lapses
but dies to removal; the one on a rite cannot be touched but runs out. Printing both in a
colour gives the deckbuilder a real question and the player a real sequencing problem.

A **negative aura takes attack, never defence.** Board-wide minus defence kills every small
minion the moment it lands, with no body to answer and no time to play around it, which is
unconditional removal wearing an aura costume.

## What follows

**No Secrets.** A Secret is an omen with the card hidden as well, so the opponent cannot
play around it even in principle. That is strictly worse than a mechanic the game already
has. Ruled out in [MECHANICS.md](MECHANICS.md).

**Counter is public.** A hidden counterspell spends the opponent's card and gives them
nothing to decide. As a wager it becomes a real question: hold the incantation and lose a
turn of your plan, or cast it and lose the card. The lost turn is the payment.

**Removal stays conditional.** Quickdraw, Combo, a Guard wall in front of the Poisonous
body - answering a threat should cost setup or tempo. Efficient unconditional removal is
the first link in the chain above; the moment a plain two mana deal 3 is printed, the board
stops being safe to commit to and the game needs unanswerable win conditions to end.

**Win conditions are visible and raceable.** Fuse and Sigils put the clock on the board with
a public counter on it. The opponent gets two outs instead of one - kill it, or go faster -
so removal does not have to be strong to keep it honest.

**Embers never touch the board.** The action menu reaches decks, hands and graveyards only,
never minions and never health. That is a whole axis of interaction that is not removal:
Pry, Snuff and Exile fight the plan instead of the body. Put a ping on that menu and every
cheap card that pings becomes unprintable, and the chain starts over. See
[CONCEPTS.md](CONCEPTS.md).
