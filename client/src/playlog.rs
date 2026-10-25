//! What the left column shows: a small icon per card played, and the opponent's play blown up
//! for as long as it is on its way to the board.

use shared::types::{Card, CardEntity, GameEvent, GameStateClient, MinionEntity};

/// Grow, hold, pull back. Long enough to read the card, short enough to stay out of the way.
const POP_TIME: f32 = 0.22;
const HOLD_TIME: f32 = 1.3;
const EXIT_TIME: f32 = 0.3;

/// What a play landed on: a body it touched, or a card it put in your hand.
pub struct Target {
    pub card: CardEntity,
    pub died: bool,
}

/// One card that was played, whose it was, and what it did.
pub struct Play {
    pub card: CardEntity,
    pub mine: bool,
    pub targets: Vec<Target>,
}

pub struct PlayLog {
    plays: Vec<Play>,
    /// Which play the events still arriving belong to, if any.
    resolving: Option<usize>,
    /// Whose ability is resolving right now, so a watcher's work is not credited to the play.
    source: Option<u32>,
    reveal: Option<CardEntity>,
    /// The play the reveal came from, so its targets can be shown beside it.
    reveal_play: Option<usize>,
    reveal_mine: bool,
    reveal_age: f32,
}

impl PlayLog {
    pub fn new() -> Self {
        Self {
            plays: vec![],
            resolving: None,
            source: None,
            reveal: None,
            reveal_play: None,
            reveal_mine: false,
            reveal_age: 0.0,
        }
    }

    /// `state` is the board as the event starts, so the card is still in the hand it is leaving.
    pub fn record(&mut self, event: &GameEvent, state: &GameStateClient) {
        let (card, mine, replayed) = match event {
            GameEvent::CardPlayed { entity_id } => {
                let Some((card, mine)) = find_in_hand(*entity_id, state) else { return };
                (card, mine, false)
            }
            GameEvent::CardReplayed { entity_id, card_id, white } => {
                let Some(card) = from_catalogue(*card_id, *entity_id) else { return };
                (card, *white == state.you_are_white, true)
            }
            // a new action opens its own burst, so nothing after it belongs to the last play
            GameEvent::Attacked { .. } | GameEvent::TurnEnded => {
                self.resolving = None;
                self.source = None;
                return;
            }
            GameEvent::TriggerFired { source_id } => {
                self.source = Some(*source_id);
                return;
            }
            other => {
                self.record_target(other, state);
                return;
            }
        };
        // your own play is on screen leaving your hand; a replayed card is on screen nowhere
        if !mine || replayed {
            self.reveal = Some(card.clone());
            self.reveal_play = Some(self.plays.len());
            self.reveal_mine = mine;
            self.reveal_age = 0.0;
        }
        self.resolving = Some(self.plays.len());
        self.source = Some(card.entity_id());
        self.plays.push(Play { card, mine, targets: vec![] });
    }

    /// Everything a play touches while it resolves, hung off the play that caused it.
    fn record_target(&mut self, event: &GameEvent, state: &GameStateClient) {
        let Some(index) = self.resolving else { return };
        let play = &mut self.plays[index];

        // a death is checked once the whole burst has drained, so it names no source of its own
        let own_work = self.source == Some(play.card.entity_id());

        let (entity_id, died) = match event {
            GameEvent::Damaged { entity_id, .. }
            | GameEvent::Healed { entity_id, .. }
            | GameEvent::Buffed { entity_id, .. }
            | GameEvent::MinionSummoned { entity_id }
            | GameEvent::ReturnedToHand { entity_id }
            | GameEvent::ControlTaken { entity_id } => (*entity_id, false),
            GameEvent::Died { entity_id } => (*entity_id, true),
            // never the opponent's, so the strip cannot be read as a peek at their hand
            GameEvent::CardDrawn { entity_id } | GameEvent::CardCreated { entity_id } if play.mine => {
                (*entity_id, false)
            }
            _ => return,
        };

        // the played body is not its own target, and a second hit is not a second target
        if entity_id == play.card.entity_id() {
            return;
        }
        if let Some(seen) = play.targets.iter_mut().find(|t| t.card.entity_id() == entity_id) {
            seen.died |= died;
            return;
        }
        if !own_work {
            return;
        }
        let Some(card) = find_entity(entity_id, state, play.mine) else { return };
        play.targets.push(Target { card, died });
    }

    pub fn tick(&mut self, dt: f32) {
        self.reveal_age += dt;
        if self.reveal_age > POP_TIME + HOLD_TIME + EXIT_TIME {
            self.reveal = None;
            self.reveal_play = None;
        }
    }

    /// Newest first, the order the column reads in.
    pub fn plays(&self) -> impl Iterator<Item = &Play> {
        self.plays.iter().rev()
    }

    pub fn reveal(&self) -> Option<&CardEntity> {
        self.reveal.as_ref()
    }

    pub fn reveal_targets(&self) -> Option<&[Target]> {
        let play = self.plays.get(self.reveal_play?)?;
        (!play.targets.is_empty()).then(|| play.targets.as_slice())
    }

    pub fn reveal_mine(&self) -> bool {
        self.reveal_mine
    }

    /// 0 the moment a card is revealed, 1 once it has grown to full size.
    pub fn pop(&self) -> f32 {
        (self.reveal_age / POP_TIME).clamp(0.0, 1.0)
    }

    /// 0 while the card is still being read, 1 once it has pulled all the way back.
    pub fn exit(&self) -> f32 {
        ((self.reveal_age - POP_TIME - HOLD_TIME) / EXIT_TIME).clamp(0.0, 1.0)
    }
}

/// A replayed card is in no zone, so the card the id names is built from the catalogue.
fn from_catalogue(card_id: u32, entity_id: u32) -> Option<CardEntity> {
    match shared::cards::get_card_by_id(card_id)? {
        Card::Minion(c) => Some(CardEntity::Minion(shared::cards::instantiate_minion(c, entity_id))),
        Card::Incantation(c) => Some(CardEntity::Incantation(shared::cards::instantiate_incantation(c, entity_id))),
        Card::Omen(_) | Card::Hero(_) => None,
    }
}

/// Bodies are public on both boards; hands and decks are only ever read on your own side.
fn find_entity(entity_id: u32, state: &GameStateClient, mine: bool) -> Option<CardEntity> {
    let on_board = [&state.self_board, &state.enemy_board].into_iter().find_map(|b| {
        let found = |m: &MinionEntity| m.entity_id == entity_id;
        b.battlefield.iter().find(|m| found(m)).or_else(|| b.graveyard.iter().find(|m| found(m)))
    });
    if let Some(minion) = on_board {
        return Some(CardEntity::Minion(minion.clone()));
    }
    if !mine {
        return None;
    }
    let board = &state.self_board;
    board.hand.iter().chain(board.deck.iter()).find(|c| c.entity_id() == entity_id).cloned()
}

fn find_in_hand(entity_id: u32, state: &GameStateClient) -> Option<(CardEntity, bool)> {
    for (mine, board) in [(true, &state.self_board), (false, &state.enemy_board)] {
        if let Some(card) = board.hand.iter().find(|c| c.entity_id() == entity_id) {
            return Some((card.clone(), mine));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::types::{Buff, Counters, GamePhase, Hero, MinionEntity, Stats};

    fn board() -> shared::types::Board {
        let card = shared::cards::hero_or_default(shared::cards::DEFAULT_HERO_ID).clone();
        let hp = card.base_hp;
        shared::types::Board {
            deck: vec![],
            hand: vec![],
            graveyard: vec![],
            battlefield: vec![],
            omens: vec![],
            hero: Hero { entity_id: 99, card, base_hp: hp, damage: 0, buff: Buff::default(), stats: Stats::new(0, hp) },
            base_mana: 1,
            mana: 1,
            embers: 0,
            counters: Counters::default(),
            fatigue: 0,
            quickdraw_log: vec![],
        }
    }

    fn view() -> GameStateClient {
        GameStateClient {
            self_board: board(),
            enemy_board: board(),
            white_player_id: "white".to_string(),
            black_player_id: "black".to_string(),
            your_turn: true,
            turn_count: 1,
            cards_played_this_turn: 0,
            arrivals_played_this_turn: 0,
            phase: GamePhase::Playing,
            you_are_white: true,
            mulligan_submitted: true,
            open_cards: false,
            hand_hints: vec![],
        }
    }

    fn minion(entity_id: u32) -> MinionEntity {
        let card = shared::cards::get_collectible_cards()
            .into_iter()
            .find_map(|c| match c {
                Card::Minion(m) => Some(m),
                _ => None,
            })
            .expect("the catalogue has minions");
        shared::cards::instantiate_minion(&card, entity_id)
    }

    fn spell(entity_id: u32) -> CardEntity {
        let card = shared::cards::get_collectible_cards()
            .into_iter()
            .find_map(|c| match c {
                Card::Incantation(i) => Some(i),
                _ => None,
            })
            .expect("the catalogue has incantations");
        CardEntity::Incantation(shared::cards::instantiate_incantation(&card, entity_id))
    }

    /// A watcher woken by the play does its own work, and the play must not be credited with it.
    #[test]
    fn a_woken_watcher_keeps_its_own_targets() {
        let mut state = view();
        state.self_board.hand.push(spell(1));
        state.enemy_board.battlefield.extend([minion(2), minion(3), minion(4)]);
        state.self_board.battlefield.push(minion(5));

        let mut log = PlayLog::new();
        for event in [
            GameEvent::CardPlayed { entity_id: 1 },
            GameEvent::TriggerFired { source_id: 1 },
            GameEvent::Damaged { entity_id: 2, amount: 3 },
            // the watcher on the board, not the card that woke it
            GameEvent::TriggerFired { source_id: 5 },
            GameEvent::Damaged { entity_id: 3, amount: 1 },
            GameEvent::Damaged { entity_id: 4, amount: 1 },
        ] {
            log.record(&event, &state);
        }

        let targets = &log.plays().next().expect("the play was logged").targets;
        assert_eq!(targets.len(), 1, "only what the card itself hit");
        assert_eq!(targets[0].card.entity_id(), 2);
    }

    /// Deaths are checked once the burst has drained, so they name no source to credit them to.
    #[test]
    fn a_death_marks_a_target_the_play_already_touched() {
        let mut state = view();
        state.self_board.hand.push(spell(1));
        state.enemy_board.battlefield.extend([minion(2), minion(3)]);
        state.self_board.battlefield.push(minion(5));

        let mut log = PlayLog::new();
        for event in [
            GameEvent::CardPlayed { entity_id: 1 },
            GameEvent::TriggerFired { source_id: 1 },
            GameEvent::Damaged { entity_id: 2, amount: 3 },
            GameEvent::TriggerFired { source_id: 5 },
            GameEvent::Damaged { entity_id: 3, amount: 1 },
            GameEvent::Died { entity_id: 2 },
            GameEvent::Died { entity_id: 3 },
        ] {
            log.record(&event, &state);
        }

        let targets = &log.plays().next().expect("the play was logged").targets;
        assert_eq!(targets.len(), 1, "the watcher's kill is not the play's target");
        assert!(targets[0].died, "the play's own target is marked dead");
    }
}
