// cards.rs

use crate::engine::IdGenerator;
use crate::types::*;
use std::collections::HashMap;
use std::sync::LazyLock;

static CARDS: LazyLock<HashMap<u32, Card>> = LazyLock::new(|| build_cards().into_iter().map(|c| (card_id(&c), c)).collect());

fn card_id(c: &Card) -> u32 {
    match c {
        Card::Minion(m) => m.id,
        Card::Incantation(i) => i.id,
    }
}

pub fn get_card_by_id(id: u32) -> Option<&'static Card> {
    CARDS.get(&id)
}

pub fn get_collectible_cards() -> Vec<Card> {
    get_all_cards()
        .into_iter()
        .filter(|c| match c {
            Card::Minion(m) => !m.is_token,
            Card::Incantation(i) => !i.is_token,
        })
        .collect()
}

pub fn get_all_cards() -> Vec<Card> {
    CARDS.values().cloned().collect()
}

pub fn instantiate_minion_by_id(card_id: u32, entity_id: u32) -> Option<MinionEntity> {
    match get_card_by_id(card_id)? {
        Card::Minion(c) => Some(MinionEntity {
            attack: c.base_attack,
            defence: c.base_defence,
            max_defence: c.base_defence,
            ward_active: c.attributes.contains(&MinionAttribute::Ward),
            stealth_active: c.attributes.contains(&MinionAttribute::Stealth),
            exhausted: false,
            entity_id,
            cost: c.base_cost,
            turns_in_hand: 0,
            turns_on_board: 0,
            just_drawn: false,
            card: c.clone(),
        }),
        Card::Incantation(_) => None, // Can't summon an incantation
    }
}

fn instantiate_minion(c: &MinionCard, entity_id: u32) -> MinionEntity {
    MinionEntity {
        attack: c.base_attack,
        defence: c.base_defence,
        max_defence: c.base_defence,
        ward_active: c.attributes.contains(&MinionAttribute::Ward),
        stealth_active: c.attributes.contains(&MinionAttribute::Stealth),
        exhausted: false,
        entity_id,
        cost: c.base_cost,
        turns_in_hand: 0,
        turns_on_board: 0,
        just_drawn: false,
        card: c.clone(),
    }
}

fn instantiate_incantation(c: &IncantationCard, entity_id: u32) -> IncantationEntity {
    IncantationEntity {
        entity_id,
        cost: c.base_cost,
        turns_in_hand: 0,
        just_drawn: false,
        card: c.clone(),
    }
}

pub fn deck_to_cards(deck: &[u32], ids: &mut IdGenerator) -> Vec<CardEntity> {
    deck.iter()
        .filter_map(|id| get_card_by_id(*id))
        .map(|card| match card {
            Card::Minion(c) => CardEntity::Minion(instantiate_minion(c, ids.next_id())),
            Card::Incantation(c) => CardEntity::Incantation(instantiate_incantation(c, ids.next_id())),
        })
        .collect()
}

pub fn build_cards() -> Vec<Card> {
    vec![
        // ── WHITE MINIONS ────────────────────────────────────────
        Card::Minion(MinionCard {
            id: 1,
            color: Color::White,
            name: "Machine Elf".to_string(),
            description: Some("<strong>Tradeable</strong> <strong>Lifesteal</strong><br><strong>Engage</strong>".to_string()),
            flavor_text: None,
            base_cost: 1,
            image_url: "Machine-Elf.webp".to_string(),
            races: vec![Race::Elf],
            base_attack: 1,
            base_defence: 2,
            attributes: vec![MinionAttribute::Charge, MinionAttribute::Tradeable, MinionAttribute::Lifesteal],
            abilities: vec![],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 2,
            color: Color::White,
            name: "Dawn Guard".to_string(),
            description: Some("<strong>Guard</strong> <strong>Ward</strong><br><strong>Fanfare:</strong> Draw a card".to_string()),
            flavor_text: None,
            base_cost: 1,
            image_url: "Dawn Guard.webp".to_string(),
            races: vec![Race::Human],
            base_attack: 0,
            base_defence: 8,
            attributes: vec![MinionAttribute::Guard, MinionAttribute::Ward],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Draw {
                    draw_amount: 1,
                    follow_up: None,
                }],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 3,
            color: Color::White,
            name: "Silverguard Knight".to_string(),
            description: Some("<strong>Deathwish:</strong> Draw a card".to_string()),
            flavor_text: None,
            base_cost: 3,
            image_url: "".to_string(),
            races: vec![Race::Human],
            base_attack: 5,
            base_defence: 3,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnDeath,
                requirements: vec![],
                effects: vec![Effect::Draw {
                    draw_amount: 1,
                    follow_up: None,
                }],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 4,
            color: Color::White,
            name: "Radiant Sentinel".to_string(),
            description: Some("<strong>Fanfare:</strong> Deal 2 damage to all minions".to_string()),
            flavor_text: None,
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
        Card::Minion(MinionCard {
            id: 5,
            color: Color::White,
            name: "Herald of the Sun".to_string(),
            description: Some("<strong>Deathwish:</strong> Deal 5 damage to the enemy hero, draw a card".to_string()),
            flavor_text: None,
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
                        target_spec: TargetSpec {
                            target_mode: TargetMode::Auto,
                            side: TargetSide::Enemy,
                            entity_type: EntityType::Hero,
                            filters: vec![],
                        },
                        damage: 5,
                        lifesteal: false,
                    },
                    Effect::Draw {
                        draw_amount: 1,
                        follow_up: None,
                    },
                ],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 6,
            color: Color::White,
            name: "Black Cat".to_string(),
            description: Some("<strong>Tradeable</strong><br><strong>Combo:</strong> Return a friendly minion from the battlefield to your hand".to_string()),
            flavor_text: None,
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
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Targeted,
                        side: TargetSide::Friendly,
                        entity_type: EntityType::Minion,
                        filters: vec![],
                    },
                    cost_reduction: None,
                }],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 7,
            color: Color::White,
            name: "Barry the Hexblade".to_string(),
            description: Some("<strong>Fanfare:</strong> Give all minions +1 +1".to_string()),
            flavor_text: None,
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
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Auto,
                        side: TargetSide::All,
                        entity_type: EntityType::Minion,
                        filters: vec![],
                    },
                    attack: 1,
                    defence: 1,
                }],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 8,
            color: Color::White,
            name: "Void Stalker".to_string(),
            description: Some("<strong>Fanfare:</strong> Deal 10 damage to your own hero".to_string()),
            flavor_text: None,
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
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Auto,
                        side: TargetSide::Friendly,
                        entity_type: EntityType::Hero,
                        filters: vec![],
                    },
                    damage: 10,
                    lifesteal: false,
                }],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 9,
            color: Color::White,
            name: "Grave Warden".to_string(),
            description: Some("<strong>Deathwish:</strong> Return all minions to their owners hand".to_string()),
            flavor_text: None,
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
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Auto,
                        side: TargetSide::All,
                        entity_type: EntityType::Minion,
                        filters: vec![],
                    },
                    cost_reduction: None,
                }],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 10,
            color: Color::White,
            name: "Admirable Minion".to_string(),
            description: Some("<strong>Fanfare:</strong> Give a friendly minion +3 +3".to_string()),
            flavor_text: None,
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
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Targeted,
                        side: TargetSide::Friendly,
                        entity_type: EntityType::Minion,
                        filters: vec![],
                    },
                    attack: 3,
                    defence: 3,
                }],
            }],
            is_token: false,
        }),
        // ── WHITE INCANTATIONS ───────────────────────────────────
        Card::Incantation(IncantationCard {
            id: 11,
            color: Color::White,
            name: "Divine Power".to_string(),
            description: Some("Give a friendly minion +4 +4".to_string()),
            flavor_text: None,
            base_cost: 4,
            image_url: "Divine Power.webp".to_string(),
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
                    attack: 4,
                    defence: 4,
                }],
            }],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 12,
            color: Color::White,
            name: "Good Friday".to_string(),
            description: Some("Deal 4 damage".to_string()),
            flavor_text: None,
            base_cost: 3,
            image_url: "Good Friday.webp".to_string(),
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
                    damage: 4,
                    lifesteal: false,
                }],
            }],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 13,
            color: Color::White,
            name: "Pot of Greed".to_string(),
            description: Some("Draw 1 card <strong>Combo:</strong> Draw 2 cards instead".to_string()),
            flavor_text: None,
            base_cost: 3,
            image_url: "Pot of Greed.webp".to_string(),
            attributes: vec![],
            abilities: vec![
                Ability {
                    trigger: Trigger::OnPlay,
                    requirements: vec![],
                    effects: vec![Effect::Draw {
                        draw_amount: 1,
                        follow_up: None,
                    }],
                },
                Ability {
                    trigger: Trigger::OnPlay,
                    requirements: vec![Requirement::Combo],
                    effects: vec![Effect::Draw {
                        draw_amount: 1,
                        follow_up: None,
                    }],
                },
            ],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 14,
            color: Color::White,
            name: "Blessing".to_string(),
            description: Some("Give a friendly minion +3 +2".to_string()),
            flavor_text: None,
            base_cost: 2,
            image_url: "".to_string(),
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
                    attack: 3,
                    defence: 2,
                }],
            }],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 15,
            color: Color::White,
            name: "Smite".to_string(),
            description: Some("Deal 2 damage <strong>Combo:</strong> Deal 4 damage instead.".to_string()),
            flavor_text: None,
            base_cost: 2,
            image_url: "Smite.webp".to_string(),
            attributes: vec![],
            abilities: vec![
                Ability {
                    trigger: Trigger::OnPlay,
                    requirements: vec![],
                    effects: vec![Effect::Damage {
                        target_spec: TargetSpec {
                            target_mode: TargetMode::Targeted,
                            side: TargetSide::All,
                            entity_type: EntityType::All,
                            filters: vec![],
                        },
                        damage: 2,
                        lifesteal: false,
                    }],
                },
                Ability {
                    trigger: Trigger::OnPlay,
                    requirements: vec![Requirement::Combo],
                    effects: vec![Effect::Damage {
                        target_spec: TargetSpec {
                            target_mode: TargetMode::Targeted,
                            side: TargetSide::All,
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
        Card::Incantation(IncantationCard {
            id: 16,
            color: Color::White,
            name: "Pull".to_string(),
            description: Some("Return a friendly minion to your hand, it costs (2) less.".to_string()),
            flavor_text: None,
            base_cost: 0,
            image_url: "Pull.webp".to_string(),
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::ReturnToHand {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Targeted,
                        side: TargetSide::Friendly,
                        entity_type: EntityType::Minion,
                        filters: vec![],
                    },
                    cost_reduction: Some(2),
                }],
            }],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 17,
            color: Color::White,
            name: "Destruction".to_string(),
            description: Some("<strong>Tradeable</strong><br>Destroy a minion".to_string()),
            flavor_text: None,
            base_cost: 5,
            image_url: "".to_string(),
            attributes: vec![IncantationAttribute::Tradeable],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Destroy {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Targeted,
                        side: TargetSide::All,
                        entity_type: EntityType::Minion,
                        filters: vec![],
                    },
                }],
            }],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 18,
            color: Color::White,
            name: "Gunslinger".to_string(),
            description: Some("<strong>Lifesteal</strong><br>Deal 1 damage <strong>Quickdraw:</strong> Deal 3 damage instead.".to_string()),
            flavor_text: None,
            base_cost: 1,
            image_url: "".to_string(),
            attributes: vec![],
            abilities: vec![
                Ability {
                    trigger: Trigger::OnPlay,
                    requirements: vec![],
                    effects: vec![Effect::Damage {
                        target_spec: TargetSpec {
                            target_mode: TargetMode::Targeted,
                            side: TargetSide::All,
                            entity_type: EntityType::All,
                            filters: vec![],
                        },
                        damage: 1,
                        lifesteal: true,
                    }],
                },
                Ability {
                    trigger: Trigger::OnPlay,
                    requirements: vec![Requirement::Quickdraw],
                    effects: vec![Effect::Damage {
                        target_spec: TargetSpec {
                            target_mode: TargetMode::Targeted,
                            side: TargetSide::All,
                            entity_type: EntityType::All,
                            filters: vec![],
                        },
                        damage: 2,
                        lifesteal: true,
                    }],
                },
            ],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 19,
            color: Color::White,
            name: "Drelf".to_string(),
            description: Some("Draw a card, if it's an elf, it costs (1) less for each minion on your board".to_string()),
            flavor_text: None,
            base_cost: 1,
            image_url: "".to_string(),
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Draw {
                    draw_amount: 1,
                    follow_up: Some(FollowUpAbility {
                        card_must_match: vec![Condition::IsRace { race: Race::Elf }],
                        follow_up_effects: vec![FollowUpEffect::Discount {
                            scaled_amount: ScaledAmount {
                                scalar: 1,
                                scaled_by: Some(ScaledBy::MinionsOnBoard),
                            },
                        }],
                    }),
                }],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 20,
            color: Color::White,
            name: "Expensive Elf".to_string(),
            description: Some("<strong>Engage</strong><br><strong>Fanfare:</strong> Deal 5 damage".to_string()),
            flavor_text: None,
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
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Targeted,
                        side: TargetSide::Enemy,
                        entity_type: EntityType::All,
                        filters: vec![],
                    },
                    damage: 5,
                    lifesteal: false,
                }],
            }],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 21,
            color: Color::White,
            name: "Copy Cat".to_string(),
            description: Some("Draw a card, add a copy to your hand".to_string()),
            flavor_text: None,
            base_cost: 1,
            image_url: "Black Cat.webp".to_string(),
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Draw {
                    draw_amount: 1,
                    follow_up: Some(FollowUpAbility {
                        card_must_match: vec![],
                        follow_up_effects: vec![FollowUpEffect::Copy { copy_amount: 1 }],
                    }),
                }],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 22,
            color: Color::White,
            name: "Rat Cage".to_string(),
            description: Some("<strong>Guard</strong><br><strong>Deathwish:</strong> <strong>Summon</strong> three 1/1 rats ".to_string()),
            flavor_text: Some("Despite all their rage they are still just rats in a cage.".to_string()),
            base_cost: 3,
            image_url: "".to_string(),
            races: vec![],
            base_attack: 0,
            base_defence: 1,
            attributes: vec![MinionAttribute::Guard],
            abilities: vec![Ability {
                trigger: Trigger::OnDeath,
                requirements: vec![],
                effects: vec![Effect::Summon {
                    summon_amount: 3,
                    minion_card_id: 23, // the 1/1 rat card
                }],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 23,
            color: Color::White,
            name: "Rat".to_string(),
            description: None,
            flavor_text: None,
            base_cost: 1,
            image_url: "".to_string(),
            abilities: vec![],
            attributes: vec![],
            races: vec![Race::Beast],
            base_attack: 1,
            base_defence: 1,
            is_token: true,
        }),
        Card::Minion(MinionCard {
            id: 24,
            color: Color::White,
            name: "Hound Master".to_string(),
            description: Some("<strong>Fanfare:</strong> Give a friendly beast +2 +2".to_string()),
            flavor_text: None,
            base_cost: 2,
            image_url: "Hound Master.webp".to_string(),
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Buff {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Targeted,
                        side: TargetSide::Friendly,
                        entity_type: EntityType::Minion,
                        filters: vec![Condition::IsRace { race: Race::Beast }],
                    },
                    attack: 2,
                    defence: 2,
                }],
            }],
            attributes: vec![],
            races: vec![Race::Human],
            base_attack: 2,
            base_defence: 2,
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 25,
            color: Color::White,
            name: "Zoo".to_string(),
            description: Some("Give all friendly beasts +1 +1".to_string()),
            flavor_text: None,
            base_cost: 1,
            image_url: "Zoo.webp".to_string(),
            attributes: vec![IncantationAttribute::Tradeable],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Buff {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Auto,
                        side: TargetSide::Friendly,
                        entity_type: EntityType::Minion,
                        filters: vec![Condition::IsRace { race: Race::Beast }],
                    },
                    attack: 1,
                    defence: 1,
                }],
            }],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 26,
            color: Color::White,
            name: "Flash Heal".to_string(),
            description: Some("Restore 5 health to your hero".to_string()),
            flavor_text: None,
            base_cost: 1,
            image_url: "Flash Heal.webp".to_string(),
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Heal {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Auto,
                        side: TargetSide::Friendly,
                        entity_type: EntityType::Hero,
                        filters: vec![],
                    },
                    heal: 5,
                }],
            }],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 27,
            color: Color::White,
            name: "Mending Aura".to_string(),
            description: Some("Restore 2 health to all your minions".to_string()),
            flavor_text: None,
            base_cost: 1,
            image_url: "Mending Aura.webp".to_string(),
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Heal {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Auto,
                        side: TargetSide::Friendly,
                        entity_type: EntityType::Minion,
                        filters: vec![],
                    },
                    heal: 2,
                }],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 28,
            color: Color::White,
            name: "Plague Rat".to_string(),
            description: Some("<strong>Engage</strong> <strong>Poisonous</strong>".to_string()),
            flavor_text: None,
            base_cost: 1,
            image_url: "Rat.jpg".to_string(),
            races: vec![Race::Beast],
            base_attack: 1,
            base_defence: 1,
            attributes: vec![MinionAttribute::Charge, MinionAttribute::Poisonous],
            abilities: vec![],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 29,
            color: Color::White,
            name: "Shieldmaiden".to_string(),
            description: Some("<strong>Engage</strong> <strong>Ward</strong>".to_string()),
            flavor_text: None,
            base_cost: 1,
            image_url: "Shieldmaiden.jpeg".to_string(),
            races: vec![Race::Human],
            base_attack: 1,
            base_defence: 1,
            attributes: vec![MinionAttribute::Charge, MinionAttribute::Ward],
            abilities: vec![],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 30,
            color: Color::White,
            name: "Assassin".to_string(),
            description: Some("<strong>Engage</strong> <strong>Stealth</strong><br><strong>Poisonous</strong>".to_string()),
            flavor_text: None,
            base_cost: 1,
            image_url: "Assassin.webp".to_string(),
            races: vec![Race::Human],
            base_attack: 1,
            base_defence: 1,
            attributes: vec![MinionAttribute::Charge, MinionAttribute::Stealth, MinionAttribute::Poisonous],
            abilities: vec![],
            is_token: false,
        }),
    ]
}
