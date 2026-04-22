#![allow(unused)]
#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum Race {
    Human,
    Elf,
    Beast,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum Trigger {
    OnPlay,
    OnDeath,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum TargetMode {
    Targeted, // player must choose a target
    Auto,     // resolves automatically
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum TargetSide {
    Friendly,
    Enemy,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum EntityType {
    Minion,
    Hero,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum TargetFilter {
    IsRace { race: Race },
    HasAttribute { attribute: MinionAttribute },
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TargetSpec {
    pub target_mode: TargetMode,
    pub side: TargetSide,
    pub entity_type: EntityType,
    pub filters: Vec<TargetFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum Requirement {
    Combo,
    Quickdraw,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum FollowUpRequirement {
    IsRace { race: Race },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum ScaledBy {
    MinionsOnBoard,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ScaledAmount {
    pub scalar: i32,
    pub scaled_by: Option<ScaledBy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum FollowUpEffect {
    Discount { scaled_amount: ScaledAmount },
    Copy { copy_amount: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct FollowUpAbility {
    pub follow_up_requirements: Vec<FollowUpRequirement>,
    pub follow_up_effects: Vec<FollowUpEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum Effect {
    Buff { target_spec: TargetSpec, attack: i32, defence: i32 },
    Damage { target_spec: TargetSpec, damage: i32, lifesteal: bool },
    Heal { target_spec: TargetSpec, heal: i32 },
    Draw { draw_amount: usize, follow_up: Option<FollowUpAbility> },
    ReturnToHand { target_spec: TargetSpec, cost_reduction: Option<i32> },
    Destroy { target_spec: TargetSpec },
    Summon { minion_card_id: u32, summon_amount: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Ability {
    pub trigger: Trigger,
    pub requirements: Vec<Requirement>, // empty vec = no requirements
    pub effects: Vec<Effect>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum MinionAttribute {
    Tradeable,
    Charge,
    Guard,
    Lifesteal,
    Poisonous,
    Ward,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum IncantationAttribute {
    Tradeable,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum Card {
    Minion(MinionCard),
    Incantation(IncantationCard),
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
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
    pub exhausted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct IncantationEntity {
    pub entity_id: u32,
    pub cost: i32,
    pub turns_in_hand: u32,
    pub just_drawn: bool,
    pub card: IncantationCard,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum CardEntity {
    Minion(MinionEntity),
    Incantation(IncantationEntity),
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Hero {
    pub entity_id: u32,
    pub attack: i32,
    pub defence: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Board {
    pub deck: Vec<CardEntity>,
    pub hand: Vec<CardEntity>,
    pub graveyard: Vec<MinionEntity>,
    pub battlefield: Vec<MinionEntity>,
    pub hero: Hero,
    pub base_mana: i32,
    pub mana: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GameStateServer {
    pub white: Board,
    pub black: Board,
    pub white_player_id: String,
    pub black_player_id: String,
    pub white_turn: bool,
    pub turn_count: u32,
    pub cards_played_this_turn: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GameStateClient {
    pub self_board: Board,
    pub enemy_board: Board,
    pub white_player_id: String,
    pub black_player_id: String,
    pub your_turn: bool,
    pub turn_count: u32,
    pub cards_played_this_turn: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TS)]
#[ts(export)]
pub struct PlayerMetaData {
    pub username: String,
    pub choosen_deck: Vec<u32>,
    pub avatar: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
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
        }
    }
    pub fn cost(&self) -> i32 {
        match self {
            CardEntity::Minion(m) => m.cost,
            CardEntity::Incantation(i) => i.cost,
        }
    }
    pub fn cost_mut(&mut self) -> &mut i32 {
        match self {
            CardEntity::Minion(m) => &mut m.cost,
            CardEntity::Incantation(i) => &mut i.cost,
        }
    }
    pub fn base_cost(&self) -> i32 {
        match self {
            CardEntity::Minion(m) => m.card.base_cost,
            CardEntity::Incantation(i) => i.card.base_cost,
        }
    }
    pub fn entity_id(&self) -> u32 {
        match self {
            CardEntity::Minion(m) => m.entity_id,
            CardEntity::Incantation(i) => i.entity_id,
        }
    }
    pub fn just_drawn(&self) -> bool {
        match self {
            CardEntity::Minion(m) => m.just_drawn,
            CardEntity::Incantation(i) => i.just_drawn,
        }
    }
    pub fn just_drawn_mut(&mut self) -> &mut bool {
        match self {
            CardEntity::Minion(m) => &mut m.just_drawn,
            CardEntity::Incantation(i) => &mut i.just_drawn,
        }
    }
    pub fn turns_in_hand_mut(&mut self) -> &mut u32 {
        match self {
            CardEntity::Minion(m) => &mut m.turns_in_hand,
            CardEntity::Incantation(i) => &mut i.turns_in_hand,
        }
    }
    pub fn abilities(&self) -> &[Ability] {
        match self {
            CardEntity::Minion(m) => &m.card.abilities,
            CardEntity::Incantation(i) => &i.card.abilities,
        }
    }
    pub fn is_tradeable(&self) -> bool {
        match self {
            CardEntity::Minion(m) => m.card.attributes.contains(&MinionAttribute::Tradeable),
            CardEntity::Incantation(i) => i.card.attributes.contains(&IncantationAttribute::Tradeable),
        }
    }
}
