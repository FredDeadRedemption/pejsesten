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
            Card::Omen(o) => !o.is_token,
            Card::Hero(_) => false,
        })
        .collect()
}

pub fn get_all_cards() -> Vec<Card> {
    CARDS.values().cloned().collect()
}

/// Heroes sit in their own id band of the pool so the card ids stay contiguous.
pub const DEFAULT_HERO_ID: u32 = 101;

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

pub fn instantiate_minion_by_id(card_id: u32, entity_id: u32) -> Option<MinionEntity> {
    match get_card_by_id(card_id)? {
        Card::Minion(c) => Some(MinionEntity {
            stats: Stats::new(c.base_attack, c.base_defence),
            ward_active: c.attributes.contains(&MinionAttribute::Ward),
            stealth_active: c.attributes.contains(&MinionAttribute::Stealth),
            exhausted: false,
            entity_id,
            cost: c.base_cost,
            turns_on_board: 0,
            arrived_on_turn: None,
            card: c.clone(),
        }),
        Card::Incantation(_) | Card::Omen(_) | Card::Hero(_) => None, // only minions can be summoned
    }
}

/// Token ids the summon effects point at.
const RAT_ID: u32 = 23;
const HOUND_ID: u32 = 24;

pub fn build_cards() -> Vec<Card> {
    vec![
        // ── WHITE MINIONS ────────────────────────────────────────
        Card::Minion(MinionCard {
            id: 1,
            color: Color::White,
            name: "Shieldmaiden".to_string(),
            description: Some("<strong>Ward</strong>".to_string()),
            flavor_text: Some("First in the line, and the line holds.".to_string()),
            base_cost: 1,
            image_url: "Shieldmaiden.png".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 1,
            base_defence: 1,
            attributes: vec![MinionAttribute::Ward],
            abilities: vec![],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 2,
            color: Color::White,
            name: "Dawn Guard".to_string(),
            description: Some("<strong>Guard</strong>".to_string()),
            flavor_text: Some("Paid to stand still.".to_string()),
            base_cost: 2,
            image_url: "Dawn Guard.png".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 1,
            base_defence: 4,
            attributes: vec![MinionAttribute::Guard],
            abilities: vec![],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 3,
            color: Color::White,
            name: "Machine Elf".to_string(),
            description: Some("<strong>Tradeable</strong> <strong>Lifesteal</strong>".to_string()),
            flavor_text: None,
            base_cost: 2,
            image_url: "Machine-Elf.png".to_string(),
            races: vec![Race::Elf],
            passives: vec![],
            base_attack: 2,
            base_defence: 2,
            attributes: vec![MinionAttribute::Tradeable, MinionAttribute::Lifesteal],
            abilities: vec![],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 4,
            color: Color::White,
            name: "Standard Bearer".to_string(),
            description: Some("<strong>Fanfare:</strong> Give a friendly minion +1 +1".to_string()),
            flavor_text: None,
            base_cost: 3,
            image_url: "".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 2,
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
                    attack: 1,
                    defence: 1,
                }],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 5,
            color: Color::White,
            name: "Silverguard Knight".to_string(),
            description: Some("<strong>Guard</strong><br><strong>Deathwish:</strong> Draw a card".to_string()),
            flavor_text: None,
            base_cost: 4,
            image_url: "".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 3,
            base_defence: 4,
            attributes: vec![MinionAttribute::Guard],
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
            id: 6,
            color: Color::White,
            name: "Herald of the Sun".to_string(),
            description: Some("<strong>Lifesteal</strong>".to_string()),
            flavor_text: Some("Dawn arrives whether you are ready or not.".to_string()),
            base_cost: 5,
            image_url: "".to_string(),
            races: vec![Race::Elf],
            passives: vec![],
            base_attack: 5,
            base_defence: 5,
            attributes: vec![MinionAttribute::Lifesteal],
            abilities: vec![],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 7,
            color: Color::White,
            name: "Gargoyle Vigil".to_string(),
            description: Some("<strong>Guard</strong>".to_string()),
            flavor_text: Some("It has watched this door for four hundred years.".to_string()),
            base_cost: 6,
            image_url: "gargoyles.png".to_string(),
            races: vec![],
            passives: vec![],
            base_attack: 5,
            base_defence: 7,
            attributes: vec![MinionAttribute::Guard],
            abilities: vec![],
            is_token: false,
        }),
        // ── WHITE INCANTATIONS ───────────────────────────────────
        Card::Incantation(IncantationCard {
            id: 8,
            color: Color::White,
            name: "Smite".to_string(),
            description: Some("Deal 2 damage <strong>Combo:</strong> Deal 4 damage instead".to_string()),
            flavor_text: None,
            base_cost: 1,
            image_url: "Smite.png".to_string(),
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
            id: 9,
            color: Color::White,
            name: "Good Friday".to_string(),
            description: Some("Deal 4 damage to a minion".to_string()),
            flavor_text: None,
            base_cost: 3,
            image_url: "Good Friday.png".to_string(),
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Damage {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Targeted,
                        side: TargetSide::All,
                        entity_type: EntityType::Minion,
                        filters: vec![],
                    },
                    damage: 4,
                    lifesteal: false,
                }],
            }],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 10,
            color: Color::White,
            name: "Divine Power".to_string(),
            description: Some("Give a friendly minion +4 +4".to_string()),
            flavor_text: None,
            base_cost: 4,
            image_url: "Divine Power.png".to_string(),
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
        // ── WHITE OMENS ──────────────────────────────────────────
        Card::Omen(OmenCard {
            id: 11,
            color: Color::White,
            name: "Grove Vigil".to_string(),
            description: Some("Give all your minions +1 +1".to_string()),
            flavor_text: Some("The forest keeps its own watch.".to_string()),
            base_cost: 2,
            image_url: "".to_string(),
            effects: vec![Effect::Buff {
                target_spec: TargetSpec {
                    target_mode: TargetMode::Auto,
                    side: TargetSide::Friendly,
                    entity_type: EntityType::Minion,
                    filters: vec![],
                },
                attack: 1,
                defence: 1,
            }],
            triggers: [OmenTrigger::EnemyPlaysMinion, OmenTrigger::EnemyAttacksMinion, OmenTrigger::EnemyEndsTurn],
            is_token: false,
        }),
        // ── BLACK MINIONS ────────────────────────────────────────
        Card::Minion(MinionCard {
            id: 12,
            color: Color::Black,
            name: "Black Cat".to_string(),
            description: Some("<strong>Tradeable</strong>".to_string()),
            flavor_text: Some("Crosses your path on the way to somewhere worse.".to_string()),
            base_cost: 1,
            image_url: "Black Cat.png".to_string(),
            races: vec![Race::Beast],
            passives: vec![],
            base_attack: 2,
            base_defence: 1,
            attributes: vec![MinionAttribute::Tradeable],
            abilities: vec![],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 13,
            color: Color::Black,
            name: "Dire Hound".to_string(),
            description: Some("<strong>Blitz</strong>".to_string()),
            flavor_text: None,
            base_cost: 2,
            image_url: "".to_string(),
            races: vec![Race::Beast],
            passives: vec![],
            base_attack: 2,
            base_defence: 2,
            attributes: vec![MinionAttribute::Charge],
            abilities: vec![],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 14,
            color: Color::Black,
            name: "Carrion Hound".to_string(),
            description: Some("<strong>Deathwish:</strong> Draw a card".to_string()),
            flavor_text: Some("It eats last, and it eats everything.".to_string()),
            base_cost: 2,
            image_url: "".to_string(),
            races: vec![Race::Beast],
            passives: vec![],
            base_attack: 1,
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
            id: 15,
            color: Color::Black,
            name: "Assassin".to_string(),
            description: Some("<strong>Stealth</strong> <strong>Poisonous</strong>".to_string()),
            flavor_text: None,
            base_cost: 2,
            image_url: "Assassin.png".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 1,
            base_defence: 1,
            attributes: vec![MinionAttribute::Stealth, MinionAttribute::Poisonous],
            abilities: vec![],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 16,
            color: Color::Black,
            name: "Hound Master".to_string(),
            description: Some("<strong>Fanfare:</strong> Give a friendly beast +2 +2".to_string()),
            flavor_text: None,
            base_cost: 3,
            image_url: "Hound Master.png".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 2,
            base_defence: 3,
            attributes: vec![],
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
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 17,
            color: Color::Black,
            name: "Plague Rat".to_string(),
            description: Some("<strong>Blitz</strong> <strong>Poisonous</strong>".to_string()),
            flavor_text: Some("One bite is the whole plan.".to_string()),
            base_cost: 3,
            image_url: "Rat.png".to_string(),
            races: vec![Race::Beast],
            passives: vec![],
            base_attack: 1,
            base_defence: 2,
            attributes: vec![MinionAttribute::Charge, MinionAttribute::Poisonous],
            abilities: vec![],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 18,
            color: Color::Black,
            name: "Rat Cage".to_string(),
            description: Some("<strong>Guard</strong><br><strong>Deathwish:</strong> <strong>Summon</strong> two 1/1 rats".to_string()),
            flavor_text: Some("Despite all their rage they are still just rats in a cage.".to_string()),
            base_cost: 4,
            image_url: "".to_string(),
            races: vec![],
            passives: vec![],
            base_attack: 2,
            base_defence: 4,
            attributes: vec![MinionAttribute::Guard],
            abilities: vec![Ability {
                trigger: Trigger::OnDeath,
                requirements: vec![],
                effects: vec![Effect::Summon {
                    summon_amount: 2,
                    minion_card_id: RAT_ID,
                }],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 19,
            color: Color::Black,
            name: "Dire Matriarch".to_string(),
            description: Some("<strong>Deathwish:</strong> <strong>Summon</strong> two 2/2 hounds".to_string()),
            flavor_text: Some("The pack does not end with her.".to_string()),
            base_cost: 6,
            image_url: "".to_string(),
            races: vec![Race::Beast],
            passives: vec![],
            base_attack: 4,
            base_defence: 5,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnDeath,
                requirements: vec![],
                effects: vec![Effect::Summon {
                    summon_amount: 2,
                    minion_card_id: HOUND_ID,
                }],
            }],
            is_token: false,
        }),
        // ── BLACK INCANTATIONS ───────────────────────────────────
        Card::Incantation(IncantationCard {
            id: 20,
            color: Color::Black,
            name: "Zoo".to_string(),
            description: Some("<strong>Tradeable</strong><br>Give all your beasts +1 +1".to_string()),
            flavor_text: None,
            base_cost: 1,
            image_url: "Zoo.png".to_string(),
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
            id: 21,
            color: Color::Black,
            name: "Blood Rite".to_string(),
            description: Some("<strong>Lifesteal</strong><br>Deal 3 damage".to_string()),
            flavor_text: None,
            base_cost: 2,
            image_url: "".to_string(),
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
        // ── BLACK OMENS ──────────────────────────────────────────
        Card::Omen(OmenCard {
            id: 22,
            color: Color::Black,
            name: "Pack Instinct".to_string(),
            description: Some("<strong>Summon</strong> three 1/1 rats".to_string()),
            flavor_text: Some("Nothing in the dark hunts alone.".to_string()),
            base_cost: 2,
            image_url: "".to_string(),
            effects: vec![Effect::Summon {
                summon_amount: 3,
                minion_card_id: RAT_ID,
            }],
            triggers: [OmenTrigger::FriendlyMinionDies, OmenTrigger::EnemyAttacksHero, OmenTrigger::EnemyPlaysIncantation],
            is_token: false,
        }),
        // ── BLACK QUICKDRAW ──────────────────────────────────────
        Card::Incantation(IncantationCard {
            id: 25,
            color: Color::Black,
            name: "Trick Shot".to_string(),
            description: Some("<strong>Tradeable</strong><br>Deal 2 damage <strong>Quickdraw:</strong> Deal 2 more".to_string()),
            flavor_text: Some("Fired before the drum roll finished.".to_string()),
            base_cost: 1,
            image_url: "".to_string(),
            attributes: vec![IncantationAttribute::Tradeable],
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
                    requirements: vec![Requirement::Quickdraw],
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
            id: 26,
            color: Color::Black,
            name: "Turnabout".to_string(),
            description: Some("Reverse your deck".to_string()),
            flavor_text: Some("The last act goes on first tonight.".to_string()),
            base_cost: 1,
            image_url: "".to_string(),
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::ReverseDeck],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 27,
            color: Color::Black,
            name: "Knife Thrower".to_string(),
            description: Some("<strong>Tradeable</strong><br><strong>Quickdraw:</strong> Deal 2 damage to a minion".to_string()),
            flavor_text: None,
            base_cost: 2,
            image_url: "".to_string(),
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
            id: 28,
            color: Color::Black,
            name: "The Ringmaster".to_string(),
            description: Some("Your <strong>Quickdraw</strong> abilities trigger twice".to_string()),
            flavor_text: Some("He calls the act, and the act obeys.".to_string()),
            base_cost: 5,
            image_url: "".to_string(),
            races: vec![Race::Human],
            passives: vec![PassiveEffect::QuickdrawTriggersTwice],
            base_attack: 3,
            base_defence: 4,
            attributes: vec![],
            abilities: vec![],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 29,
            color: Color::Black,
            name: "Grand Finale".to_string(),
            description: Some("<strong>Fanfare:</strong> If you played seven <strong>Quickdraws</strong> this game, deal 7 damage to a minion and to the enemy hero".to_string()),
            flavor_text: Some("Seven shots, and the seventh is the one they came for.".to_string()),
            base_cost: 9,
            image_url: "".to_string(),
            races: vec![],
            passives: vec![],
            base_attack: 5,
            base_defence: 5,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![Requirement::CounterAtLeast {
                    counter: Counter::QuickdrawsPlayed,
                    amount: 7,
                }],
                effects: vec![
                    Effect::Damage {
                        target_spec: TargetSpec {
                            target_mode: TargetMode::Targeted,
                            side: TargetSide::All,
                            entity_type: EntityType::Minion,
                            filters: vec![],
                        },
                        damage: 7,
                        lifesteal: false,
                    },
                    Effect::Damage {
                        target_spec: TargetSpec {
                            target_mode: TargetMode::Auto,
                            side: TargetSide::Enemy,
                            entity_type: EntityType::Hero,
                            filters: vec![],
                        },
                        damage: 7,
                        lifesteal: false,
                    },
                ],
            }],
            is_token: false,
        }),
        // ── BLACK CIRCUS ─────────────────────────────────────────
        Card::Minion(MinionCard {
            id: 30,
            color: Color::Black,
            name: "Powder Monkey".to_string(),
            description: Some("<strong>Fanfare:</strong> Draw a card. It costs 1 less".to_string()),
            flavor_text: Some("Loads the cannon, rides the cannon.".to_string()),
            base_cost: 1,
            image_url: "".to_string(),
            races: vec![Race::Beast],
            passives: vec![],
            base_attack: 1,
            base_defence: 1,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![],
                effects: vec![Effect::Draw {
                    draw_amount: 1,
                    follow_up: Some(FollowUpAbility {
                        card_must_match: vec![],
                        follow_up_effects: vec![FollowUpEffect::Discount {
                            scaled_amount: ScaledAmount { scalar: 1, scaled_by: None },
                        }],
                    }),
                }],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 31,
            color: Color::Black,
            name: "Tightrope Walker".to_string(),
            description: Some("<strong>Fanfare:</strong> <strong>Combo:</strong> Give it +2 attack".to_string()),
            flavor_text: Some("No net. That is the whole trick.".to_string()),
            base_cost: 2,
            image_url: "".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 1,
            base_defence: 3,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![Requirement::Combo],
                effects: vec![Effect::Buff {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::SelfOnly,
                        side: TargetSide::Friendly,
                        entity_type: EntityType::Minion,
                        filters: vec![],
                    },
                    attack: 2,
                    defence: 0,
                }],
            }],
            is_token: false,
        }),
        Card::Incantation(IncantationCard {
            id: 32,
            color: Color::Black,
            name: "Hat Trick".to_string(),
            description: Some("Draw a card and add a copy of it to your hand".to_string()),
            flavor_text: None,
            base_cost: 2,
            image_url: "".to_string(),
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
        Card::Incantation(IncantationCard {
            id: 33,
            color: Color::Black,
            name: "Encore".to_string(),
            description: Some("Return a friendly minion to your hand. It costs 2 less".to_string()),
            flavor_text: Some("They never leave after one.".to_string()),
            base_cost: 2,
            image_url: "".to_string(),
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
        Card::Omen(OmenCard {
            id: 34,
            color: Color::Black,
            name: "Safety Net".to_string(),
            description: Some("Restore 8 hp to your hero".to_string()),
            flavor_text: Some("Strung up between the acts nobody survives.".to_string()),
            base_cost: 2,
            image_url: "".to_string(),
            effects: vec![Effect::Heal {
                target_spec: TargetSpec {
                    target_mode: TargetMode::Auto,
                    side: TargetSide::Friendly,
                    entity_type: EntityType::Hero,
                    filters: vec![],
                },
                heal: 8,
            }],
            triggers: [OmenTrigger::EnemyAttacksHero, OmenTrigger::EnemyPlaysIncantation, OmenTrigger::EnemyEndsTurn],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 35,
            color: Color::Black,
            name: "Human Cannonball".to_string(),
            description: Some("<strong>Blitz</strong><br><strong>Deathwish:</strong> Deal 2 damage to the enemy hero".to_string()),
            flavor_text: Some("Aimed on Tuesday, lands on Thursday.".to_string()),
            base_cost: 3,
            image_url: "".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 4,
            base_defence: 2,
            attributes: vec![MinionAttribute::Charge],
            abilities: vec![Ability {
                trigger: Trigger::OnDeath,
                requirements: vec![],
                effects: vec![Effect::Damage {
                    target_spec: TargetSpec {
                        target_mode: TargetMode::Auto,
                        side: TargetSide::Enemy,
                        entity_type: EntityType::Hero,
                        filters: vec![],
                    },
                    damage: 2,
                    lifesteal: false,
                }],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 36,
            color: Color::Black,
            name: "Beast Tamer".to_string(),
            description: Some("<strong>Fanfare:</strong> If you are holding a beast, <strong>Summon</strong> two 1/1 rats".to_string()),
            flavor_text: None,
            base_cost: 3,
            image_url: "".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 2,
            base_defence: 3,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![Requirement::IsHolding {
                    filters: vec![Condition::IsRace { race: Race::Beast }],
                }],
                effects: vec![Effect::Summon {
                    summon_amount: 2,
                    minion_card_id: RAT_ID,
                }],
            }],
            is_token: false,
        }),
        Card::Minion(MinionCard {
            id: 37,
            color: Color::Black,
            name: "Fire Eater".to_string(),
            description: Some("<strong>Quickdraw:</strong> Deal 2 damage to all enemy minions".to_string()),
            flavor_text: Some("Holds the mouthful until the front row leans in.".to_string()),
            base_cost: 4,
            image_url: "".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 3,
            base_defence: 3,
            attributes: vec![],
            abilities: vec![Ability {
                trigger: Trigger::OnPlay,
                requirements: vec![Requirement::Quickdraw],
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
            id: 39,
            color: Color::Black,
            name: "The Strongman".to_string(),
            description: Some("<strong>Guard</strong><br><strong>Deathwish:</strong> Draw two cards".to_string()),
            flavor_text: Some("Lifts the wagon so the show can leave town.".to_string()),
            base_cost: 6,
            image_url: "".to_string(),
            races: vec![Race::Human],
            passives: vec![],
            base_attack: 4,
            base_defence: 6,
            attributes: vec![MinionAttribute::Guard],
            abilities: vec![Ability {
                trigger: Trigger::OnDeath,
                requirements: vec![],
                effects: vec![Effect::Draw {
                    draw_amount: 2,
                    follow_up: None,
                }],
            }],
            is_token: false,
        }),
        // ── TOKENS ───────────────────────────────────────────────
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
            is_token: true,
        }),
        Card::Minion(MinionCard {
            id: HOUND_ID,
            color: Color::Black,
            name: "Hound".to_string(),
            description: None,
            flavor_text: None,
            base_cost: 2,
            image_url: "".to_string(),
            races: vec![Race::Beast],
            passives: vec![],
            base_attack: 2,
            base_defence: 2,
            attributes: vec![],
            abilities: vec![],
            is_token: true,
        }),
        // ── HEROES ───────────────────────────────────────────────
        Card::Hero(HeroCard {
            id: 101,
            name: "The Magician".to_string(),
            description: None,
            flavor_text: Some("As above, so below, and both of them yours.".to_string()),
            color: Color::White,
            image_url: "".to_string(),
            base_hp: 30,
        }),
        Card::Hero(HeroCard {
            id: 102,
            name: "The High Priestess".to_string(),
            description: None,
            flavor_text: Some("She has already read the card you are about to draw.".to_string()),
            color: Color::White,
            image_url: "".to_string(),
            base_hp: 30,
        }),
        Card::Hero(HeroCard {
            id: 103,
            name: "The Hermit".to_string(),
            description: None,
            flavor_text: Some("One lamp, one road, no company.".to_string()),
            color: Color::Black,
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
        Card::Hero(HeroCard {
            id: 105,
            name: "Clown Snake".to_string(),
            description: None,
            flavor_text: Some("The big top burned down. The act goes on.".to_string()),
            color: Color::Black,
            image_url: "Clown Snake.png".to_string(),
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
        assert_eq!(get_heroes().len(), 5);
    }

    #[test]
    fn heroes_are_not_deck_cards() {
        assert!(get_collectible_cards().iter().all(|c| !matches!(c, Card::Hero(_))));
        assert!(get_heroes().iter().all(|h| h.id >= DEFAULT_HERO_ID));
    }

    #[test]
    fn summoned_tokens_exist() {
        assert!(instantiate_minion_by_id(RAT_ID, 1).is_some());
        assert!(instantiate_minion_by_id(HOUND_ID, 2).is_some());
    }
}
