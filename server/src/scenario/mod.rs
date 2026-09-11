//! Deterministic scenarios that exercise one mechanic each.
//!
//! A case builds its own cards (see [`build`]), sets an exact board up, runs a few steps and
//! records expectations. The same case definition drives both `cargo test` and the visual
//! runner, so nothing can drift between what is asserted and what is shown.

#![allow(dead_code)]

pub mod build;
pub mod cases;
pub mod runner;
#[cfg(test)]
mod tests;

use crate::engine::{Game, IdGenerator, instantiate_incantation, instantiate_minion};
use crate::settings;
use serde::Serialize;
use shared::types::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum FrameKind {
    Setup,
    Action,
    Check,
}

/// A snapshot after one step, with the label the runner shows.
#[derive(Clone, Serialize)]
pub struct Frame {
    pub label: String,
    pub kind: FrameKind,
    pub ok: bool,
    pub state: GameStateServer,
}

/// Build phase. White is always the acting player; `start` freezes the board and begins the run.
pub struct Setup {
    ids: IdGenerator,
    state: GameStateServer,
    summonable: Vec<MinionCard>,
}

impl Setup {
    pub fn new() -> Self {
        let mut ids = IdGenerator::new();
        let white = Self::empty_board(&mut ids);
        let black = Self::empty_board(&mut ids);
        Setup {
            ids,
            state: GameStateServer {
                white,
                black,
                white_player_id: "white".to_string(),
                black_player_id: "black".to_string(),
                white_turn: true,
                turn_count: 1,
                cards_played_this_turn: 0,
                phase: GamePhase::Playing,
                mulligan_white_done: true,
                mulligan_black_done: true,
            },
            summonable: vec![],
        }
    }

    fn empty_board(ids: &mut IdGenerator) -> Board {
        Board {
            deck: vec![],
            hand: vec![],
            graveyard: vec![],
            battlefield: vec![],
            hero: Hero { entity_id: ids.next_id(), attack: 0, defence: settings::STARTING_HP },
            // cases that care about mana set it explicitly; the rest should not have to
            base_mana: settings::MAX_MANA,
            mana: settings::MAX_MANA,
            embers: settings::MAX_EMBERS,
        }
    }

    fn board_mut(&mut self, side: Side) -> &mut Board {
        match side {
            Side::White => &mut self.state.white,
            Side::Black => &mut self.state.black,
        }
    }

    /// Puts a minion straight onto the battlefield, ready to attack. Returns its entity id.
    pub fn on_board(&mut self, side: Side, card: MinionCard) -> u32 {
        let id = self.ids.next_id();
        let mut m = instantiate_minion(&card, id);
        m.turns_on_board = 1;
        self.board_mut(side).battlefield.push(m);
        id
    }

    /// Same as [`Self::on_board`] but summoning sick, as if just played.
    pub fn on_board_fresh(&mut self, side: Side, card: MinionCard) -> u32 {
        let id = self.ids.next_id();
        let mut m = instantiate_minion(&card, id);
        m.exhausted = true;
        self.board_mut(side).battlefield.push(m);
        id
    }

    /// Adds a minion to hand. Returns its hand index.
    pub fn in_hand(&mut self, side: Side, card: MinionCard) -> usize {
        let id = self.ids.next_id();
        let board = self.board_mut(side);
        board.hand.push(CardEntity::Minion(instantiate_minion(&card, id)));
        board.hand.len() - 1
    }

    /// Adds an incantation to hand. Returns its hand index.
    pub fn spell_in_hand(&mut self, side: Side, card: IncantationCard) -> usize {
        let id = self.ids.next_id();
        let board = self.board_mut(side);
        board.hand.push(CardEntity::Incantation(instantiate_incantation(&card, id)));
        board.hand.len() - 1
    }

    /// Appends to the top of the deck; the first card added is the first drawn.
    pub fn in_deck(&mut self, side: Side, card: MinionCard) -> u32 {
        let id = self.ids.next_id();
        self.board_mut(side).deck.push(CardEntity::Minion(instantiate_minion(&card, id)));
        id
    }

    pub fn spell_in_deck(&mut self, side: Side, card: IncantationCard) -> u32 {
        let id = self.ids.next_id();
        self.board_mut(side).deck.push(CardEntity::Incantation(instantiate_incantation(&card, id)));
        id
    }

    pub fn hp(&mut self, side: Side, hp: i32) -> &mut Self {
        self.board_mut(side).hero.defence = hp;
        self
    }

    pub fn mana(&mut self, side: Side, mana: i32) -> &mut Self {
        let board = self.board_mut(side);
        board.base_mana = mana;
        board.mana = mana;
        self
    }

    pub fn embers(&mut self, side: Side, embers: i32) -> &mut Self {
        self.board_mut(side).embers = embers;
        self
    }

    pub fn hero_id(&self, side: Side) -> u32 {
        match side {
            Side::White => self.state.white.hero.entity_id,
            Side::Black => self.state.black.hero.entity_id,
        }
    }

    pub fn hand_entity_id(&self, side: Side, index: usize) -> u32 {
        let board = match side {
            Side::White => &self.state.white,
            Side::Black => &self.state.black,
        };
        board.hand[index].entity_id()
    }

    pub fn cards_played_this_turn(&mut self, n: u32) -> &mut Self {
        self.state.cards_played_this_turn = n;
        self
    }

    /// Marks the hand card at `index` as just drawn, which is what Quickdraw keys off.
    pub fn mark_just_drawn(&mut self, side: Side, index: usize) -> &mut Self {
        if let Some(c) = self.board_mut(side).hand.get_mut(index) {
            *c.just_drawn_mut() = true;
        }
        self
    }

    /// Registers a card that `Summon` can mint without it existing in `cards.rs`.
    pub fn summonable(&mut self, card: MinionCard) -> &mut Self {
        self.summonable.push(card);
        self
    }

    pub fn start(self, name: &str) -> Scenario {
        let opening = self.state.clone();
        let mut game = Game::from_state(self.state, self.ids);
        for card in self.summonable {
            game.register_summonable(card);
        }
        Scenario {
            name: name.to_string(),
            frames: vec![Frame { label: "setup".to_string(), kind: FrameKind::Setup, ok: true, state: opening }],
            game,
            failures: vec![],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    White,
    Black,
}

/// Run phase. Every step and every expectation is recorded as a frame.
pub struct Scenario {
    pub name: String,
    game: Game,
    frames: Vec<Frame>,
    failures: Vec<String>,
}

impl Scenario {
    // --- steps ---

    pub fn play(&mut self, index: usize, target: Option<u32>) -> bool {
        let ok = self.game.play_card(index, target);
        self.record(FrameKind::Action, format!("play hand[{index}] -> {}", ok), true);
        ok
    }

    pub fn attack(&mut self, origin: u32, target: u32) -> bool {
        let ok = self.game.attack(AttackData { origin_id: origin, target_id: target });
        self.record(FrameKind::Action, format!("attack {origin} -> {target} = {ok}"), true);
        ok
    }

    pub fn trade(&mut self, index: usize) -> bool {
        let ok = self.game.trade_card(index);
        self.record(FrameKind::Action, format!("trade hand[{index}] -> {ok}"), true);
        ok
    }

    pub fn end_turn(&mut self) {
        self.game.end_turn();
        self.record(FrameKind::Action, "end turn".to_string(), true);
    }

    /// Runs a step and asserts in one go; keeps cases from tripping over borrow rules.
    pub fn expect_play(&mut self, what: &str, index: usize, target: Option<u32>, want: bool) {
        let got = self.play(index, target);
        self.expect_eq(what, got, want);
    }

    pub fn expect_attack(&mut self, what: &str, origin: u32, target: u32, want: bool) {
        let got = self.attack(origin, target);
        self.expect_eq(what, got, want);
    }

    pub fn expect_trade(&mut self, what: &str, index: usize, want: bool) {
        let got = self.trade(index);
        self.expect_eq(what, got, want);
    }

    // --- expectations ---

    pub fn expect(&mut self, what: &str, ok: bool) {
        if !ok {
            self.failures.push(format!("{}: {}", self.name, what));
        }
        self.record(FrameKind::Check, what.to_string(), ok);
    }

    pub fn expect_eq<T: PartialEq + Debug>(&mut self, what: &str, got: T, want: T) {
        let ok = got == want;
        let label = if ok { what.to_string() } else { format!("{what} (got {got:?}, want {want:?})") };
        if !ok {
            self.failures.push(format!("{}: {}", self.name, label));
        }
        self.record(FrameKind::Check, label, ok);
    }

    fn record(&mut self, kind: FrameKind, label: String, ok: bool) {
        self.frames.push(Frame { label, kind, ok, state: self.game.state.clone() });
    }

    // --- queries ---

    pub fn state(&self) -> &GameStateServer {
        &self.game.state
    }

    pub fn board(&self, side: Side) -> &Board {
        match side {
            Side::White => &self.game.state.white,
            Side::Black => &self.game.state.black,
        }
    }

    fn find(&self, id: u32) -> Option<&MinionEntity> {
        self.game
            .state
            .white
            .battlefield
            .iter()
            .chain(self.game.state.black.battlefield.iter())
            .find(|m| m.entity_id == id)
    }

    pub fn alive(&self, id: u32) -> bool {
        self.find(id).is_some()
    }

    /// Defence of a minion on the battlefield, or `None` once it has left.
    pub fn defence(&self, id: u32) -> Option<i32> {
        self.find(id).map(|m| m.defence)
    }

    pub fn attack_of(&self, id: u32) -> Option<i32> {
        self.find(id).map(|m| m.attack)
    }

    pub fn has_ward(&self, id: u32) -> bool {
        self.find(id).map_or(false, |m| m.ward_active)
    }

    pub fn has_stealth(&self, id: u32) -> bool {
        self.find(id).map_or(false, |m| m.stealth_active)
    }

    pub fn exhausted(&self, id: u32) -> bool {
        self.find(id).map_or(false, |m| m.exhausted)
    }

    pub fn hero_id(&self, side: Side) -> u32 {
        self.board(side).hero.entity_id
    }

    pub fn hp(&self, side: Side) -> i32 {
        self.board(side).hero.defence
    }

    pub fn mana(&self, side: Side) -> i32 {
        self.board(side).mana
    }

    pub fn embers(&self, side: Side) -> i32 {
        self.board(side).embers
    }

    pub fn board_len(&self, side: Side) -> usize {
        self.board(side).battlefield.len()
    }

    pub fn hand_len(&self, side: Side) -> usize {
        self.board(side).hand.len()
    }

    pub fn deck_len(&self, side: Side) -> usize {
        self.board(side).deck.len()
    }

    pub fn graveyard_len(&self, side: Side) -> usize {
        self.board(side).graveyard.len()
    }

    pub fn in_hand(&self, side: Side, id: u32) -> bool {
        self.board(side).hand.iter().any(|c| c.entity_id() == id)
    }

    pub fn hand_cost(&self, side: Side, index: usize) -> Option<i32> {
        self.board(side).hand.get(index).map(|c| c.cost())
    }

    // --- results ---

    pub fn frames(&self) -> &[Frame] {
        &self.frames
    }

    pub fn failures(&self) -> &[String] {
        &self.failures
    }

    pub fn passed(&self) -> bool {
        self.failures.is_empty()
    }
}
