#![allow(unused)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GamePhase {
    Mulligan,
    Playing,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Race {
    Human,
    Elf,
    Beast,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Trigger {
    OnPlay,
    OnDeath,
    OnOmenFired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TargetMode {
    Targeted, // player must choose a target
    Auto,     // resolves automatically
    SelfOnly, // the entity the ability sits on
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TargetSide {
    Friendly,
    Enemy,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntityType {
    Minion,
    Hero,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Condition {
    IsRace { race: Race },
    HasAttribute { attribute: MinionAttribute },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetSpec {
    pub target_mode: TargetMode,
    pub side: TargetSide,
    pub entity_type: EntityType,
    pub filters: Vec<Condition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Requirement {
    Combo,
    Quickdraw,
    IsHolding { filters: Vec<Condition> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScaledBy {
    MinionsOnBoard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaledAmount {
    pub scalar: i32,
    pub scaled_by: Option<ScaledBy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FollowUpEffect {
    Discount { scaled_amount: ScaledAmount },
    Copy { copy_amount: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowUpAbility {
    pub card_must_match: Vec<Condition>,
    pub follow_up_effects: Vec<FollowUpEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effect {
    Buff { target_spec: TargetSpec, attack: i32, defence: i32 },
    Damage { target_spec: TargetSpec, damage: i32, lifesteal: bool },
    Heal { target_spec: TargetSpec, heal: i32 },
    Draw { draw_amount: usize, follow_up: Option<FollowUpAbility> },
    ReturnToHand { target_spec: TargetSpec, cost_reduction: Option<i32> },
    Destroy { target_spec: TargetSpec },
    Summon { minion_card_id: u32, summon_amount: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub trigger: Trigger,
    pub requirements: Vec<Requirement>, // empty vec = no requirements
    pub effects: Vec<Effect>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MinionAttribute {
    Tradeable,
    Charge,
    Guard,
    Lifesteal,
    Poisonous,
    Ward,
    Stealth,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IncantationAttribute {
    Tradeable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinionCard {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub flavor_text: Option<String>,
    pub color: Color,
    pub base_cost: i32,
    pub image_url: String,
    pub abilities: Vec<Ability>,
    pub base_attack: i32,
    pub base_defence: i32,
    pub races: Vec<Race>,
    pub attributes: Vec<MinionAttribute>,
    pub is_token: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncantationCard {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub flavor_text: Option<String>,
    pub color: Color,
    pub base_cost: i32,
    pub image_url: String,
    pub abilities: Vec<Ability>,
    pub attributes: Vec<IncantationAttribute>,
    pub is_token: bool,
}

/// The event an omen watches for. Always read from the omen owner's side.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum OmenTrigger {
    EnemyPlaysMinion,
    EnemyPlaysIncantation,
    EnemyAttacksHero,
    EnemyAttacksMinion,
    FriendlyMinionDies,
    EnemyEndsTurn,
    EnemyEndsTurnWithoutAttacking,
    EnemyBoardReachesThree,
}

impl OmenTrigger {
    pub fn label(&self) -> &'static str {
        match self {
            OmenTrigger::EnemyPlaysMinion => "After the enemy plays a minion",
            OmenTrigger::EnemyPlaysIncantation => "After the enemy plays an incantation",
            OmenTrigger::EnemyAttacksHero => "After your hero is attacked",
            OmenTrigger::EnemyAttacksMinion => "After one of your minions is attacked",
            OmenTrigger::FriendlyMinionDies => "After one of your minions dies",
            OmenTrigger::EnemyEndsTurn => "When the enemy ends their turn",
            OmenTrigger::EnemyEndsTurnWithoutAttacking => "When the enemy ends their turn without attacking",
            OmenTrigger::EnemyBoardReachesThree => "After the enemy has three or more minions",
        }
    }
}

/// Effects are public card text, the armed trigger is not. They resolve on the
/// opponent's turn, so every effect must be auto-targeted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmenCard {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub flavor_text: Option<String>,
    pub color: Color,
    pub base_cost: i32,
    pub image_url: String,
    pub effects: Vec<Effect>,
    pub triggers: [OmenTrigger; 3],
    pub is_token: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmenEntity {
    pub entity_id: u32,
    pub cost: i32,
    pub turns_in_hand: u32,
    pub just_drawn: bool,
    pub card: OmenCard,
    // none in hand, and stripped before the opponent sees an armed one
    pub armed_trigger: Option<OmenTrigger>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Card {
    Minion(MinionCard),
    Incantation(IncantationCard),
    Omen(OmenCard),
}

impl Card {
    pub fn id(&self) -> u32 {
        match self {
            Card::Minion(m) => m.id,
            Card::Incantation(i) => i.id,
            Card::Omen(o) => o.id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinionEntity {
    pub entity_id: u32,
    pub cost: i32,
    pub turns_in_hand: u32,
    pub turns_on_board: u32,
    pub just_drawn: bool,
    pub card: MinionCard,
    pub attack: i32,
    pub defence: i32,
    pub max_defence: i32,
    pub ward_active: bool,
    pub stealth_active: bool,
    pub exhausted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncantationEntity {
    pub entity_id: u32,
    pub cost: i32,
    pub turns_in_hand: u32,
    pub just_drawn: bool,
    pub card: IncantationCard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CardEntity {
    Minion(MinionEntity),
    Incantation(IncantationEntity),
    Omen(OmenEntity),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hero {
    pub entity_id: u32,
    pub attack: i32,
    pub defence: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub deck: Vec<CardEntity>,
    pub hand: Vec<CardEntity>,
    pub graveyard: Vec<MinionEntity>,
    pub battlefield: Vec<MinionEntity>,
    pub omens: Vec<OmenEntity>,
    pub hero: Hero,
    pub base_mana: i32,
    pub mana: i32,
    pub embers: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStateServer {
    pub white: Board,
    pub black: Board,
    pub white_player_id: String,
    pub black_player_id: String,
    pub white_turn: bool,
    pub turn_count: u32,
    pub cards_played_this_turn: u32,
    pub attacked_this_turn: bool,
    pub phase: GamePhase,
    pub mulligan_white_done: bool,
    pub mulligan_black_done: bool,
}

/// Banner sent alongside each scenario frame so the client can narrate the run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioFrameInfo {
    pub case: String,
    pub group: String,
    pub label: String,
    pub kind: String,
    pub ok: bool,
    pub case_index: usize,
    pub case_total: usize,
    pub failed: usize,
    pub done: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardHint {
    pub playable: bool,
    pub condition_met: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStateClient {
    pub self_board: Board,
    pub enemy_board: Board,
    pub white_player_id: String,
    pub black_player_id: String,
    pub your_turn: bool,
    pub turn_count: u32,
    pub cards_played_this_turn: u32,
    pub phase: GamePhase,
    pub mulligan_submitted: bool,
    pub open_cards: bool,
    pub hand_hints: Vec<CardHint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerMetaData {
    pub username: String,
    pub chosen_deck: Vec<u32>,
    pub avatar: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackData {
    pub origin_id: u32,
    pub target_id: u32,
}

impl MinionEntity {
    pub fn card_id(&self) -> u32 {
        self.card.id
    }
}

impl CardEntity {
    pub fn card_id(&self) -> u32 {
        match self {
            CardEntity::Minion(m) => m.card.id,
            CardEntity::Incantation(i) => i.card.id,
            CardEntity::Omen(o) => o.card.id,
        }
    }
    pub fn cost(&self) -> i32 {
        match self {
            CardEntity::Minion(m) => m.cost,
            CardEntity::Incantation(i) => i.cost,
            CardEntity::Omen(o) => o.cost,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            CardEntity::Minion(m) => &m.card.name,
            CardEntity::Incantation(i) => &i.card.name,
            CardEntity::Omen(o) => &o.card.name,
        }
    }
    pub fn cost_mut(&mut self) -> &mut i32 {
        match self {
            CardEntity::Minion(m) => &mut m.cost,
            CardEntity::Incantation(i) => &mut i.cost,
            CardEntity::Omen(o) => &mut o.cost,
        }
    }
    pub fn base_cost(&self) -> i32 {
        match self {
            CardEntity::Minion(m) => m.card.base_cost,
            CardEntity::Incantation(i) => i.card.base_cost,
            CardEntity::Omen(o) => o.card.base_cost,
        }
    }
    pub fn entity_id(&self) -> u32 {
        match self {
            CardEntity::Minion(m) => m.entity_id,
            CardEntity::Incantation(i) => i.entity_id,
            CardEntity::Omen(o) => o.entity_id,
        }
    }
    pub fn just_drawn(&self) -> bool {
        match self {
            CardEntity::Minion(m) => m.just_drawn,
            CardEntity::Incantation(i) => i.just_drawn,
            CardEntity::Omen(o) => o.just_drawn,
        }
    }
    pub fn just_drawn_mut(&mut self) -> &mut bool {
        match self {
            CardEntity::Minion(m) => &mut m.just_drawn,
            CardEntity::Incantation(i) => &mut i.just_drawn,
            CardEntity::Omen(o) => &mut o.just_drawn,
        }
    }
    pub fn turns_in_hand_mut(&mut self) -> &mut u32 {
        match self {
            CardEntity::Minion(m) => &mut m.turns_in_hand,
            CardEntity::Incantation(i) => &mut i.turns_in_hand,
            CardEntity::Omen(o) => &mut o.turns_in_hand,
        }
    }
    pub fn abilities(&self) -> &[Ability] {
        match self {
            CardEntity::Minion(m) => &m.card.abilities,
            CardEntity::Incantation(i) => &i.card.abilities,
            CardEntity::Omen(_) => &[],
        }
    }
    pub fn is_tradeable(&self) -> bool {
        match self {
            CardEntity::Minion(m) => m.card.attributes.contains(&MinionAttribute::Tradeable),
            CardEntity::Incantation(i) => i.card.attributes.contains(&IncantationAttribute::Tradeable),
            CardEntity::Omen(_) => false,
        }
    }
}
