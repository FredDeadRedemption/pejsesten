//! Card builders for scenarios.
//!
//! Deliberately independent of `shared::cards`: every scenario mints the exact card it needs,
//! so the real card pool can change freely without touching the tests.
#![allow(dead_code)]

use shared::types::*;

/// Scenario cards live in a high id range so they can never collide with `cards.rs`.
pub const SCENARIO_ID_BASE: u32 = 900_000;

pub fn minion(id: u32, name: &str, cost: i32, attack: i32, defence: i32) -> MinionCard {
    MinionCard {
        id: SCENARIO_ID_BASE + id,
        name: name.to_string(),
        description: None,
        flavor_text: None,
        color: Color::White,
        base_cost: cost,
        image_url: String::new(),
        abilities: vec![],
        base_attack: attack,
        base_defence: defence,
        races: vec![],
        attributes: vec![],
        is_token: false,
    }
}

pub fn incantation(id: u32, name: &str, cost: i32) -> IncantationCard {
    IncantationCard {
        id: SCENARIO_ID_BASE + id,
        name: name.to_string(),
        description: None,
        flavor_text: None,
        color: Color::White,
        base_cost: cost,
        image_url: String::new(),
        abilities: vec![],
        attributes: vec![],
        is_token: false,
    }
}

pub trait MinionBuild: Sized {
    fn with(self, a: MinionAttribute) -> Self;
    fn of(self, r: Race) -> Self;
    fn does(self, a: Ability) -> Self;
}

impl MinionBuild for MinionCard {
    fn with(mut self, a: MinionAttribute) -> Self {
        self.attributes.push(a);
        self
    }
    fn of(mut self, r: Race) -> Self {
        self.races.push(r);
        self
    }
    fn does(mut self, a: Ability) -> Self {
        self.abilities.push(a);
        self
    }
}

pub trait IncantationBuild: Sized {
    fn with(self, a: IncantationAttribute) -> Self;
    fn does(self, a: Ability) -> Self;
}

impl IncantationBuild for IncantationCard {
    fn with(mut self, a: IncantationAttribute) -> Self {
        self.attributes.push(a);
        self
    }
    fn does(mut self, a: Ability) -> Self {
        self.abilities.push(a);
        self
    }
}

// abilities

pub fn on_play(effects: Vec<Effect>) -> Ability {
    Ability { trigger: Trigger::OnPlay, requirements: vec![], effects }
}

pub fn on_death(effects: Vec<Effect>) -> Ability {
    Ability { trigger: Trigger::OnDeath, requirements: vec![], effects }
}

pub fn requiring(mut a: Ability, r: Requirement) -> Ability {
    a.requirements.push(r);
    a
}

// target specs

pub fn spec(target_mode: TargetMode, side: TargetSide, entity_type: EntityType) -> TargetSpec {
    TargetSpec { target_mode, side, entity_type, filters: vec![] }
}

pub fn filtered(mut s: TargetSpec, c: Condition) -> TargetSpec {
    s.filters.push(c);
    s
}

/// Shorthand: a single chosen enemy minion.
pub fn pick_enemy_minion() -> TargetSpec {
    spec(TargetMode::Targeted, TargetSide::Enemy, EntityType::Minion)
}

/// Shorthand: a single chosen friendly minion.
pub fn pick_friendly_minion() -> TargetSpec {
    spec(TargetMode::Targeted, TargetSide::Friendly, EntityType::Minion)
}

/// Shorthand: every minion on both boards.
pub fn all_minions() -> TargetSpec {
    spec(TargetMode::Auto, TargetSide::All, EntityType::Minion)
}

// effects

pub fn damage(target_spec: TargetSpec, damage: i32) -> Effect {
    Effect::Damage { target_spec, damage, lifesteal: false }
}

pub fn drain(target_spec: TargetSpec, damage: i32) -> Effect {
    Effect::Damage { target_spec, damage, lifesteal: true }
}

pub fn heal(target_spec: TargetSpec, heal: i32) -> Effect {
    Effect::Heal { target_spec, heal }
}

pub fn buff(target_spec: TargetSpec, attack: i32, defence: i32) -> Effect {
    Effect::Buff { target_spec, attack, defence }
}

pub fn destroy(target_spec: TargetSpec) -> Effect {
    Effect::Destroy { target_spec }
}

pub fn draw(draw_amount: usize) -> Effect {
    Effect::Draw { draw_amount, follow_up: None }
}

pub fn draw_then(draw_amount: usize, follow_up: FollowUpAbility) -> Effect {
    Effect::Draw { draw_amount, follow_up: Some(follow_up) }
}

pub fn bounce(target_spec: TargetSpec, cost_reduction: Option<i32>) -> Effect {
    Effect::ReturnToHand { target_spec, cost_reduction }
}

pub fn summon(card: &MinionCard, summon_amount: usize) -> Effect {
    Effect::Summon { minion_card_id: card.id, summon_amount }
}

pub fn follow_up(card_must_match: Vec<Condition>, follow_up_effects: Vec<FollowUpEffect>) -> FollowUpAbility {
    FollowUpAbility { card_must_match, follow_up_effects }
}

pub fn discount(scalar: i32, scaled_by: Option<ScaledBy>) -> FollowUpEffect {
    FollowUpEffect::Discount { scaled_amount: ScaledAmount { scalar, scaled_by } }
}

pub fn copies(copy_amount: usize) -> FollowUpEffect {
    FollowUpEffect::Copy { copy_amount }
}
