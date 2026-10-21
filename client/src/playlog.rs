//! What the left column shows: the card the opponent played last, and every card played so far.

use shared::types::{CardEntity, GameEvent, GameStateClient};

/// How long the revealed card takes to settle into its slot.
const POP_TIME: f32 = 0.3;

/// One card that was played, and whose it was.
pub struct Play {
    pub card: CardEntity,
    pub mine: bool,
}

pub struct PlayLog {
    plays: Vec<Play>,
    reveal: Option<CardEntity>,
    reveal_age: f32,
}

impl PlayLog {
    pub fn new() -> Self {
        Self { plays: vec![], reveal: None, reveal_age: POP_TIME }
    }

    /// `state` is the board as the event starts, so the card is still in the hand it is leaving.
    pub fn record(&mut self, event: &GameEvent, state: &GameStateClient) {
        let GameEvent::CardPlayed { entity_id } = event else { return };
        let Some((card, mine)) = find_in_hand(*entity_id, state) else { return };
        if !mine {
            self.reveal = Some(card.clone());
            self.reveal_age = 0.0;
        }
        self.plays.push(Play { card, mine });
    }

    pub fn tick(&mut self, dt: f32) {
        self.reveal_age += dt;
    }

    /// Newest first, the order the column reads in.
    pub fn plays(&self) -> impl Iterator<Item = &Play> {
        self.plays.iter().rev()
    }

    pub fn reveal(&self) -> Option<&CardEntity> {
        self.reveal.as_ref()
    }

    /// 0 the moment a card is revealed, 1 once it has settled.
    pub fn pop(&self) -> f32 {
        (self.reveal_age / POP_TIME).clamp(0.0, 1.0)
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
