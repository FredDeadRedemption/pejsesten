// cards.rs

use crate::types::*;
use std::collections::HashMap;
use std::sync::LazyLock;

static CARDS: LazyLock<HashMap<u32, Card>> = LazyLock::new(|| build_cards().into_iter().map(|c| (c.id(), c)).collect());

pub fn get_card_by_id(id: u32) -> Option<&'static Card> {
    CARDS.get(&id)
}

pub fn get_collectible_cards() -> Vec<Card> {
    get_all_cards()
        .into_iter()
        .filter(|c| match c {
            Card::Minion(m) => !m.is_token,
            Card::Incantation(i) => !i.is_token,
            Card::Hero(_) => false,
        })
        .collect()
}

pub fn get_all_cards() -> Vec<Card> {
    CARDS.values().cloned().collect()
}

/// Heroes sit in their own id band of the pool so the card ids stay contiguous.
pub const DEFAULT_HERO_ID: u32 = 104;

pub fn get_hero_by_id(id: u32) -> Option<&'static HeroCard> {
    match get_card_by_id(id) {
        Some(Card::Hero(h)) => Some(h),
        _ => None,
    }
}

/// Falls back to the default hero whenever an id is unknown, including the unset 0.
pub fn hero_or_default(id: u32) -> &'static HeroCard {
    get_hero_by_id(id)
        .or_else(|| get_hero_by_id(DEFAULT_HERO_ID))
        .expect("the default hero must be in the pool")
}

pub fn get_heroes() -> Vec<HeroCard> {
    let mut heroes: Vec<HeroCard> = CARDS
        .values()
        .filter_map(|c| match c {
            Card::Hero(h) => Some(h.clone()),
            _ => None,
        })
        .collect();
    heroes.sort_by_key(|h| h.id);
    heroes
}

pub fn instantiate_minion(c: &MinionCard, entity_id: u32) -> MinionEntity {
    MinionEntity {
        stats: Stats::new(c.base_attack, c.base_defence),
        damage: 0,
        buff: Buff::default(),
        doomed: false,
        ward_active: c.attributes.contains(&MinionAttribute::Ward),
        stealth_active: c.attributes.contains(&MinionAttribute::Stealth),
        exhausted: false,
        entity_id,
        cost: c.base_cost,
        discount: 0,
        turns_on_board: 0,
        arrived_on_turn: None,
        card: c.clone(),
    }
}

pub fn instantiate_incantation(c: &IncantationCard, entity_id: u32) -> IncantationEntity {
    IncantationEntity {
        entity_id,
        cost: c.base_cost,
        discount: 0,
        arrived_on_turn: None,
        armed_trigger: None,
        turns_left: None,
        card: c.clone(),
    }
}

pub fn instantiate_minion_by_id(card_id: u32, entity_id: u32) -> Option<MinionEntity> {
    match get_card_by_id(card_id)? {
        Card::Minion(c) => Some(instantiate_minion(c, entity_id)),
        Card::Incantation(_) | Card::Hero(_) => None, // only minions can be summoned
    }
}

/// Token ids the summon effects point at, in their own band so a collectible id can never
/// land on one and overwrite it in the catalogue.
const RAT_ID: u32 = 201;
const BRAMBLE_ID: u32 = 203;
const BIG_RAT_ID: u32 = 204;

pub fn build_cards() -> Vec<Card> {
    vec![
        // ── GUNSLINGER (BLACK) ────────────────────────────────────────
        Card::Incantation(IncantationCard {
            id: 25,
            color: Color::Black,
            name: "Quick Shot".to_string(),
            description: Some("<strong>Tradeable</strong><br>Deal 1 damage to an enemy<br><strong>Quickdraw:</strong> Deal 2 more".to_string()),
            flavor_text: Some("Fired before the drum roll finished.".to_string()),
            base_cost: 1,
            image_url: "Quick Shot.png".to_string(),
            form: IncantationForm::Cast,
            passives: vec![],
            attributes: vec![IncantationAttribute::Tradeable],
            abilities: vec![
                Ability {
                    trigger: Trigger::OnPlay,
                    requirements: vec![],
                    effects: vec![Effect::Damage {
                        target_spec: TargetSpec {
                            target_mode: TargetMode::Targeted,
                            side: TargetSide::Enemy,
                            entity_type: EntityType::All,
                            filters: vec![],
                        },
                        damage: 1,
                        lifesteal: false,
                    }],
                },
                Ability {
                    trigger: Trigger::OnPlay,
                    requirements: vec![Requirement::Quickdraw],
                    effects: vec![Effect::Damage {
                        target_spec: TargetSpec {
                            target_mode: TargetMode::Targeted,
                            side: TargetSide::Enemy,
                            entity_type: EntityType::All,
                            filters: vec![],
                        },
                        damage: 2,
                        lifesteal: false,
                    }],
                },
            ],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 27,
            color: Color::Black,
            name: "Knife Thrower".to_string(),
            description: Some("<strong>Tradeable</strong><br><strong>Quickdraw:</strong> Deal 2 damage to an enemy minion".to_string()),
            flavor_text: None,
            base_cost: 2,
            image_url: "Knife Thrower.png".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 2,
            base_defence: 2,
            attributes: vec![MinionAttribute::Tradeable],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![Requirement::Quickdraw],
                effects: vec![Effect::Damage {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Targeted,
                        side: TargetSide::Enemy,
                        entity_type: EntityType::Minion,
                        filters: vec![],
                    },
                    damage: 2,
                    lifesteal: false,
                }],
            }],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 32,
            color: Color::Black,
            name: "Hat Trick".to_string(),
            description: Some("Draw a card<br><strong>Quickdraw:</strong> Add a copy of it to your hand".to_string()),
            flavor_text: None,
            base_cost: 2,
            image_url: "Hat Trick.png".to_string(),
            form: IncantationForm::Cast,
            passives: vec![],
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Draw {
                    draw_amount: 1,
                    follow_up: Some(FollowUpAbility {
                        card_must_match: vec![],
                        requirements: vec![Requirement::Quickdraw],
                        follow_up_effects: vec![FollowUpEffect::Copy { copy_amount: 1 }],
                    }),
                }],
            }],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 38,
            color: Color::Black,
            name: "Knife Juggle".to_string(),
            description: Some("Deal 2 damage to all minions".to_string()),
            flavor_text: Some("Eleven in the air, and he is counting.".to_string()),
            base_cost: 4,
            image_url: "".to_string(),
            form: IncantationForm::Cast,
            passives: vec![],
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Damage {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Auto,
                        side: TargetSide::All,
                        entity_type: EntityType::Minion,
                        filters: vec![],
                    },
                    damage: 2,
                    lifesteal: false,
                }],
            }],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 59,
            color: Color::Black,
            name: "Bottom Deal".to_string(),
            description: Some("Reverse your deck<br>Draw a card".to_string()),
            flavor_text: Some("The deck has two tops if you hold it right.".to_string()),
            base_cost: 2,
            image_url: "Card Shark.png".to_string(),
            form: IncantationForm::Cast,
            passives: vec![],
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![
                    Effect::ReverseDeck,
                    Effect::Draw { draw_amount: 1, follow_up: None },
                ],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 41,
            color: Color::Black,
            name: "Dust Jackal".to_string(),
            description: Some("<strong>Blitz</strong>".to_string()),
            flavor_text: Some("Arrives with the dust, leaves before it settles.".to_string()),
            base_cost: 1,
            image_url: "Dust Jackal.png".to_string(),
            races: vec![Race::Beast],
            passives: vec![],
            base_attack: 1,
            base_defence: 1,
            attributes: vec![MinionAttribute::Charge],
            abilities: vec![],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 42,
            color: Color::Black,
            name: "Gunsmith".to_string(),
            description: Some("Whenever you land a <strong>Quickdraw</strong>, gain +1 +1".to_string()),
            flavor_text: Some("Builds them, and knows exactly how fast they are.".to_string()),
            base_cost: 1,
            image_url: "Gunsmith.png".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 1,
            base_defence: 3,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnQuickdraw,
                requirements: vec![],
                effects: vec![Effect::Buff {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::SelfOnly,
                        side: TargetSide::Friendly,
                        entity_type: EntityType::Minion,
                        filters: vec![],
                    },
                    attack: 1,
                    defence: 1,
                }],
            }],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 43,
            color: Color::Black,
            name: "Toad".to_string(),
            description: Some("<strong>Deathwish:</strong> Add a random omen to your hand".to_string()),
            flavor_text: Some("Swallowed something it should not have.".to_string()),
            base_cost: 1,
            image_url: "Toad.png".to_string(),
            races: vec![Race::Beast],
            passives: vec![],
            base_attack: 1,
            base_defence: 1,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnDeath,
                requirements: vec![],
                effects: vec![Effect::AddRandomOmen],
            }],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 44,
            color: Color::Black,
            name: "Rat Cage".to_string(),
            description: Some("<strong>Deathwish:</strong> <strong>Summon</strong> three 1/1 rats".to_string()),
            flavor_text: Some("Despite all their rage they are still just rats in a cage.".to_string()),
            base_cost: 1,
            image_url: "Rat Cage.png".to_string(),
            races: vec![],
            passives: vec![],
            base_attack: 0,
            base_defence: 3,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnDeath,
                requirements: vec![],
                effects: vec![Effect::Summon {
                    summon_amount: 3,
                    minion_card_id: RAT_ID,
                }],
            }],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 45,
            color: Color::Black,
            name: "Card Shark".to_string(),
            description: Some("<strong>Guard</strong><br><strong>Fanfare:</strong> Reverse your deck".to_string()),
            flavor_text: Some("Deals from whichever end pays better.".to_string()),
            base_cost: 2,
            image_url: "Card Shark.png".to_string(),
            races: vec![Race::Beast],
            passives: vec![],
            base_attack: 2,
            base_defence: 2,
            attributes: vec![MinionAttribute::Guard],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::ReverseDeck],
            }],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 46,
            color: Color::Black,
            name: "Gunslinger".to_string(),
            description: Some("Whenever you land a <strong>Quickdraw</strong>, deal 1 damage to all enemies".to_string()),
            flavor_text: Some("Counts the shots out loud. Everyone else counts with them.".to_string()),
            base_cost: 2,
            image_url: "".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 2,
            base_defence: 4,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnQuickdraw,
                requirements: vec![],
                effects: vec![Effect::Damage {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Auto,
                        side: TargetSide::Enemy,
                        entity_type: EntityType::All,
                        filters: vec![],
                    },
                    damage: 1,
                    lifesteal: false,
                }],
            }],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 47,
            color: Color::Black,
            name: "Shiny Magpie".to_string(),
            description: Some("<strong>Fanfare:</strong> If you already played a card that arrived this turn, gain +1 +1".to_string()),
            flavor_text: Some("Takes the brightest thing on the table.".to_string()),
            base_cost: 2,
            image_url: "Shiny Magpie.png".to_string(),
            races: vec![Race::Beast],
            passives: vec![],
            base_attack: 2,
            base_defence: 2,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![Requirement::ArrivalPlayedThisTurn],
                effects: vec![Effect::Buff {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::SelfOnly,
                        side: TargetSide::Friendly,
                        entity_type: EntityType::Minion,
                        filters: vec![],
                    },
                    attack: 1,
                    defence: 1,
                }],
            }],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 49,
            color: Color::Black,
            name: "Fox Courier".to_string(),
            description: Some("<strong>Fanfare:</strong> If you control another beast, draw a card".to_string()),
            flavor_text: Some("Paid in eggs, never late.".to_string()),
            base_cost: 3,
            image_url: "Fox Courier.png".to_string(),
            races: vec![Race::Beast],
            passives: vec![],
            base_attack: 2,
            base_defence: 3,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![Requirement::ControlsOther {
                    filters: vec![Condition::IsRace { race: Race::Beast }],
                }],
                effects: vec![Effect::Draw { draw_amount: 1, follow_up: None }],
            }],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 50,
            color: Color::Black,
            name: "Bristleback".to_string(),
            description: Some("<strong>Guard</strong>".to_string()),
            flavor_text: Some("Stands in the road and dares the wagon.".to_string()),
            base_cost: 3,
            image_url: "".to_string(),
            races: vec![Race::Beast],
            passives: vec![],
            base_attack: 3,
            base_defence: 3,
            attributes: vec![MinionAttribute::Guard],
            abilities: vec![],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 51,
            color: Color::Black,
            name: "Fat Rat".to_string(),
            description: Some("<strong>Tradeable</strong><br><strong>Quickdraw:</strong> <strong>Summon</strong> a 2/2 rat".to_string()),
            flavor_text: Some("Ate the evidence.".to_string()),
            base_cost: 4,
            image_url: "Fat Rat.png".to_string(),
            races: vec![Race::Beast],
            passives: vec![],
            base_attack: 3,
            base_defence: 3,
            attributes: vec![MinionAttribute::Tradeable],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![Requirement::Quickdraw],
                effects: vec![Effect::Summon {
                    summon_amount: 1,
                    minion_card_id: BIG_RAT_ID,
                }],
            }],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 52,
            color: Color::Black,
            name: "Bear".to_string(),
            description: Some("<strong>Tradeable</strong>".to_string()),
            flavor_text: Some("No act, no cage, no leash.".to_string()),
            base_cost: 5,
            image_url: "Bear.png".to_string(),
            races: vec![Race::Beast],
            passives: vec![],
            base_attack: 5,
            base_defence: 6,
            attributes: vec![MinionAttribute::Tradeable],
            abilities: vec![],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 53,
            color: Color::Black,
            name: "Gazelle".to_string(),
            description: Some("<strong>Blitz</strong><br>Costs 1 less for each <strong>Quickdraw</strong> you landed".to_string()),
            flavor_text: Some("Gone by the time the shot is heard.".to_string()),
            base_cost: 6,
            image_url: "Gazelle.png".to_string(),
            races: vec![Race::Beast],
            passives: vec![],
            base_attack: 4,
            base_defence: 4,
            attributes: vec![MinionAttribute::Charge],
            abilities: vec![],
            cost_scaling: Some(ScaledAmount {
                scalar: 1,
                scaled_by: Some(ScaledBy::Counter { counter: Counter::QuickdrawsPlayed }),
            }),
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 54,
            color: Color::Black,
            name: "The Dentist".to_string(),
            description: Some("<strong>Fanfare:</strong> Replay every <strong>Quickdraw</strong> you landed this game".to_string()),
            flavor_text: Some("Open wide.".to_string()),
            base_cost: 7,
            image_url: "The Dentist.png".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 7,
            base_defence: 7,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::ReplayQuickdraws],
            }],
            cost_scaling: None,
            is_token: false,
        }),
        // ── THORNWAKE (GREEN) ─────────────────────────────────────────
        Card::Minion(MinionCard {
            id: 62,
            color: Color::Green,
            name: "Briar Elf".to_string(),
            description: Some("<strong>Tradeable</strong>".to_string()),
            flavor_text: Some("Grew up in the hedge and never left it.".to_string()),
            base_cost: 1,
            image_url: "".to_string(),
            races: vec![Race::Elf],
            passives: vec![],
            base_attack: 1,
            base_defence: 2,
            attributes: vec![MinionAttribute::Tradeable],
            abilities: vec![],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 63,
            color: Color::Green,
            name: "Grovewarden".to_string(),
            description: Some("<strong>Guard</strong>".to_string()),
            flavor_text: Some("Counts the rings and lets nobody past.".to_string()),
            base_cost: 2,
            image_url: "".to_string(),
            races: vec![Race::Elf],
            passives: vec![],
            base_attack: 1,
            base_defence: 3,
            attributes: vec![MinionAttribute::Guard],
            abilities: vec![],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 64,
            color: Color::Green,
            name: "Thorn Dancer".to_string(),
            description: Some("<strong>Blitz</strong>".to_string()),
            flavor_text: Some("Barefoot, and the brambles move aside.".to_string()),
            base_cost: 2,
            image_url: "".to_string(),
            races: vec![Race::Elf],
            passives: vec![],
            base_attack: 2,
            base_defence: 2,
            attributes: vec![MinionAttribute::Charge],
            abilities: vec![],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 65,
            color: Color::Green,
            name: "Thicket Nanny".to_string(),
            description: Some("<strong>Fanfare:</strong> <strong>Summon</strong> a 0/3 bramble with <strong>Guard</strong>".to_string()),
            flavor_text: Some("Raises them from cuttings and never asks whose.".to_string()),
            base_cost: 3,
            image_url: "".to_string(),
            races: vec![Race::Elf],
            passives: vec![],
            base_attack: 2,
            base_defence: 3,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Summon { summon_amount: 1, minion_card_id: BRAMBLE_ID }],
            }],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 66,
            color: Color::Green,
            name: "Greenmantle".to_string(),
            description: Some("Your elves have +1 +1".to_string()),
            flavor_text: Some("Human, and they let her carry the banner anyway.".to_string()),
            base_cost: 4,
            image_url: "".to_string(),
            races: vec![Race::Human],
            passives: vec![PassiveEffect::Aura {
                side: TargetSide::Friendly,
                filters: vec![Condition::IsRace { race: Race::Elf }],
                attack: 1,
                defence: 1,
            }],
            base_attack: 3,
            base_defence: 3,
            attributes: vec![],
            abilities: vec![],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 67,
            color: Color::Green,
            name: "Elder Oak".to_string(),
            description: Some("<strong>Guard</strong>".to_string()),
            flavor_text: Some("Older than the road, and in the way of it.".to_string()),
            base_cost: 6,
            image_url: "".to_string(),
            races: vec![Race::Elf],
            passives: vec![],
            base_attack: 5,
            base_defence: 7,
            attributes: vec![MinionAttribute::Guard],
            abilities: vec![],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 68,
            color: Color::Green,
            name: "Quickening".to_string(),
            description: Some("Give a friendly minion +2 +2".to_string()),
            flavor_text: Some("One season's growth, spent in an afternoon.".to_string()),
            base_cost: 1,
            image_url: "".to_string(),
            form: IncantationForm::Cast,
            passives: vec![],
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Buff {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Targeted,
                        side: TargetSide::Friendly,
                        entity_type: EntityType::Minion,
                        filters: vec![],
                    },
                    attack: 2,
                    defence: 2,
                }],
            }],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 69,
            color: Color::Green,
            name: "Thornwatch".to_string(),
            description: Some("<strong>Summon</strong> two 0/3 brambles with <strong>Guard</strong>".to_string()),
            flavor_text: Some("The hedge was thinner yesterday.".to_string()),
            base_cost: 2,
            image_url: "".to_string(),
            form: IncantationForm::Omen {
                triggers: [RiteTrigger::EnemyPlaysMinion, RiteTrigger::EnemyAttacksMinion, RiteTrigger::EnemyEndsTurn],
                effects: vec![Effect::Summon { summon_amount: 2, minion_card_id: BRAMBLE_ID }],
            },
            abilities: vec![],
            passives: vec![],
            attributes: vec![],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 70,
            color: Color::Green,
            name: "Hold the Grove".to_string(),
            description: Some("<strong>Wager:</strong> if you cast an incantation this turn, I draw two cards".to_string()),
            flavor_text: Some("Read it, then decide what your spell is worth.".to_string()),
            base_cost: 2,
            image_url: "".to_string(),
            form: IncantationForm::Wager {
                trigger: RiteTrigger::EnemyPlaysIncantation,
                effects: vec![Effect::Draw { draw_amount: 2, follow_up: None }],
            },
            abilities: vec![],
            passives: vec![],
            attributes: vec![],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 71,
            color: Color::Green,
            name: "Wildgrowth".to_string(),
            description: Some("<strong>Aura:</strong> your minions have +1 +1".to_string()),
            flavor_text: Some("Three turns of summer, and the hedge remembers none of it.".to_string()),
            base_cost: 3,
            image_url: "".to_string(),
            form: IncantationForm::Aura { turns: 3 },
            abilities: vec![],
            passives: vec![PassiveEffect::Aura {
                side: TargetSide::Friendly,
                filters: vec![],
                attack: 1,
                defence: 1,
            }],
            attributes: vec![],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 72,
            color: Color::Green,
            name: "Greenwake".to_string(),
            description: Some("<strong>Fanfare:</strong> give a friendly elf +2 +2<br><strong>Aura:</strong> your elves have +2 +2".to_string()),
            flavor_text: Some("What the wood gives back, it gives to one of you for good.".to_string()),
            base_cost: 4,
            image_url: "".to_string(),
            form: IncantationForm::Aura { turns: 3 },
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Buff {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Targeted,
                        side: TargetSide::Friendly,
                        entity_type: EntityType::Minion,
                        filters: vec![Condition::IsRace { race: Race::Elf }],
                    },
                    attack: 2,
                    defence: 2,
                }],
            }],
            passives: vec![PassiveEffect::Aura {
                side: TargetSide::Friendly,
                filters: vec![Condition::IsRace { race: Race::Elf }],
                attack: 2,
                defence: 2,
            }],
            attributes: vec![],
            is_token: false,
        }),
        // ── TOLLGATE (BLACK) ──────────────────────────────────────────
        Card::Minion(MinionCard {
            id: 73,
            color: Color::Black,
            name: "Coinbiter".to_string(),
            description: Some("<strong>Tradeable</strong>".to_string()),
            flavor_text: Some("Tests every coin, and keeps the ones that bend.".to_string()),
            base_cost: 1,
            image_url: "".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 1,
            base_defence: 2,
            attributes: vec![MinionAttribute::Tradeable],
            abilities: vec![],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 74,
            color: Color::Black,
            name: "Gatewatch".to_string(),
            description: Some("<strong>Guard</strong>".to_string()),
            flavor_text: Some("Not paid to win, only to still be standing.".to_string()),
            base_cost: 2,
            image_url: "".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 1,
            base_defence: 4,
            attributes: vec![MinionAttribute::Guard],
            abilities: vec![],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 75,
            color: Color::Black,
            name: "The Toll".to_string(),
            description: Some("<strong>Wager:</strong> if you attack my hero this turn, deal 2 damage to all your minions".to_string()),
            flavor_text: Some("The gate is open. The price is printed on it.".to_string()),
            base_cost: 2,
            image_url: "".to_string(),
            form: IncantationForm::Wager {
                trigger: RiteTrigger::EnemyAttacksHero,
                effects: vec![Effect::Damage {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Auto,
                        side: TargetSide::Enemy,
                        entity_type: EntityType::Minion,
                        filters: vec![],
                    },
                    damage: 2,
                    lifesteal: false,
                }],
            },
            abilities: vec![],
            passives: vec![],
            attributes: vec![],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 21,
            color: Color::Black,
            name: "Blood Rite".to_string(),
            description: Some("<strong>Lifesteal</strong><br>Deal 3 damage".to_string()),
            flavor_text: None,
            base_cost: 2,
            image_url: "".to_string(),
            form: IncantationForm::Cast,
            passives: vec![],
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Damage {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Targeted,
                        side: TargetSide::All,
                        entity_type: EntityType::All,
                        filters: vec![],
                    },
                    damage: 3,
                    lifesteal: true,
                }],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 76,
            color: Color::Black,
            name: "Tithe Collector".to_string(),
            description: Some("<strong>Fanfare:</strong> Deal 1 damage to all enemy minions".to_string()),
            flavor_text: Some("Takes a tenth of everything, starting with the small ones.".to_string()),
            base_cost: 3,
            image_url: "".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 3,
            base_defence: 3,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Damage {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Auto,
                        side: TargetSide::Enemy,
                        entity_type: EntityType::Minion,
                        filters: vec![],
                    },
                    damage: 1,
                    lifesteal: false,
                }],
            }],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 77,
            color: Color::Black,
            name: "Hollowing".to_string(),
            description: Some("<strong>Aura:</strong> enemy minions have -1 attack".to_string()),
            flavor_text: Some("Nothing is taken. It simply weighs less.".to_string()),
            base_cost: 3,
            image_url: "".to_string(),
            form: IncantationForm::Aura { turns: 3 },
            abilities: vec![],
            passives: vec![PassiveEffect::Aura {
                side: TargetSide::Enemy,
                filters: vec![],
                attack: -1,
                defence: 0,
            }],
            attributes: vec![],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 78,
            color: Color::Black,
            name: "The Assessor".to_string(),
            description: Some("Enemy minions have -1 attack".to_string()),
            flavor_text: Some("Writes a number beside your name and never says it.".to_string()),
            base_cost: 4,
            image_url: "".to_string(),
            races: vec![Race::Human],
            passives: vec![PassiveEffect::Aura {
                side: TargetSide::Enemy,
                filters: vec![],
                attack: -1,
                defence: 0,
            }],
            base_attack: 2,
            base_defence: 5,
            attributes: vec![],
            abilities: vec![],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 79,
            color: Color::Black,
            name: "Foreclosure".to_string(),
            description: Some("Take control of the rightmost enemy minion".to_string()),
            flavor_text: Some("Signed in advance, for whatever you put down last.".to_string()),
            base_cost: 4,
            image_url: "".to_string(),
            form: IncantationForm::Omen {
                triggers: [RiteTrigger::EnemyPlaysMinion, RiteTrigger::EnemyBoardReachesThree, RiteTrigger::EnemyAttacksHero],
                effects: vec![Effect::TakeControl {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Rightmost,
                        side: TargetSide::Enemy,
                        entity_type: EntityType::Minion,
                        filters: vec![],
                    },
                }],
            },
            abilities: vec![],
            passives: vec![],
            attributes: vec![],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 80,
            color: Color::Black,
            name: "Pillarman".to_string(),
            description: Some("<strong>Guard</strong>".to_string()),
            flavor_text: Some("Has not moved since the gate was built around him.".to_string()),
            base_cost: 5,
            image_url: "".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 4,
            base_defence: 6,
            attributes: vec![MinionAttribute::Guard],
            abilities: vec![],
            cost_scaling: None,
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 81,
            color: Color::Black,
            name: "The Long Winter".to_string(),
            description: Some("<strong>Aura:</strong> enemy minions have -2 attack".to_string()),
            flavor_text: Some("They are all still there in the spring. None of them fought.".to_string()),
            base_cost: 5,
            image_url: "".to_string(),
            form: IncantationForm::Aura { turns: 3 },
            abilities: vec![],
            passives: vec![PassiveEffect::Aura {
                side: TargetSide::Enemy,
                filters: vec![],
                attack: -2,
                defence: 0,
            }],
            attributes: vec![],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 61,
            color: Color::Black,
            name: "Sylvanas".to_string(),
            description: Some("<strong>Deathwish:</strong> Take control of the rightmost enemy minion".to_string()),
            flavor_text: Some("Takes one of yours on the way out.".to_string()),
            base_cost: 6,
            image_url: "".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 5,
            base_defence: 5,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnDeath,
                requirements: vec![],
                effects: vec![Effect::TakeControl {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Rightmost,
                        side: TargetSide::Enemy,
                        entity_type: EntityType::Minion,
                        filters: vec![],
                    },
                }],
            }],
            cost_scaling: None,
            is_token: false,
        }),
        // ── TOKENS ────────────────────────────────────────────────────
        Card::Minion(MinionCard {
            id: BRAMBLE_ID,
            color: Color::Green,
            name: "Bramble".to_string(),
            description: Some("<strong>Guard</strong>".to_string()),
            flavor_text: None,
            base_cost: 1,
            image_url: "".to_string(),
            races: vec![],
            passives: vec![],
            base_attack: 0,
            base_defence: 3,
            attributes: vec![MinionAttribute::Guard],
            abilities: vec![],
            cost_scaling: None,
            is_token: true,
        }),
        Card::Minion(MinionCard {
            id: RAT_ID,
            color: Color::Black,
            name: "Rat".to_string(),
            description: None,
            flavor_text: None,
            base_cost: 1,
            image_url: "Rat.png".to_string(),
            races: vec![Race::Beast],
            passives: vec![],
            base_attack: 1,
            base_defence: 1,
            attributes: vec![],
            abilities: vec![],
            cost_scaling: None,
            is_token: true,
        }),
        Card::Minion(MinionCard {
            id: BIG_RAT_ID,
            color: Color::Black,
            name: "Big Rat".to_string(),
            description: Some("<strong>Blitz</strong>".to_string()),
            flavor_text: None,
            base_cost: 2,
            image_url: "Rat.png".to_string(),
            races: vec![Race::Beast],
            passives: vec![],
            base_attack: 2,
            base_defence: 2,
            attributes: vec![MinionAttribute::Charge],
            abilities: vec![],
            cost_scaling: None,
            is_token: true,
        }),
        // ── HEROES ────────────────────────────────────────────────────
        Card::Hero(HeroCard {
            id: 105,
            name: "Clown Snake".to_string(),
            description: None,
            flavor_text: Some("The big top burned down. The act goes on.".to_string()),
            color: Color::Black,
            image_url: "Clown Snake.png".to_string(),
            base_hp: 30,
        }),
        Card::Hero(HeroCard {
            id: 106,
            name: "The Empress".to_string(),
            description: None,
            flavor_text: Some("Everything in the garden answers to her, briefly.".to_string()),
            color: Color::Green,
            image_url: "".to_string(),
            base_hp: 30,
        }),
        Card::Hero(HeroCard {
            id: 104,
            name: "The Tower".to_string(),
            description: None,
            flavor_text: Some("Everything you built, and the ground it stood on.".to_string()),
            color: Color::Black,
            image_url: "".to_string(),
            base_hp: 30,
        }),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_hero_resolves() {
        assert_eq!(hero_or_default(0).id, DEFAULT_HERO_ID);
        assert_eq!(get_heroes().len(), 3);
    }

    #[test]
    fn heroes_are_not_deck_cards() {
        assert!(get_collectible_cards().iter().all(|c| !matches!(c, Card::Hero(_))));
        assert!(get_heroes().iter().all(|h| h.id >= DEFAULT_HERO_ID));
    }

    /// The catalogue is a map keyed by id, so a duplicate does not clash, it silently wins.
    #[test]
    fn every_card_id_is_unique() {
        let built = build_cards();
        let mut ids: Vec<u32> = built.iter().map(|c| c.id()).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), built.len());
    }

    #[test]
    fn summoned_tokens_exist() {
        assert!(instantiate_minion_by_id(RAT_ID, 1).is_some());
        assert!(instantiate_minion_by_id(BIG_RAT_ID, 3).is_some());
        assert!(instantiate_minion_by_id(BRAMBLE_ID, 4).is_some());
    }
}
