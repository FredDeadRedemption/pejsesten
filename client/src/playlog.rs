//! What the left column shows: a small icon per card played, and the opponent's play blown up
//! for as long as it is on its way to the board.

use shared::types::{Card, CardEntity, GameEvent, GameStateClient};

/// Grow, hold, pull back. Long enough to read the card, short enough to stay out of the way.
const POP_TIME: f32 = 0.22;
const HOLD_TIME: f32 = 1.3;
const EXIT_TIME: f32 = 0.3;

/// One card that was played, and whose it was.
pub struct Play {
    pub card: CardEntity,
    pub mine: bool,
}

pub struct PlayLog {
    plays: Vec<Play>,
    reveal: Option<CardEntity>,
    reveal_mine: bool,
    reveal_age: f32,
}

impl PlayLog {
    pub fn new() -> Self {
        Self { plays: vec![], reveal: None, reveal_mine: false, reveal_age: 0.0 }
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
            _ => return,
        };
        // your own play is on screen leaving your hand; a replayed card is on screen nowhere
        if !mine || replayed {
            self.reveal = Some(card.clone());
            self.reveal_mine = mine;
            self.reveal_age = 0.0;
        }
        self.plays.push(Play { card, mine });
    }

    pub fn tick(&mut self, dt: f32) {
        self.reveal_age += dt;
        if self.reveal_age > POP_TIME + HOLD_TIME + EXIT_TIME {
            self.reveal = None;
        }
    }

    /// Newest first, the order the column reads in.
    pub fn plays(&self) -> impl Iterator<Item = &Play> {
        self.plays.iter().rev()
    }

    pub fn reveal(&self) -> Option<&CardEntity> {
        self.reveal.as_ref()
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

fn find_in_hand(entity_id: u32, state: &GameStateClient) -> Option<(CardEntity, bool)> {
    for (mine, board) in [(true, &state.self_board), (false, &state.enemy_board)] {
        if let Some(card) = board.hand.iter().find(|c| c.entity_id() == entity_id) {
            return Some((card.clone(), mine));
        }
    }
    None
}
