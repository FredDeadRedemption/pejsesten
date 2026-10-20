//! The animations the timeline drives. Each one runs on its own clock and expires; [`Overlay`] is the
//! per-entity lookup the renderer reads while drawing a board.

use macroquad::prelude::*;
use std::collections::HashMap;

use crate::layout;
use crate::render::{COL_BUFFED, COL_DAMAGED};
use crate::timeline;
use shared::types::{GameEvent, GameStateClient};

const BUMP_TIME: f32 = 0.38;
const FLIP_RATE: f32 = 8.0;
const NUMBER_LIFE: f32 = 0.6;
const NUMBER_RISE: f32 = 42.0;

/// An entity-anchored effect that runs for a fixed time and then expires.
struct Timed {
    entity_id: u32,
    t: f32,
    life: f32,
}

impl Timed {
    fn new(entity_id: u32, life: f32) -> Self {
        Self { entity_id, t: 0.0, life }
    }

    fn progress(&self) -> f32 {
        (self.t / self.life).clamp(0.0, 1.0)
    }
}

/// A minion lunging at what it attacks and settling back.
struct Bump {
    entity_id: u32,
    dir: Vec2,
    reach: f32, // px to travel — computed so the card edge just touches the target
    t: f32,
}

impl Bump {
    fn offset(&self) -> Vec2 {
        let frac = if self.t < 0.35 {
            let p = self.t / 0.35;
            p * (2.0 - p) // ease-out quad: 0 → 1
        } else {
            let p = (self.t - 0.35) / 0.65;
            (1.0 - p) * (1.0 - p) // ease-in quad: 1 → 0
        };
        self.dir * self.reach * frac
    }
}

/// A card turning face up as it lands in hand.
struct CardFlip {
    rotation_y: f32,
}

impl CardFlip {
    fn new_drawn() -> Self {
        Self { rotation_y: std::f32::consts::PI }
    }

    // exponential form: a texture preload can stall a frame for >0.25s, and the plain
    // dt lerp overshoots past that, leaving the card spinning at a sliver forever
    fn update(&mut self, dt: f32) {
        self.rotation_y += (0.0 - self.rotation_y) * (1.0 - (-FLIP_RATE * dt).exp());
    }
}

/// A number floating up off an entity. It carries its anchor, so a board that reflows mid-flight
/// cannot drag it sideways.
pub struct Number {
    pub pos: Vec2,
    pub text: String,
    pub color: Color,
    t: f32,
}

impl Number {
    fn progress(&self) -> f32 {
        (self.t / NUMBER_LIFE).clamp(0.0, 1.0)
    }

    pub fn offset(&self) -> Vec2 {
        Vec2::new(0.0, -NUMBER_RISE * self.progress())
    }

    pub fn alpha(&self) -> f32 {
        let p = self.progress();
        1.0 - p * p
    }
}

/// Everything per-entity the renderer looks up while drawing.
pub struct Overlay<'a> {
    offsets: Vec<(u32, Vec2)>,
    fades: Vec<(u32, f32)>,
    flashes: Vec<(u32, f32)>,
    hand_flips: Vec<(u32, f32)>,
    pub numbers: &'a [Number],
}

impl Overlay<'_> {
    pub fn offset(&self, entity_id: u32) -> Vec2 {
        lookup(&self.offsets, entity_id).unwrap_or(Vec2::ZERO)
    }

    /// 0 while alive, 1 once a dying minion has collapsed.
    pub fn fade(&self, entity_id: u32) -> f32 {
        lookup(&self.fades, entity_id).unwrap_or(0.0)
    }

    pub fn flash(&self, entity_id: u32) -> f32 {
        lookup(&self.flashes, entity_id).unwrap_or(0.0)
    }

    /// cos(rotation_y) of a card turning face up, 1 for a card that already is.
    pub fn hand_flip(&self, entity_id: u32) -> f32 {
        lookup(&self.hand_flips, entity_id).unwrap_or(1.0)
    }
}

fn lookup<T: Copy>(pairs: &[(u32, T)], entity_id: u32) -> Option<T> {
    pairs.iter().find(|(id, _)| *id == entity_id).map(|(_, v)| *v)
}

/// Owns every running animation and turns timeline events into new ones.
pub struct Anims {
    bumps: Vec<Bump>,
    fades: Vec<Timed>,
    flashes: Vec<Timed>,
    numbers: Vec<Number>,
    hand_flips: HashMap<u32, CardFlip>,
}

impl Anims {
    pub fn new() -> Self {
        Self { bumps: vec![], fades: vec![], flashes: vec![], numbers: vec![], hand_flips: HashMap::new() }
    }

    /// `state` is the board as it looks when the event starts, which is where the anchors and the
    /// travel distances come from — nothing it refers to has left play yet.
    pub fn spawn(&mut self, event: &GameEvent, state: &GameStateClient, w: f32, h: f32) {
        let life = timeline::duration(event);
        match event {
            GameEvent::Attacked { attacker_id, target_id } => {
                if let Some(bump) = bump(*attacker_id, *target_id, state, w, h) {
                    self.bumps.push(bump);
                }
            }
            GameEvent::Damaged { entity_id, amount } => {
                self.flashes.push(Timed::new(*entity_id, life));
                self.number(*entity_id, format!("-{amount}"), COL_DAMAGED, state, w, h);
            }
            GameEvent::Healed { entity_id, amount } => {
                self.number(*entity_id, format!("+{amount}"), COL_BUFFED, state, w, h);
            }
            GameEvent::Buffed { entity_id, attack, defence } => {
                self.number(*entity_id, format!("{attack:+}/{defence:+}"), COL_BUFFED, state, w, h);
            }
            GameEvent::Died { entity_id } => self.fades.push(Timed::new(*entity_id, life)),
            GameEvent::WardBroken { entity_id } => self.flashes.push(Timed::new(*entity_id, life)),
            GameEvent::TriggerFired { source_id } => self.flashes.push(Timed::new(*source_id, life)),
            GameEvent::OmenFired { entity_id } => self.flashes.push(Timed::new(*entity_id, life)),
            GameEvent::CardPlayed { .. } | GameEvent::MinionSummoned { .. } => {}
            GameEvent::ReturnedToHand { .. } | GameEvent::CardTraded { .. } => {}
            GameEvent::CardDrawn { .. } | GameEvent::CardCreated { .. } => {}
            GameEvent::TurnEnded => {}
        }
    }

    fn number(&mut self, entity_id: u32, text: String, color: Color, state: &GameStateClient, w: f32, h: f32) {
        if let Some(pos) = entity_center(entity_id, state, w, h) {
            self.numbers.push(Number { pos, text, color, t: 0.0 });
        }
    }

    /// `state` is the board being drawn, so a card appearing in hand picks up its flip here.
    pub fn tick(&mut self, dt: f32, state: &GameStateClient) {
        for b in self.bumps.iter_mut() {
            b.t = (b.t + dt / BUMP_TIME).min(1.0);
        }
        self.bumps.retain(|b| b.t < 1.0);

        for timed in self.fades.iter_mut().chain(self.flashes.iter_mut()) {
            timed.t += dt;
        }
        self.fades.retain(|f| f.t < f.life);
        self.flashes.retain(|f| f.t < f.life);

        for n in self.numbers.iter_mut() {
            n.t += dt;
        }
        self.numbers.retain(|n| n.t < NUMBER_LIFE);

        self.hand_flips.retain(|id, _| state.self_board.hand.iter().any(|c| c.entity_id() == *id));
        for card in &state.self_board.hand {
            self.hand_flips.entry(card.entity_id()).or_insert_with(CardFlip::new_drawn);
        }
        for flip in self.hand_flips.values_mut() {
            flip.update(dt);
        }
    }

    pub fn overlay(&self) -> Overlay<'_> {
        Overlay {
            offsets: self.bumps.iter().map(|b| (b.entity_id, b.offset())).collect(),
            fades: self.fades.iter().map(|f| (f.entity_id, f.progress())).collect(),
            // a half sine, so a flash swells and falls inside its own event
            flashes: self.flashes.iter().map(|f| (f.entity_id, (f.progress() * std::f32::consts::PI).sin())).collect(),
            hand_flips: self.hand_flips.iter().map(|(id, f)| (*id, f.rotation_y.cos())).collect(),
            numbers: &self.numbers,
        }
    }
}

/// Middle of an entity on either half of the board, hero included.
pub fn entity_center(entity_id: u32, state: &GameStateClient, w: f32, h: f32) -> Option<Vec2> {
    let halves = [
        (
            &state.self_board,
            layout::self_minion_rects(state.self_board.battlefield.len(), w, h),
            layout::self_hero_rect(w, h),
        ),
        (
            &state.enemy_board,
            layout::enemy_minion_rects(state.enemy_board.battlefield.len(), w, h),
            layout::enemy_hero_rect(w, h),
        ),
    ];
    for (board, rects, hero) in halves {
        if let Some((_, r)) = board.battlefield.iter().zip(rects.iter()).find(|(m, _)| m.entity_id == entity_id) {
            return Some(center(*r));
        }
        if board.hero.entity_id == entity_id {
            return Some(center(hero));
        }
    }
    None
}

fn center(r: Rect) -> Vec2 {
    Vec2::new(r.x + r.w / 2.0, r.y + r.h / 2.0)
}

/// Travels until the attacker's edge just meets the target's edge, from either half of the board.
fn bump(attacker_id: u32, target_id: u32, state: &GameStateClient, w: f32, h: f32) -> Option<Bump> {
    let from = entity_center(attacker_id, state, w, h)?;
    let to = entity_center(target_id, state, w, h)?;
    let delta = to - from;
    let dist = delta.length();
    if dist <= 0.0 {
        return None;
    }
    let reach = (dist - layout::MINION_W).max(20.0);
    Some(Bump { entity_id: attacker_id, dir: delta / dist, reach, t: 0.0 })
}
