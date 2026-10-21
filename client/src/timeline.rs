//! Keeps the board the player sees behind the board the server has confirmed.
//!
//! The snapshot the server pushes is the truth, and every rule and every input reads it. The
//! events riding along only walk a second, lagging copy of the board forward one step at a time,
//! so something leaving play is still on screen while its animation runs. Every batch ends by
//! assigning the snapshot over that copy, so the two cannot drift apart.

use std::collections::VecDeque;

use shared::types::{Board, CardEntity, GameEvent, GameStateClient, MinionEntity, OmenEntity, StateUpdate};

/// How long an event holds the board before the next one starts.
pub fn duration(event: &GameEvent) -> f32 {
    match event {
        GameEvent::OmenFired { .. } => 0.45,
        // both fly a card across the board, so they get the time that takes to read
        GameEvent::CardPlayed { .. } => 0.40,
        GameEvent::CardDrawn { .. } => 0.32,
        GameEvent::Attacked { .. } => 0.38,
        GameEvent::Died { .. } => 0.35,
        GameEvent::Damaged { .. } | GameEvent::Healed { .. } => 0.30,
        GameEvent::MinionSummoned { .. } | GameEvent::ReturnedToHand { .. } => 0.30,
        GameEvent::Buffed { .. } | GameEvent::WardBroken { .. } | GameEvent::TriggerFired { .. } => 0.25,
        GameEvent::CardCreated { .. } | GameEvent::CardTraded { .. } => 0.22,
        GameEvent::TurnEnded => 0.20,
    }
}

/// One push from the server: the events still to play, and the board they resolved against.
struct Batch {
    events: VecDeque<GameEvent>,
    state: GameStateClient,
}

struct Playing {
    event: GameEvent,
    elapsed: f32,
}

pub struct Timeline {
    state: GameStateClient,
    visual: GameStateClient,
    batches: VecDeque<Batch>,
    playing: Option<Playing>,
}

impl Timeline {
    /// A fresh timeline has nothing on screen to animate from, so its first board is simply shown.
    pub fn new(state: GameStateClient) -> Self {
        Self { visual: state.clone(), state, batches: VecDeque::new(), playing: None }
    }

    /// The board every rule and every input reads.
    pub fn state(&self) -> &GameStateClient {
        &self.state
    }

    /// The board to draw, which equals the truth only once the queue has run dry.
    pub fn visual(&self) -> &GameStateClient {
        &self.visual
    }

    pub fn idle(&self) -> bool {
        self.playing.is_none() && self.batches.iter().all(|b| b.events.is_empty())
    }

    pub fn push(&mut self, update: StateUpdate) {
        self.state = update.state.clone();
        // a refused action broadcasts too, so an empty push must not cut a running batch short
        if update.events.is_empty() {
            if self.idle() {
                self.visual = update.state;
            }
            return;
        }
        self.batches.push_back(Batch { events: update.events.into(), state: update.state });
    }

    /// Advances the clock and returns the events that start playing on this frame, in order.
    pub fn tick(&mut self, dt: f32) -> Vec<GameEvent> {
        let mut started = vec![];
        let mut left = dt;
        loop {
            match self.playing.take() {
                Some(Playing { event, elapsed }) => {
                    let remaining = duration(&event) - elapsed;
                    if left < remaining {
                        self.playing = Some(Playing { event, elapsed: elapsed + left });
                        return started;
                    }
                    left -= remaining;
                    self.commit(&event);
                }
                None => match self.take_next() {
                    Some(event) => {
                        started.push(event.clone());
                        self.playing = Some(Playing { event, elapsed: 0.0 });
                    }
                    None => {
                        self.visual = self.state.clone();
                        return started;
                    }
                },
            }
        }
    }

    fn take_next(&mut self) -> Option<GameEvent> {
        while let Some(batch) = self.batches.front_mut() {
            if let Some(event) = batch.events.pop_front() {
                return Some(event);
            }
            self.batches.pop_front();
        }
        None
    }

    /// An event that finished playing lands on the visual board. Anything new is copied out of
    /// the snapshot the batch is heading toward, so nothing here has to invent an entity.
    fn commit(&mut self, event: &GameEvent) {
        let target = match self.batches.front() {
            Some(batch) => &batch.state,
            None => &self.state,
        };
        apply(&mut self.visual, target, event);
    }
}

/// Which half of a view an entity sits on, so a copy lands back on its own side.
#[derive(Clone, Copy)]
enum Half {
    Own,
    Enemy,
}

fn board(state: &mut GameStateClient, half: Half) -> &mut Board {
    match half {
        Half::Own => &mut state.self_board,
        Half::Enemy => &mut state.enemy_board,
    }
}

/// Battlefield first, then graveyard: a minion that died inside the same batch must still be
/// placed on the board before its death can play.
fn find_minion(state: &GameStateClient, id: u32) -> Option<(Half, &MinionEntity)> {
    for (half, board) in [(Half::Own, &state.self_board), (Half::Enemy, &state.enemy_board)] {
        if let Some(m) = board.battlefield.iter().chain(board.graveyard.iter()).find(|m| m.entity_id == id) {
            return Some((half, m));
        }
    }
    None
}

fn find_card(state: &GameStateClient, id: u32) -> Option<(Half, &CardEntity)> {
    for (half, board) in [(Half::Own, &state.self_board), (Half::Enemy, &state.enemy_board)] {
        if let Some(c) = board.hand.iter().find(|c| c.entity_id() == id) {
            return Some((half, c));
        }
    }
    None
}

fn find_omen(state: &GameStateClient, id: u32) -> Option<(Half, &OmenEntity)> {
    for (half, board) in [(Half::Own, &state.self_board), (Half::Enemy, &state.enemy_board)] {
        if let Some(o) = board.omens.iter().find(|o| o.entity_id == id) {
            return Some((half, o));
        }
    }
    None
}

fn take_from_hand(state: &mut GameStateClient, id: u32) -> Option<(Half, CardEntity)> {
    for half in [Half::Own, Half::Enemy] {
        let b = board(state, half);
        if let Some(i) = b.hand.iter().position(|c| c.entity_id() == id) {
            return Some((half, b.hand.remove(i)));
        }
    }
    None
}

fn take_from_battlefield(state: &mut GameStateClient, id: u32) -> Option<(Half, MinionEntity)> {
    for half in [Half::Own, Half::Enemy] {
        let b = board(state, half);
        if let Some(i) = b.battlefield.iter().position(|m| m.entity_id == id) {
            return Some((half, b.battlefield.remove(i)));
        }
    }
    None
}

/// Battlefield only: a minion already in the graveyard takes no further hits.
fn minion_mut(state: &mut GameStateClient, id: u32) -> Option<&mut MinionEntity> {
    if let Some(m) = state.self_board.battlefield.iter_mut().find(|m| m.entity_id == id) {
        return Some(m);
    }
    state.enemy_board.battlefield.iter_mut().find(|m| m.entity_id == id)
}

/// Damage moving on a minion or a hero, with derived defence pulled along so the badge ticks.
fn hit(state: &mut GameStateClient, id: u32, amount: i32) {
    if let Some(m) = minion_mut(state, id) {
        m.damage = (m.damage + amount).max(0);
        m.stats.defence = m.stats.max_defence - m.damage;
        return;
    }
    for half in [Half::Own, Half::Enemy] {
        let hero = &mut board(state, half).hero;
        if hero.entity_id == id {
            hero.damage = (hero.damage + amount).max(0);
            hero.stats.defence = hero.stats.max_defence - hero.damage;
            return;
        }
    }
}

/// Moves entities between zones and nothing else. The engine resolved every rule already, so an
/// event is a finished fact; an imprecision here lasts one batch and is then overwritten.
fn apply(visual: &mut GameStateClient, target: &GameStateClient, event: &GameEvent) {
    match event {
        GameEvent::CardPlayed { entity_id } => {
            take_from_hand(visual, *entity_id);
            // an incantation is in neither zone afterwards, so it simply leaves the hand
            if let Some((half, minion)) = find_minion(target, *entity_id) {
                let minion = minion.clone();
                board(visual, half).battlefield.push(minion);
            } else if let Some((half, omen)) = find_omen(target, *entity_id) {
                let omen = omen.clone();
                board(visual, half).omens.push(omen);
            }
        }
        GameEvent::MinionSummoned { entity_id } => {
            if let Some((half, minion)) = find_minion(target, *entity_id) {
                let minion = minion.clone();
                board(visual, half).battlefield.push(minion);
            }
        }
        GameEvent::Attacked { attacker_id, .. } => {
            if let Some(m) = minion_mut(visual, *attacker_id) {
                m.exhausted = true;
                m.stealth_active = false;
            }
        }
        GameEvent::Damaged { entity_id, amount } => hit(visual, *entity_id, *amount),
        GameEvent::Healed { entity_id, amount } => hit(visual, *entity_id, -amount),
        GameEvent::Buffed { entity_id, attack, defence } => {
            if let Some(m) = minion_mut(visual, *entity_id) {
                m.buff.attack += attack;
                m.buff.defence += defence;
                // auras are rebuilt server side, so only the direct buff shows before the snap
                m.stats.attack += attack;
                m.stats.max_defence += defence;
                m.stats.defence = m.stats.max_defence - m.damage;
            }
        }
        GameEvent::WardBroken { entity_id } => {
            if let Some(m) = minion_mut(visual, *entity_id) {
                m.ward_active = false;
            }
        }
        GameEvent::Died { entity_id } => {
            if let Some((half, minion)) = take_from_battlefield(visual, *entity_id) {
                board(visual, half).graveyard.push(minion);
            }
        }
        GameEvent::ReturnedToHand { entity_id } => {
            take_from_battlefield(visual, *entity_id);
            if let Some((half, card)) = find_card(target, *entity_id) {
                let card = card.clone();
                board(visual, half).hand.push(card);
            }
        }
        GameEvent::CardDrawn { entity_id } => {
            if let Some((half, card)) = find_card(target, *entity_id) {
                let card = card.clone();
                let board = board(visual, half);
                // only the count is drawn, so any card off the pile shrinks it correctly
                board.deck.pop();
                board.hand.push(card);
            }
        }
        GameEvent::CardCreated { entity_id } => {
            if let Some((half, card)) = find_card(target, *entity_id) {
                let card = card.clone();
                board(visual, half).hand.push(card);
            }
        }
        GameEvent::CardTraded { entity_id } => {
            if let Some((half, card)) = take_from_hand(visual, *entity_id) {
                board(visual, half).deck.push(card);
            }
        }
        GameEvent::OmenFired { entity_id } => {
            for half in [Half::Own, Half::Enemy] {
                board(visual, half).omens.retain(|o| o.entity_id != *entity_id);
            }
        }
        GameEvent::TriggerFired { .. } | GameEvent::TurnEnded => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::types::{Buff, Card, Counters, GamePhase, Hero, Stats};

    fn board(hero_id: u32) -> Board {
        let card = shared::cards::hero_or_default(shared::cards::DEFAULT_HERO_ID).clone();
        let hp = card.base_hp;
        Board {
            deck: vec![],
            hand: vec![],
            graveyard: vec![],
            battlefield: vec![],
            omens: vec![],
            hero: Hero { entity_id: hero_id, card, base_hp: hp, damage: 0, buff: Buff::default(), stats: Stats::new(0, hp) },
            base_mana: 1,
            mana: 1,
            embers: 0,
            counters: Counters::default(),
            quickdraw_log: vec![],
        }
    }

    fn view() -> GameStateClient {
        GameStateClient {
            self_board: board(0),
            enemy_board: board(1),
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

    fn damage_of(state: &GameStateClient, id: u32) -> i32 {
        state.self_board.battlefield.iter().find(|m| m.entity_id == id).expect("minion on the board").damage
    }

    /// Each event lands as it finishes, and the snapshot has the last word.
    #[test]
    fn a_batch_steps_the_board_forward_and_ends_on_the_truth() {
        let mut before = view();
        before.self_board.battlefield.push(minion(1));
        let mut after = before.clone();
        after.self_board.battlefield[0].damage = 2;

        let mut timeline = Timeline::new(before);
        timeline.push(StateUpdate {
            state: after,
            events: vec![GameEvent::Damaged { entity_id: 1, amount: 1 }, GameEvent::Damaged { entity_id: 1, amount: 1 }],
        });

        timeline.tick(0.0);
        assert_eq!(damage_of(timeline.visual(), 1), 0, "nothing lands before its event is over");
        timeline.tick(duration(&GameEvent::Damaged { entity_id: 1, amount: 1 }));
        assert_eq!(damage_of(timeline.visual(), 1), 1, "the first hit landed on its own");
        assert!(!timeline.idle());

        timeline.tick(10.0);
        assert_eq!(damage_of(timeline.visual(), 1), 2);
        assert!(timeline.idle());
    }

    /// The whole point of the lagging copy: something that left play is still there to animate.
    #[test]
    fn a_dying_minion_stays_on_the_board_until_its_event_is_over() {
        let mut before = view();
        before.self_board.battlefield.push(minion(1));
        let mut after = view();
        after.self_board.graveyard.push(minion(1));

        let mut timeline = Timeline::new(before);
        timeline.push(StateUpdate { state: after, events: vec![GameEvent::Died { entity_id: 1 }] });

        timeline.tick(0.0);
        assert_eq!(timeline.visual().self_board.battlefield.len(), 1, "still on screen while it dies");
        assert_eq!(timeline.state().self_board.battlefield.len(), 0, "already gone as far as the rules go");

        timeline.tick(10.0);
        assert_eq!(timeline.visual().self_board.battlefield.len(), 0);
        assert_eq!(timeline.visual().self_board.graveyard.len(), 1);
    }

    /// A refused action broadcasts an empty list, which must not swallow what is still playing.
    #[test]
    fn an_empty_push_leaves_a_running_batch_alone_and_still_wins_in_the_end() {
        let mut before = view();
        before.self_board.battlefield.push(minion(1));
        let mut after = before.clone();
        after.self_board.battlefield[0].damage = 1;

        let mut timeline = Timeline::new(before);
        timeline.push(StateUpdate {
            state: after,
            events: vec![GameEvent::Damaged { entity_id: 1, amount: 1 }],
        });
        timeline.tick(0.05);
        assert!(!timeline.idle());

        let mut refused = view();
        refused.self_board.battlefield.push(minion(1));
        refused.self_board.battlefield[0].damage = 1;
        refused.turn_count = 9;
        timeline.push(StateUpdate { state: refused, events: vec![] });
        assert!(!timeline.idle(), "the batch is still running");

        timeline.tick(10.0);
        assert!(timeline.idle());
        assert_eq!(timeline.visual().turn_count, 9, "the newest board is what it settles on");
    }
}
