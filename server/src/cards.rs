// cards.rs

use crate::types::*;

pub fn get_cards() -> Vec<Card> {
    vec![
        // ── WHITE MINIONS ────────────────────────────────────────
        Card::Minion(MinionCard {
            id: 1,
            color: Color::White,
            name: "Machine Elf".to_string(),
            description: Some("<strong>Tradeable</strong><br><strong>Blitz</strong>".to_string()),
            base_cost: 1,
            image_url: "Machine-Elf.webp".to_string(),
            races: vec![Race::Elf],
            base_attack: 1,
            base_defence: 2,
            attributes: vec![MinionAttribute::Charge, MinionAttribute::Tradeable],
            abilities: vec![],
        }),
        Card::Minion(MinionCard {
            id: 2,
            color: Color::White,
            name: "Overzealous Priest".to_string(),
            description: Some("<strong>Fanfare:</strong> Draw a card".to_string()),
            base_cost: 3,
            image_url: "".to_string(),
            races: vec![Race::Human],
            base_attack: 2,
            base_defence: 1,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Draw { draw_amount: 1, follow_up: None }],
            }],
        }),
        Card::Minion(MinionCard {
            id: 3,
            color: Color::White,
            name: "Silverguard Knight".to_string(),
            description: Some("<strong>Deathwish:</strong> Draw a card".to_string()),
            base_cost: 3,
            image_url: "".to_string(),
            races: vec![Race::Human],
            base_attack: 5,
            base_defence: 3,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnDeath,
                requirements: vec![],
                effects: vec![Effect::Draw { draw_amount: 1, follow_up: None }],
            }],
        }),
        Card::Minion(MinionCard {
            id: 4,
            color: Color::White,
            name: "Radiant Sentinel".to_string(),
            description: Some("<strong>Fanfare:</strong> Deal 2 damage to all minions".to_string()),
            base_cost: 4,
            image_url: "".to_string(),
            races: vec![Race::Elf],
            base_attack: 4,
            base_defence: 6,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Damage {
                    target_spec: TargetSpec { target_mode: TargetMode::Auto, side: TargetSide::All, entity_type: EntityType::Minion },
                    damage: 2,
                }],
            }],
        }),
        Card::Minion(MinionCard {
            id: 5,
            color: Color::White,
            name: "Herald of the Sun".to_string(),
            description: Some("<strong>Deathwish:</strong> Deal 5 damage to the enemy hero, draw a card".to_string()),
            base_cost: 5,
            image_url: "".to_string(),
            races: vec![Race::Elf],
            base_attack: 6,
            base_defence: 5,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnDeath,
                requirements: vec![],
                effects: vec![
                    Effect::Damage {
                        target_spec: TargetSpec { target_mode: TargetMode::Auto, side: TargetSide::Enemy, entity_type: EntityType::Hero },
                        damage: 5,
                    },
                    Effect::Draw { draw_amount: 1, follow_up: None },
                ],
            }],
        }),
        Card::Minion(MinionCard {
            id: 6,
            color: Color::White,
            name: "Black Cat".to_string(),
            description: Some("<strong>Tradeable</strong><br><strong>Combo:</strong> Return a friendly minion from the battlefield to your hand".to_string()),
            base_cost: 1,
            image_url: "Black Cat.webp".to_string(),
            races: vec![Race::Beast],
            base_attack: 2,
            base_defence: 1,
            attributes: vec![MinionAttribute::Tradeable],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![Requirement::Combo],
                effects: vec![Effect::ReturnToHand {
                    target_spec: TargetSpec { target_mode: TargetMode::Targeted, side: TargetSide::Friendly, entity_type: EntityType::Minion },
                    cost_reduction: None,
                }],
            }],
        }),
        Card::Minion(MinionCard {
            id: 7,
            color: Color::White,
            name: "Barry the Hexblade".to_string(),
            description: Some("<strong>Fanfare:</strong> Give all minions +1 +1".to_string()),
            base_cost: 2,
            image_url: "".to_string(),
            races: vec![Race::Human],
            base_attack: 4,
            base_defence: 2,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Buff {
                    target_spec: TargetSpec { target_mode: TargetMode::Auto, side: TargetSide::All, entity_type: EntityType::Minion },
                    attack: 1,
                    defence: 1,
                }],
            }],
        }),
        Card::Minion(MinionCard {
            id: 8,
            color: Color::White,
            name: "Void Stalker".to_string(),
            description: Some("<strong>Fanfare:</strong> Deal 10 damage to your own hero".to_string()),
            base_cost: 2,
            image_url: "".to_string(),
            races: vec![Race::Elf],
            base_attack: 5,
            base_defence: 5,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Damage {
                    target_spec: TargetSpec { target_mode: TargetMode::Auto, side: TargetSide::Friendly, entity_type: EntityType::Hero },
                    damage: 10,
                }],
            }],
        }),
        Card::Minion(MinionCard {
            id: 9,
            color: Color::White,
            name: "Grave Warden".to_string(),
            description: Some("<strong>Deathwish:</strong> Return all minions to their owners hand".to_string()),
            base_cost: 4,
            image_url: "".to_string(),
            races: vec![Race::Human],
            base_attack: 5,
            base_defence: 4,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnDeath,
                requirements: vec![],
                effects: vec![Effect::ReturnToHand {
                    target_spec: TargetSpec { target_mode: TargetMode::Auto, side: TargetSide::All, entity_type: EntityType::Minion },
                    cost_reduction: None,
                }],
            }],
        }),
        Card::Minion(MinionCard {
            id: 10,
            color: Color::White,
            name: "Admirable Minion".to_string(),
            description: Some("<strong>Fanfare:</strong> Give a friendly minion +3 +3".to_string()),
            base_cost: 6,
            image_url: "".to_string(),
            races: vec![Race::Elf],
            base_attack: 5,
            base_defence: 4,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Buff {
                    target_spec: TargetSpec { target_mode: TargetMode::Targeted, side: TargetSide::Friendly, entity_type: EntityType::Minion },
                    attack: 3,
                    defence: 3,
                }],
            }],
        }),

        // ── WHITE INCANTATIONS ───────────────────────────────────
        Card::Incantation(IncantationCard {
            id: 11,
            color: Color::White,
            name: "Divine Power".to_string(),
            description: Some("Give a friendly minion +4 +4".to_string()),
            base_cost: 4,
            image_url: "Divine Power.webp".to_string(),
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Buff {
                    target_spec: TargetSpec { target_mode: TargetMode::Targeted, side: TargetSide::Friendly, entity_type: EntityType::Minion },
                    attack: 4,
                    defence: 4,
                }],
            }],
        }),
        Card::Incantation(IncantationCard {
            id: 12,
            color: Color::White,
            name: "Good Friday".to_string(),
            description: Some("Deal 4 damage".to_string()),
            base_cost: 3,
            image_url: "Good Friday.webp".to_string(),
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Damage {
                    target_spec: TargetSpec { target_mode: TargetMode::Targeted, side: TargetSide::All, entity_type: EntityType::All },
                    damage: 4,
                }],
            }],
        }),
        Card::Incantation(IncantationCard {
            id: 13,
            color: Color::White,
            name: "Pot of Greed".to_string(),
            description: Some("Draw 1 card <strong>Combo:</strong> Draw 2 cards instead".to_string()),
            base_cost: 3,
            image_url: "Pot of Greed.webp".to_string(),
            attributes: vec![],
            abilities: vec![
                Ability {
                    trigger: Trigger::OnPlay,
                    requirements: vec![],
                    effects: vec![Effect::Draw { draw_amount: 1, follow_up: None }],
                },
                Ability {
                    trigger: Trigger::OnPlay,
                    requirements: vec![Requirement::Combo],
                    effects: vec![Effect::Draw { draw_amount: 1, follow_up: None }],
                },
            ],
        }),
        Card::Incantation(IncantationCard {
            id: 14,
            color: Color::White,
            name: "Blessing".to_string(),
            description: Some("Give a friendly minion +3 +2".to_string()),
            base_cost: 2,
            image_url: "".to_string(),
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Buff {
                    target_spec: TargetSpec { target_mode: TargetMode::Targeted, side: TargetSide::Friendly, entity_type: EntityType::Minion },
                    attack: 3,
                    defence: 2,
                }],
            }],
        }),
        Card::Incantation(IncantationCard {
            id: 15,
            color: Color::White,
            name: "Smite".to_string(),
            description: Some("Deal 2 damage <strong>Combo:</strong> Deal 4 damage instead.".to_string()),
            base_cost: 2,
            image_url: "Smite.webp".to_string(),
            attributes: vec![],
            abilities: vec![
                Ability {
                    trigger: Trigger::OnPlay,
                    requirements: vec![],
                    effects: vec![Effect::Damage {
                        target_spec: TargetSpec { target_mode: TargetMode::Targeted, side: TargetSide::All, entity_type: EntityType::All },
                        damage: 2,
                    }],
                },
                Ability {
                    trigger: Trigger::OnPlay,
                    requirements: vec![Requirement::Combo],
                    effects: vec![Effect::Damage {
                        target_spec: TargetSpec { target_mode: TargetMode::Targeted, side: TargetSide::All, entity_type: EntityType::All },
                        damage: 2,
                    }],
                },
            ],
        }),
        Card::Incantation(IncantationCard {
            id: 16,
            color: Color::White,
            name: "Pull".to_string(),
            description: Some("Return a friendly minion to your hand, it costs (2) less.".to_string()),
            base_cost: 0,
            image_url: "Pull.webp".to_string(),
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::ReturnToHand {
                    target_spec: TargetSpec { target_mode: TargetMode::Targeted, side: TargetSide::Friendly, entity_type: EntityType::Minion },
                    cost_reduction: Some(2),
                }],
            }],
        }),
        Card::Incantation(IncantationCard {
            id: 17,
            color: Color::White,
            name: "Destruction".to_string(),
            description: Some("<strong>Tradeable</strong><br>Destroy a minion".to_string()),
            base_cost: 5,
            image_url: "".to_string(),
            attributes: vec![IncantationAttribute::Tradeable],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Destroy {
                    target_spec: TargetSpec { target_mode: TargetMode::Targeted, side: TargetSide::All, entity_type: EntityType::Minion },
                }],
            }],
        }),
        Card::Incantation(IncantationCard {
            id: 18,
            color: Color::White,
            name: "Gunslinger".to_string(),
            description: Some("Deal 1 damage <strong>Quickdraw:</strong> Deal 3 damage instead.".to_string()),
            base_cost: 1,
            image_url: "".to_string(),
            attributes: vec![],
            abilities: vec![
                Ability {
                    trigger: Trigger::OnPlay,
                    requirements: vec![],
                    effects: vec![Effect::Damage {
                        target_spec: TargetSpec { target_mode: TargetMode::Targeted, side: TargetSide::All, entity_type: EntityType::All },
                        damage: 1,
                    }],
                },
                Ability {
                    trigger: Trigger::OnPlay,
                    requirements: vec![Requirement::Quickdraw],
                    effects: vec![Effect::Damage {
                        target_spec: TargetSpec { target_mode: TargetMode::Targeted, side: TargetSide::All, entity_type: EntityType::All },
                        damage: 2,
                    }],
                },
            ],
        }),
        Card::Incantation(IncantationCard {
            id: 19,
            color: Color::White,
            name: "Drelf".to_string(),
            description: Some("Draw a card, if it's an elf, it costs (1) less for each minion on your board".to_string()),
            base_cost: 1,
            image_url: "".to_string(),
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Draw {
                    draw_amount: 1,
                    follow_up: Some(FollowUpAbility {
                        follow_up_requirements: vec![FollowUpRequirement::IsRace { race: Race::Elf }],
                        follow_up_effects: vec![FollowUpEffect::Discount {
                            scaled_amount: ScaledAmount { scalar: 1, scaled_by: Some(ScaledBy::MinionsOnBoard) },
                        }],
                    }),
                }],
            }],
        }),
        Card::Minion(MinionCard {
            id: 20,
            color: Color::White,
            name: "Expensive Elf".to_string(),
            description: Some("<strong>Blitz.</strong><br>Deal 5 damage".to_string()),
            base_cost: 5,
            image_url: "".to_string(),
            races: vec![Race::Elf],
            base_attack: 5,
            base_defence: 5,
            attributes: vec![MinionAttribute::Charge, MinionAttribute::Tradeable],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Damage {
                    target_spec: TargetSpec { target_mode: TargetMode::Targeted, side: TargetSide::Enemy, entity_type: EntityType::All },
                    damage: 5,
                }],
            }],
        }),
        Card::Incantation(IncantationCard {
            id: 21,
            color: Color::White,
            name: "Copy Cat".to_string(),
            description: Some("Draw a card, add a copy to your hand".to_string()),
            base_cost: 1,
            image_url: "Black Cat.webp".to_string(),
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Draw {
                    draw_amount: 1,
                    follow_up: Some(FollowUpAbility {
                        follow_up_requirements: vec![],
                        follow_up_effects: vec![FollowUpEffect::Copy { copy_amount: 1 }],
                    }),
                }],
            }],
        }),
    ]
}

pub fn get_card_by_id(id: u32) -> Option<Card> {
    get_cards().into_iter().find(|c| match c {
        Card::Minion(m) => m.id == id,
        Card::Incantation(i) => i.id == id,
    })
}

pub fn deck_to_cards(deck: &[u32]) -> Vec<CardEntity> {
    deck.iter()
        .filter_map(|id| get_card_by_id(*id))
        .map(|card| match card {
            Card::Minion(c) => CardEntity::Minion(MinionEntity {
                attack: c.base_attack,
                defence: c.base_defence,
                exhausted: false,
                base: EntityBase {
                    entity_id: uuid(),
                    cost: c.base_cost,
                    turns_in_hand: 0,
                    just_drawn: false,
                },
                card: c,
            }),
            Card::Incantation(c) => CardEntity::Incantation(IncantationEntity {
                base: EntityBase {
                    entity_id: uuid(),
                    cost: c.base_cost,
                    turns_in_hand: 0,
                    just_drawn: false,
                },
                card: c,
            }),
        })
        .collect()
}

fn uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    // simple uuid-like id without pulling in the uuid crate
    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("{:x}-{:x}", t, rand::random::<u64>())
}