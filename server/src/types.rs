#![allow(unused)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Race {
    Human,
    Elf,
    Beast,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Trigger {
    OnPlay,
    OnDeath,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TargetMode {
    Targeted, // player must choose a target
    Auto,     // resolves automatically
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TargetSide {
    Friendly,
    Enemy,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum EntityType {
    Minion,
    Hero,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TargetSpec {
    pub targeted: bool,
    pub side: TargetSide,
    pub entity_type: EntityType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Requirement {
    Combo,
    Quickdraw,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum FollowUpRequirement {
    IsRace { race: Race },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ScaledBy {
    MinionsOnBoard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaledAmount {
    pub scalar: i32,
    pub scaled_by: Option<ScaledBy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum FollowUpEffect {
    Discount { scaled_amount: ScaledAmount },
    Copy { copy_amount: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowUpAbility {
    pub follow_up_requirements: Vec<FollowUpRequirement>,
    pub follow_up_effects: Vec<FollowUpEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Effect {
    Buff {
        target_spec: TargetSpec,
        attack: i32,
        defence: i32,
    },
    Damage {
        target_spec: TargetSpec,
        damage: i32,
    },
    Draw {
        draw_amount: usize,
        follow_up: Option<FollowUpAbility>,
    },
    ReturnToHand {
        target_spec: TargetSpec,
        cost_reduction: Option<i32>,
    },
    Destroy {
        target_spec: TargetSpec,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    pub trigger: Trigger,
    pub requirements: Vec<Requirement>, // empty vec = no requirements
    pub effects: Vec<Effect>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum MinionAttribute {
    Tradeable,
    Charge,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum IncantationAttribute {
    Tradeable,
    Twinspell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinionCard {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub color: Color,
    pub base_cost: i32,
    pub image_url: String,
    pub abilities: Vec<Ability>,
    pub base_attack: i32,
    pub base_defence: i32,
    pub races: Vec<Race>,
    pub attributes: Vec<MinionAttribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncantationCard {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub color: Color,
    pub base_cost: i32,
    pub image_url: String,
    pub abilities: Vec<Ability>,
    pub attributes: Vec<IncantationAttribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Card {
    Minion(MinionCard),
    Incantation(IncantationCard),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityBase {
    pub entity_id: String,
    pub cost: i32,
    pub turns_in_hand: u32,
    pub just_drawn: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinionEntity {
    pub card: MinionCard,
    pub base: EntityBase,
    pub attack: i32,
    pub defence: i32,
    pub exhausted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncantationEntity {
    pub card: IncantationCard,
    pub base: EntityBase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CardEntity {
    Minion(MinionEntity),
    Incantation(IncantationEntity),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hero {
    pub attack: i32,
    pub defence: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub deck: Vec<CardEntity>,
    pub hand: Vec<CardEntity>,
    pub graveyard: Vec<MinionEntity>,
    pub battlefield: Vec<MinionEntity>,
    pub hero: Hero,
    pub base_mana: i32,
    pub mana: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub white: Board,
    pub black: Board,
    pub white_player_id: String,
    pub black_player_id: String,
    pub white_turn: bool,
    pub turn_count: u32,
    pub cards_played_this_turn: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStateClient {
    #[serde(rename = "self")]
    pub self_board: Board,
    pub enemy: Board,
    pub white_player_id: String,
    pub black_player_id: String,
    pub your_turn: bool,
    pub turn_count: u32,
    pub cards_played_this_turn: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMetaData {
    pub username: String,
    pub chosen_deck: Vec<u32>,
    pub avatar: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackData {
    pub origin_id: String,
    pub target_id: String,
}

impl CardEntity {
    pub fn cost(&self) -> i32 {
        match self {
            CardEntity::Minion(m) => m.base.cost,
            CardEntity::Incantation(i) => i.base.cost,
        }
    }
    pub fn cost_mut(&mut self) -> &mut i32 {
        match self {
            CardEntity::Minion(m) => &mut m.base.cost,
            CardEntity::Incantation(i) => &mut i.base.cost,
        }
    }
    pub fn entity_id(&self) -> &str {
        match self {
            CardEntity::Minion(m) => &m.base.entity_id,
            CardEntity::Incantation(i) => &i.base.entity_id,
        }
    }
    pub fn just_drawn(&self) -> bool {
        match self {
            CardEntity::Minion(m) => m.base.just_drawn,
            CardEntity::Incantation(i) => i.base.just_drawn,
        }
    }
    pub fn just_drawn_mut(&mut self) -> &mut bool {
        match self {
            CardEntity::Minion(m) => &mut m.base.just_drawn,
            CardEntity::Incantation(i) => &mut i.base.just_drawn,
        }
    }
    pub fn turns_in_hand_mut(&mut self) -> &mut u32 {
        match self {
            CardEntity::Minion(m) => &mut m.base.turns_in_hand,
            CardEntity::Incantation(i) => &mut i.base.turns_in_hand,
        }
    }
    pub fn abilities(&self) -> &[Ability] {
        match self {
            CardEntity::Minion(m) => &m.card.abilities,
            CardEntity::Incantation(i) => &i.card.abilities,
        }
    }
    pub fn base_cost(&self) -> i32 {
        match self {
            CardEntity::Minion(m) => m.card.base_cost,
            CardEntity::Incantation(i) => i.card.base_cost,
        }
    }
    pub fn is_tradeable(&self) -> bool {
        match self {
            CardEntity::Minion(m) => m.card.attributes.contains(&MinionAttribute::Tradeable),
            CardEntity::Incantation(i) => i.card.attributes.contains(&IncantationAttribute::Tradeable),
        }
    }
}
