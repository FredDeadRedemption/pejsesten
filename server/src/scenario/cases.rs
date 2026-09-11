//! The scenario catalogue.
//!
//! Every case mints its own cards, so nothing here depends on `shared::cards`.
#![allow(dead_code)]

use super::build::*;
use super::{Scenario, Setup, Side};
use crate::settings;
use shared::types::*;

pub struct Case {
    pub name: String,
    pub group: &'static str,
    pub run: Box<dyn Fn() -> Scenario + Send + Sync>,
}

fn case(group: &'static str, name: &str, run: fn() -> Scenario) -> Case {
    Case { name: name.to_string(), group, run: Box::new(run) }
}

pub fn all() -> Vec<Case> {
    let mut cases = vec![
        // effects
        case("effects", "buff raises attack, defence and the heal ceiling", buff_raises_ceiling),
        case("effects", "damage reduces defence", damage_reduces_defence),
        case("effects", "damage with lifesteal heals once per entity hit", lifesteal_scales_with_hits),
        case("effects", "heal on a minion stops at max defence", heal_caps_at_max_defence),
        case("effects", "heal on a hero stops at max hp", heal_caps_at_max_hp),
        case("effects", "destroy ignores ward", destroy_ignores_ward),
        case("effects", "draw moves cards from the top of the deck", draw_takes_from_top),
        case("effects", "draw on an empty deck is a no-op", draw_on_empty_deck),
        case("effects", "draw into a full hand burns the card", draw_burns_at_full_hand),
        case("effects", "draw follow-up discounts a matching card", draw_follow_up_discount),
        case("effects", "draw follow-up discount scales with minions on board", draw_follow_up_scaled_discount),
        case("effects", "draw follow-up copies a matching card", draw_follow_up_copy),
        case("effects", "return to hand resets a buffed minion", bounce_resets_stats),
        case("effects", "return to hand can set a reduced cost", bounce_cost_reduction),
        case("effects", "returning an enemy minion puts it in the enemy hand", bounce_enemy_goes_to_enemy_hand),
        case("effects", "summon mints from the scenario pool", summon_from_pool),
        case("effects", "summon stops at a full board", summon_stops_at_full_board),
        // attributes
        case("attributes", "charge attacks a minion the turn it lands", charge_can_trade_immediately),
        case("attributes", "charge still cannot reach the hero that turn", charge_cannot_hit_hero),
        case("attributes", "guard forces attacks onto itself", guard_forces_attacks),
        case("attributes", "a stealthed attacker ignores guard", stealth_attacker_ignores_guard),
        case("attributes", "stealth blocks being attacked", stealth_blocks_attack),
        case("attributes", "stealth blocks a targeted spell but not an auto one", stealth_blocks_targeting),
        case("attributes", "attacking breaks the attacker's stealth", attacking_breaks_stealth),
        case("attributes", "ward absorbs one instance of spell damage", ward_absorbs_spell),
        case("attributes", "ward absorbs combat damage", ward_absorbs_combat),
        case("attributes", "lifesteal heals the hero on attack", lifesteal_attribute_on_attack),
        case("attributes", "poisonous kills whatever it damages", poisonous_kills),
        case("attributes", "only tradeable cards can be traded", tradeable_gate),
        // triggers
        case("triggers", "fanfare fires when the minion is played", fanfare_fires),
        case("triggers", "deathwish fires when the minion dies", deathwish_fires),
        case("triggers", "a deathwish that kills fires the next deathwish", deathwish_chains),
        // requirements
        case("requirements", "combo is off on the first card of the turn", combo_blocked),
        case("requirements", "combo is on after a card has been played", combo_allowed),
        case("requirements", "quickdraw only fires on a freshly drawn card", quickdraw_gate),
        case("requirements", "is holding checks the rest of the hand", is_holding_gate),
        // rules
        case("rules", "a card over your mana cannot be played", mana_gate),
        case("rules", "playing a card spends its mana", mana_is_spent),
        case("rules", "a full board rejects another minion", board_full_gate),
        case("rules", "an exhausted minion cannot attack", exhausted_cannot_attack),
        case("rules", "a zero attack minion cannot attack", zero_attack_cannot_attack),
        case("rules", "a summoning sick minion cannot hit the hero", summoning_sick_cannot_hit_hero),
        case("rules", "a targeted spell with no target is rejected", targeted_spell_needs_a_target),
        case("rules", "dead minions move to the graveyard", deaths_go_to_graveyard),
        case("rules", "end turn draws, refills mana and clears exhaustion", end_turn_upkeep),
        case("rules", "trading puts the card on the bottom and draws", trade_bottoms_and_draws),
    ];
    cases.extend(auto_target_matrix());
    cases.extend(targeted_validation_matrix());
    cases
}

// ── targeting matrices ──────────────────────────────────────

const SIDES: [TargetSide; 3] = [TargetSide::Friendly, TargetSide::Enemy, TargetSide::All];
const ENTITIES: [EntityType; 3] = [EntityType::Minion, EntityType::Hero, EntityType::All];

fn side_name(s: &TargetSide) -> &'static str {
    match s {
        TargetSide::Friendly => "friendly",
        TargetSide::Enemy => "enemy",
        TargetSide::All => "all sides",
    }
}

fn entity_name(e: &EntityType) -> &'static str {
    match e {
        EntityType::Minion => "minions",
        EntityType::Hero => "heroes",
        EntityType::All => "everything",
    }
}

fn hits_minions(e: &EntityType) -> bool {
    matches!(e, EntityType::Minion | EntityType::All)
}

fn hits_heroes(e: &EntityType) -> bool {
    matches!(e, EntityType::Hero | EntityType::All)
}

fn covers_friendly(s: &TargetSide) -> bool {
    matches!(s, TargetSide::Friendly | TargetSide::All)
}

fn covers_enemy(s: &TargetSide) -> bool {
    matches!(s, TargetSide::Enemy | TargetSide::All)
}

/// Every `(side, entity_type)` pair under `TargetMode::Auto`, checked by who took the damage.
fn auto_target_matrix() -> Vec<Case> {
    let mut out = vec![];
    for side in SIDES {
        for entity in ENTITIES {
            let name = format!("auto damage hits {} {}", side_name(&side), entity_name(&entity));
            let s = side.clone();
            let e = entity.clone();
            out.push(Case {
                name,
                group: "targeting",
                run: Box::new(move || auto_target_case(s.clone(), e.clone())),
            });
        }
    }
    out
}

fn auto_target_case(side: TargetSide, entity: EntityType) -> Scenario {
    let mut s = Setup::new();
    let mine = s.on_board(Side::White, minion(1, "Mine", 1, 1, 5));
    let theirs = s.on_board(Side::Black, minion(2, "Theirs", 1, 1, 5));
    s.hp(Side::White, 20);
    s.hp(Side::Black, 20);
    let bolt = s.spell_in_hand(
        Side::White,
        incantation(3, "Spread", 0).does(on_play(vec![damage(spec(TargetMode::Auto, side.clone(), entity.clone()), 1)])),
    );

    let label = format!("auto {} {}", side_name(&side), entity_name(&entity));
    let mut scn = s.start(&label);
    scn.play(bolt, None);

    let friendly_minion = if covers_friendly(&side) && hits_minions(&entity) { 4 } else { 5 };
    let enemy_minion = if covers_enemy(&side) && hits_minions(&entity) { 4 } else { 5 };
    let friendly_hero = if covers_friendly(&side) && hits_heroes(&entity) { 19 } else { 20 };
    let enemy_hero = if covers_enemy(&side) && hits_heroes(&entity) { 19 } else { 20 };

    scn.expect_eq("friendly minion", scn.defence(mine), Some(friendly_minion));
    scn.expect_eq("enemy minion", scn.defence(theirs), Some(enemy_minion));
    scn.expect_eq("friendly hero", scn.hp(Side::White), friendly_hero);
    scn.expect_eq("enemy hero", scn.hp(Side::Black), enemy_hero);
    scn
}

/// Every `(side, entity_type)` pair under `TargetMode::Targeted`, checked by which picks are legal.
fn targeted_validation_matrix() -> Vec<Case> {
    let mut out = vec![];
    for side in SIDES {
        for entity in ENTITIES {
            let name = format!("targeted {} {} accepts only legal picks", side_name(&side), entity_name(&entity));
            let s = side.clone();
            let e = entity.clone();
            out.push(Case {
                name,
                group: "targeting",
                run: Box::new(move || targeted_validation_case(s.clone(), e.clone())),
            });
        }
    }
    out
}

fn targeted_validation_case(side: TargetSide, entity: EntityType) -> Scenario {
    let want = [
        ("friendly minion", covers_friendly(&side) && hits_minions(&entity)),
        ("enemy minion", covers_enemy(&side) && hits_minions(&entity)),
        ("friendly hero", covers_friendly(&side) && hits_heroes(&entity)),
        ("enemy hero", covers_enemy(&side) && hits_heroes(&entity)),
    ];

    let label = format!("targeted {} {}", side_name(&side), entity_name(&entity));
    let mut scn = Setup::new().start(&label);

    // one fresh game per pick, so an accepted play never disturbs the next attempt
    for (i, (what, expected)) in want.iter().enumerate() {
        let mut s = Setup::new();
        let mine = s.on_board(Side::White, minion(1, "Mine", 1, 1, 5));
        let theirs = s.on_board(Side::Black, minion(2, "Theirs", 1, 1, 5));
        let bolt = s.spell_in_hand(
            Side::White,
            incantation(3, "Snipe", 0).does(on_play(vec![damage(spec(TargetMode::Targeted, side.clone(), entity.clone()), 1)])),
        );
        let white_hero = s.hero_id(Side::White);
        let black_hero = s.hero_id(Side::Black);
        let pick = [mine, theirs, white_hero, black_hero][i];

        let mut inner = s.start(&label);
        let accepted = inner.play(bolt, Some(pick));
        scn.expect_eq(&format!("{what} accepted"), accepted, *expected);
    }
    scn
}

// ── effects ─────────────────────────────────────────────────

fn buff_raises_ceiling() -> Scenario {
    let mut s = Setup::new();
    let target = s.on_board(Side::White, minion(1, "Squire", 1, 1, 2));
    let card = s.spell_in_hand(
        Side::White,
        incantation(2, "Blessing", 0).does(on_play(vec![buff(pick_friendly_minion(), 2, 3)])),
    );
    let mut scn = s.start("buff");
    scn.play(card, Some(target));
    scn.expect_eq("attack", scn.attack_of(target), Some(3));
    scn.expect_eq("defence", scn.defence(target), Some(5));
    scn
}

fn damage_reduces_defence() -> Scenario {
    let mut s = Setup::new();
    let target = s.on_board(Side::Black, minion(1, "Ogre", 4, 4, 7));
    let card = s.spell_in_hand(
        Side::White,
        incantation(2, "Bolt", 0).does(on_play(vec![damage(pick_enemy_minion(), 3)])),
    );
    let mut scn = s.start("damage");
    scn.play(card, Some(target));
    scn.expect_eq("defence", scn.defence(target), Some(4));
    scn
}

fn lifesteal_scales_with_hits() -> Scenario {
    let mut s = Setup::new();
    s.on_board(Side::Black, minion(1, "A", 1, 1, 5));
    s.on_board(Side::Black, minion(2, "B", 1, 1, 5));
    s.hp(Side::White, 10);
    let card = s.spell_in_hand(
        Side::White,
        incantation(3, "Siphon", 0).does(on_play(vec![drain(spec(TargetMode::Auto, TargetSide::Enemy, EntityType::Minion), 2)])),
    );
    let mut scn = s.start("lifesteal effect");
    scn.play(card, None);
    // 2 damage across 2 minions heals 4
    scn.expect_eq("hero healed per hit", scn.hp(Side::White), 14);
    scn
}

fn heal_caps_at_max_defence() -> Scenario {
    let mut s = Setup::new();
    let target = s.on_board(Side::White, minion(1, "Squire", 1, 1, 5));
    let hurt = s.spell_in_hand(
        Side::White,
        incantation(2, "Cut", 0).does(on_play(vec![damage(pick_friendly_minion(), 3)])),
    );
    let mend = s.spell_in_hand(
        Side::White,
        incantation(3, "Mend", 0).does(on_play(vec![heal(pick_friendly_minion(), 9)])),
    );
    let mut scn = s.start("heal cap");
    scn.play(hurt, Some(target));
    scn.expect_eq("after damage", scn.defence(target), Some(2));
    // index shifts down after the first card leaves the hand
    scn.play(mend - 1, Some(target));
    scn.expect_eq("healed no further than max", scn.defence(target), Some(5));
    scn
}

fn heal_caps_at_max_hp() -> Scenario {
    let mut s = Setup::new();
    s.hp(Side::White, settings::MAX_HP - 1);
    let card = s.spell_in_hand(
        Side::White,
        incantation(1, "Mend", 0).does(on_play(vec![heal(spec(TargetMode::Auto, TargetSide::Friendly, EntityType::Hero), 10)])),
    );
    let mut scn = s.start("hero heal cap");
    scn.play(card, None);
    scn.expect_eq("hp", scn.hp(Side::White), settings::MAX_HP);
    scn
}

fn destroy_ignores_ward() -> Scenario {
    let mut s = Setup::new();
    let target = s.on_board(Side::Black, minion(1, "Warded", 4, 4, 8).with(MinionAttribute::Ward));
    let card = s.spell_in_hand(
        Side::White,
        incantation(2, "Doom", 0).does(on_play(vec![destroy(pick_enemy_minion())])),
    );
    let mut scn = s.start("destroy vs ward");
    scn.expect("ward is up", scn.has_ward(target));
    scn.play(card, Some(target));
    scn.expect("destroyed anyway", !scn.alive(target));
    scn
}

fn draw_takes_from_top() -> Scenario {
    let mut s = Setup::new();
    let top = s.in_deck(Side::White, minion(1, "Top", 1, 1, 1));
    s.in_deck(Side::White, minion(2, "Below", 1, 1, 1));
    let card = s.spell_in_hand(Side::White, incantation(3, "Study", 0).does(on_play(vec![draw(1)])));
    let mut scn = s.start("draw");
    scn.play(card, None);
    scn.expect("top card is in hand", scn.in_hand(Side::White, top));
    scn.expect_eq("deck shrank", scn.deck_len(Side::White), 1);
    scn
}

fn draw_on_empty_deck() -> Scenario {
    let mut s = Setup::new();
    let card = s.spell_in_hand(Side::White, incantation(1, "Study", 0).does(on_play(vec![draw(2)])));
    let mut scn = s.start("draw empty");
    scn.play(card, None);
    scn.expect_eq("hand is empty", scn.hand_len(Side::White), 0);
    scn.expect_eq("deck is empty", scn.deck_len(Side::White), 0);
    scn
}

fn draw_burns_at_full_hand() -> Scenario {
    let mut s = Setup::new();
    // the draw spell occupies one slot, so fill the rest and it is full the moment it resolves
    for i in 0..settings::MAX_HAND_SIZE {
        s.in_hand(Side::White, minion(100 + i as u32, "Filler", 1, 1, 1));
    }
    s.in_deck(Side::White, minion(1, "Burned", 1, 1, 1));
    let card = s.spell_in_hand(Side::White, incantation(2, "Study", 0).does(on_play(vec![draw(1)])));
    let mut scn = s.start("hand burn");
    // the spell was appended past the cap during setup; playing it frees a slot back to the cap
    scn.play(card, None);
    scn.expect_eq("hand did not exceed the cap", scn.hand_len(Side::White), settings::MAX_HAND_SIZE);
    scn.expect_eq("card left the deck", scn.deck_len(Side::White), 0);
    scn
}

fn draw_follow_up_discount() -> Scenario {
    let mut s = Setup::new();
    s.in_deck(Side::White, minion(1, "Wolf", 5, 3, 3).of(Race::Beast));
    let card = s.spell_in_hand(
        Side::White,
        incantation(2, "Call", 0).does(on_play(vec![draw_then(
            1,
            follow_up(vec![Condition::IsRace { race: Race::Beast }], vec![discount(2, None)]),
        )])),
    );
    let mut scn = s.start("draw discount");
    scn.play(card, None);
    scn.expect_eq("cost reduced", scn.hand_cost(Side::White, 0), Some(3));
    scn
}

fn draw_follow_up_scaled_discount() -> Scenario {
    let mut s = Setup::new();
    s.on_board(Side::White, minion(1, "A", 1, 1, 1));
    s.on_board(Side::White, minion(2, "B", 1, 1, 1));
    s.in_deck(Side::White, minion(3, "Wolf", 5, 3, 3).of(Race::Beast));
    let card = s.spell_in_hand(
        Side::White,
        incantation(4, "Call", 0).does(on_play(vec![draw_then(
            1,
            follow_up(vec![Condition::IsRace { race: Race::Beast }], vec![discount(1, Some(ScaledBy::MinionsOnBoard))]),
        )])),
    );
    let mut scn = s.start("scaled discount");
    scn.play(card, None);
    scn.expect_eq("2 minions means 2 off", scn.hand_cost(Side::White, 0), Some(3));
    scn
}

fn draw_follow_up_copy() -> Scenario {
    let mut s = Setup::new();
    s.in_deck(Side::White, minion(1, "Wolf", 2, 3, 3).of(Race::Beast));
    let card = s.spell_in_hand(
        Side::White,
        incantation(2, "Echo", 0).does(on_play(vec![draw_then(
            1,
            follow_up(vec![Condition::IsRace { race: Race::Beast }], vec![copies(1)]),
        )])),
    );
    let mut scn = s.start("draw copy");
    scn.play(card, None);
    scn.expect_eq("drawn card plus one copy", scn.hand_len(Side::White), 2);
    scn
}

fn bounce_resets_stats() -> Scenario {
    let mut s = Setup::new();
    let target = s.on_board(Side::White, minion(1, "Squire", 3, 1, 2));
    let pump = s.spell_in_hand(
        Side::White,
        incantation(2, "Blessing", 0).does(on_play(vec![buff(pick_friendly_minion(), 4, 4)])),
    );
    let recall = s.spell_in_hand(
        Side::White,
        incantation(3, "Recall", 0).does(on_play(vec![bounce(pick_friendly_minion(), None)])),
    );
    let mut scn = s.start("bounce reset");
    scn.play(pump, Some(target));
    scn.expect_eq("buffed", scn.attack_of(target), Some(5));
    scn.play(recall - 1, Some(target));
    scn.expect("off the board", !scn.alive(target));
    scn.expect("back in hand", scn.in_hand(Side::White, target));
    scn.expect_eq("cost is the printed cost", scn.hand_cost(Side::White, 0), Some(3));
    scn
}

fn bounce_cost_reduction() -> Scenario {
    let mut s = Setup::new();
    let target = s.on_board(Side::White, minion(1, "Squire", 5, 1, 2));
    let recall = s.spell_in_hand(
        Side::White,
        incantation(2, "Recall", 0).does(on_play(vec![bounce(pick_friendly_minion(), Some(2))])),
    );
    let mut scn = s.start("bounce discount");
    scn.play(recall, Some(target));
    scn.expect_eq("cost reduced from the printed cost", scn.hand_cost(Side::White, 0), Some(3));
    scn
}

fn bounce_enemy_goes_to_enemy_hand() -> Scenario {
    let mut s = Setup::new();
    let target = s.on_board(Side::Black, minion(1, "Ogre", 4, 4, 4));
    let card = s.spell_in_hand(
        Side::White,
        incantation(2, "Banish", 0).does(on_play(vec![bounce(pick_enemy_minion(), None)])),
    );
    let mut scn = s.start("bounce enemy");
    scn.play(card, Some(target));
    scn.expect("left the board", !scn.alive(target));
    scn.expect("in the enemy hand", scn.in_hand(Side::Black, target));
    scn.expect_eq("not in our hand", scn.hand_len(Side::White), 0);
    scn
}

fn summon_from_pool() -> Scenario {
    let token = minion(1, "Wisp", 0, 1, 1);
    let mut s = Setup::new();
    s.summonable(token.clone());
    let card = s.spell_in_hand(
        Side::White,
        incantation(2, "Conjure", 0).does(on_play(vec![summon(&token, 2)])),
    );
    let mut scn = s.start("summon");
    scn.play(card, None);
    scn.expect_eq("two tokens", scn.board_len(Side::White), 2);
    scn
}

fn summon_stops_at_full_board() -> Scenario {
    let token = minion(1, "Wisp", 0, 1, 1);
    let mut s = Setup::new();
    s.summonable(token.clone());
    for i in 0..settings::MAX_BOARD_SIZE - 1 {
        s.on_board(Side::White, minion(10 + i as u32, "Filler", 1, 1, 1));
    }
    let card = s.spell_in_hand(
        Side::White,
        incantation(2, "Conjure", 0).does(on_play(vec![summon(&token, 3)])),
    );
    let mut scn = s.start("summon cap");
    scn.play(card, None);
    scn.expect_eq("board stopped at the cap", scn.board_len(Side::White), settings::MAX_BOARD_SIZE);
    scn
}

// ── attributes ──────────────────────────────────────────────

fn charge_can_trade_immediately() -> Scenario {
    let mut s = Setup::new();
    let prey = s.on_board(Side::Black, minion(1, "Prey", 1, 0, 2));
    let idx = s.in_hand(Side::White, minion(2, "Raider", 1, 2, 2).with(MinionAttribute::Charge));
    let mut scn = s.start("charge trade");
    scn.play(idx, None);
    let raider = scn.board(Side::White).battlefield[0].entity_id;
    scn.expect("not exhausted", !scn.exhausted(raider));
    scn.expect_attack("attack landed", raider, prey, true);
    scn.expect("prey died", !scn.alive(prey));
    scn
}

fn charge_cannot_hit_hero() -> Scenario {
    let mut s = Setup::new();
    let idx = s.in_hand(Side::White, minion(1, "Raider", 1, 2, 2).with(MinionAttribute::Charge));
    let mut scn = s.start("charge vs hero");
    scn.play(idx, None);
    let raider = scn.board(Side::White).battlefield[0].entity_id;
    let foe_hero = scn.hero_id(Side::Black);
    scn.expect_attack("hero attack rejected", raider, foe_hero, false);
    scn
}

fn guard_forces_attacks() -> Scenario {
    let mut s = Setup::new();
    let guard = s.on_board(Side::Black, minion(1, "Wall", 2, 0, 5).with(MinionAttribute::Guard));
    let soft = s.on_board(Side::Black, minion(2, "Soft", 1, 0, 2));
    let attacker = s.on_board(Side::White, minion(3, "Raider", 2, 2, 2));
    let mut scn = s.start("guard");
    scn.expect_attack("cannot reach the soft target", attacker, soft, false);
    scn.expect_attack("can hit the guard", attacker, guard, true);
    scn
}

fn stealth_attacker_ignores_guard() -> Scenario {
    let mut s = Setup::new();
    s.on_board(Side::Black, minion(1, "Wall", 2, 0, 5).with(MinionAttribute::Guard));
    let soft = s.on_board(Side::Black, minion(2, "Soft", 1, 0, 2));
    let attacker = s.on_board(Side::White, minion(3, "Sneak", 2, 2, 2).with(MinionAttribute::Stealth));
    let mut scn = s.start("stealth vs guard");
    scn.expect_attack("stealth walks past the guard", attacker, soft, true);
    scn
}

fn stealth_blocks_attack() -> Scenario {
    let mut s = Setup::new();
    let hidden = s.on_board(Side::Black, minion(1, "Hidden", 1, 1, 2).with(MinionAttribute::Stealth));
    let attacker = s.on_board(Side::White, minion(2, "Raider", 2, 2, 2));
    let mut scn = s.start("stealth defender");
    scn.expect_attack("cannot be attacked", attacker, hidden, false);
    scn
}

fn stealth_blocks_targeting() -> Scenario {
    let mut s = Setup::new();
    let hidden = s.on_board(Side::Black, minion(1, "Hidden", 1, 1, 5).with(MinionAttribute::Stealth));
    let snipe = s.spell_in_hand(
        Side::White,
        incantation(2, "Snipe", 0).does(on_play(vec![damage(pick_enemy_minion(), 1)])),
    );
    let sweep = s.spell_in_hand(
        Side::White,
        incantation(3, "Sweep", 0).does(on_play(vec![damage(spec(TargetMode::Auto, TargetSide::Enemy, EntityType::Minion), 1)])),
    );
    let mut scn = s.start("stealth targeting");
    scn.expect_play("targeted spell rejected", snipe, Some(hidden), false);
    scn.play(sweep, None);
    scn.expect_eq("auto spell still hits", scn.defence(hidden), Some(4));
    scn
}

fn attacking_breaks_stealth() -> Scenario {
    let mut s = Setup::new();
    let prey = s.on_board(Side::Black, minion(1, "Prey", 1, 0, 5));
    let sneak = s.on_board(Side::White, minion(2, "Sneak", 2, 2, 2).with(MinionAttribute::Stealth));
    let mut scn = s.start("stealth breaks");
    scn.expect("stealth is up", scn.has_stealth(sneak));
    scn.attack(sneak, prey);
    scn.expect("stealth is gone", !scn.has_stealth(sneak));
    scn
}

fn ward_absorbs_spell() -> Scenario {
    let mut s = Setup::new();
    let target = s.on_board(Side::Black, minion(1, "Warded", 3, 2, 3).with(MinionAttribute::Ward));
    let card = s.spell_in_hand(
        Side::White,
        incantation(2, "Bolt", 0).does(on_play(vec![damage(pick_enemy_minion(), 99)])),
    );
    let mut scn = s.start("ward vs spell");
    scn.play(card, Some(target));
    scn.expect("survived", scn.alive(target));
    scn.expect_eq("undamaged", scn.defence(target), Some(3));
    scn.expect("ward is spent", !scn.has_ward(target));
    scn
}

fn ward_absorbs_combat() -> Scenario {
    let mut s = Setup::new();
    let big = s.on_board(Side::Black, minion(1, "Ogre", 4, 9, 9));
    let attacker = s.on_board(Side::White, minion(2, "Warded", 2, 2, 2).with(MinionAttribute::Ward));
    let mut scn = s.start("ward vs combat");
    scn.attack(attacker, big);
    scn.expect("attacker survived", scn.alive(attacker));
    scn.expect_eq("attacker undamaged", scn.defence(attacker), Some(2));
    scn.expect("ward is spent", !scn.has_ward(attacker));
    scn.expect_eq("defender still took the hit", scn.defence(big), Some(7));
    scn
}

fn lifesteal_attribute_on_attack() -> Scenario {
    let mut s = Setup::new();
    s.hp(Side::White, 10);
    let prey = s.on_board(Side::Black, minion(1, "Prey", 1, 0, 9));
    let vamp = s.on_board(Side::White, minion(2, "Vampire", 3, 3, 3).with(MinionAttribute::Lifesteal));
    let mut scn = s.start("lifesteal attribute");
    scn.attack(vamp, prey);
    scn.expect_eq("healed for the attack", scn.hp(Side::White), 13);
    scn
}

fn poisonous_kills() -> Scenario {
    let mut s = Setup::new();
    let big = s.on_board(Side::Black, minion(1, "Colossus", 8, 0, 12));
    let snake = s.on_board(Side::White, minion(2, "Snake", 2, 1, 3).with(MinionAttribute::Poisonous));
    let mut scn = s.start("poisonous");
    scn.attack(snake, big);
    scn.expect("colossus died", !scn.alive(big));
    scn.expect("snake lived", scn.alive(snake));
    scn
}

fn tradeable_gate() -> Scenario {
    let mut s = Setup::new();
    let plain = s.in_hand(Side::White, minion(1, "Plain", 1, 1, 1));
    let swap = s.in_hand(Side::White, minion(2, "Swap", 1, 1, 1).with(MinionAttribute::Tradeable));
    s.in_deck(Side::White, minion(3, "Fresh", 1, 1, 1));
    let mut scn = s.start("tradeable");
    scn.expect_trade("plain card cannot trade", plain, false);
    scn.expect_trade("tradeable card can", swap, true);
    scn
}

// ── triggers ────────────────────────────────────────────────

fn fanfare_fires() -> Scenario {
    let mut s = Setup::new();
    s.hp(Side::Black, 20);
    let idx = s.in_hand(
        Side::White,
        minion(1, "Herald", 2, 1, 1).does(on_play(vec![damage(spec(TargetMode::Auto, TargetSide::Enemy, EntityType::Hero), 3)])),
    );
    let mut scn = s.start("fanfare");
    scn.play(idx, None);
    scn.expect_eq("enemy hero took 3", scn.hp(Side::Black), 17);
    scn
}

fn deathwish_fires() -> Scenario {
    let mut s = Setup::new();
    s.hp(Side::Black, 20);
    let bomb = s.on_board(
        Side::White,
        minion(1, "Bomb", 2, 1, 1).does(on_death(vec![damage(spec(TargetMode::Auto, TargetSide::Enemy, EntityType::Hero), 4)])),
    );
    let card = s.spell_in_hand(
        Side::White,
        incantation(2, "Sacrifice", 0).does(on_play(vec![destroy(pick_friendly_minion())])),
    );
    let mut scn = s.start("deathwish");
    scn.play(card, Some(bomb));
    scn.expect("bomb died", !scn.alive(bomb));
    scn.expect_eq("deathwish hit the hero", scn.hp(Side::Black), 16);
    scn
}

fn deathwish_chains() -> Scenario {
    let mut s = Setup::new();
    s.hp(Side::Black, 20);
    // second bomb dies to the first bomb's blast, and its own deathwish must still resolve
    let second = s.on_board(
        Side::White,
        minion(2, "Second", 2, 1, 1).does(on_death(vec![damage(spec(TargetMode::Auto, TargetSide::Enemy, EntityType::Hero), 5)])),
    );
    let first = s.on_board(
        Side::White,
        minion(1, "First", 2, 1, 1).does(on_death(vec![damage(spec(TargetMode::Auto, TargetSide::Friendly, EntityType::Minion), 9)])),
    );
    let card = s.spell_in_hand(
        Side::White,
        incantation(3, "Sacrifice", 0).does(on_play(vec![destroy(pick_friendly_minion())])),
    );
    let mut scn = s.start("deathwish chain");
    scn.play(card, Some(first));
    scn.expect("first died", !scn.alive(first));
    scn.expect("second died to the blast", !scn.alive(second));
    scn.expect_eq("chained deathwish resolved", scn.hp(Side::Black), 15);
    scn
}

// ── requirements ────────────────────────────────────────────

fn combo_card(id: u32) -> IncantationCard {
    incantation(id, "Combo", 0).does(requiring(
        on_play(vec![damage(spec(TargetMode::Auto, TargetSide::Enemy, EntityType::Hero), 3)]),
        Requirement::Combo,
    ))
}

fn combo_blocked() -> Scenario {
    let mut s = Setup::new();
    s.hp(Side::Black, 20);
    let card = s.spell_in_hand(Side::White, combo_card(1));
    let mut scn = s.start("combo off");
    scn.play(card, None);
    scn.expect_eq("no damage on the first card", scn.hp(Side::Black), 20);
    scn
}

fn combo_allowed() -> Scenario {
    let mut s = Setup::new();
    s.hp(Side::Black, 20);
    s.cards_played_this_turn(1);
    let card = s.spell_in_hand(Side::White, combo_card(1));
    let mut scn = s.start("combo on");
    scn.play(card, None);
    scn.expect_eq("combo damage landed", scn.hp(Side::Black), 17);
    scn
}

fn quickdraw_gate() -> Scenario {
    let mut s = Setup::new();
    s.hp(Side::Black, 20);
    let quick = incantation(1, "Quickdraw", 0).does(requiring(
        on_play(vec![damage(spec(TargetMode::Auto, TargetSide::Enemy, EntityType::Hero), 3)]),
        Requirement::Quickdraw,
    ));
    let stale = s.spell_in_hand(Side::White, quick.clone());
    let fresh = s.spell_in_hand(Side::White, quick);
    s.mark_just_drawn(Side::White, fresh);
    let mut scn = s.start("quickdraw");
    scn.play(stale, None);
    scn.expect_eq("stale card did nothing", scn.hp(Side::Black), 20);
    scn.play(fresh - 1, None);
    scn.expect_eq("fresh card fired", scn.hp(Side::Black), 17);
    scn
}

fn is_holding_gate() -> Scenario {
    let mut s = Setup::new();
    s.hp(Side::Black, 20);
    let card = s.spell_in_hand(
        Side::White,
        incantation(1, "Kennel", 0).does(requiring(
            on_play(vec![damage(spec(TargetMode::Auto, TargetSide::Enemy, EntityType::Hero), 3)]),
            Requirement::IsHolding { filters: vec![Condition::IsRace { race: Race::Beast }] },
        )),
    );
    s.in_hand(Side::White, minion(2, "Wolf", 1, 1, 1).of(Race::Beast));
    let mut scn = s.start("is holding");
    scn.play(card, None);
    scn.expect_eq("fired while holding a beast", scn.hp(Side::Black), 17);
    scn
}

// ── rules ───────────────────────────────────────────────────

fn mana_gate() -> Scenario {
    let mut s = Setup::new();
    s.mana(Side::White, 2);
    let idx = s.in_hand(Side::White, minion(1, "Giant", 8, 8, 8));
    let mut scn = s.start("mana gate");
    scn.expect_play("too expensive", idx, None, false);
    scn.expect_eq("still in hand", scn.hand_len(Side::White), 1);
    scn
}

fn mana_is_spent() -> Scenario {
    let mut s = Setup::new();
    s.mana(Side::White, 5);
    let idx = s.in_hand(Side::White, minion(1, "Squire", 3, 1, 1));
    let mut scn = s.start("mana spend");
    scn.play(idx, None);
    scn.expect_eq("mana left", scn.mana(Side::White), 2);
    scn
}

fn board_full_gate() -> Scenario {
    let mut s = Setup::new();
    for i in 0..settings::MAX_BOARD_SIZE {
        s.on_board(Side::White, minion(10 + i as u32, "Filler", 1, 1, 1));
    }
    let idx = s.in_hand(Side::White, minion(1, "Extra", 1, 1, 1));
    let mut scn = s.start("board full");
    scn.expect_play("rejected", idx, None, false);
    scn.expect_eq("board unchanged", scn.board_len(Side::White), settings::MAX_BOARD_SIZE);
    scn
}

fn exhausted_cannot_attack() -> Scenario {
    let mut s = Setup::new();
    let prey = s.on_board(Side::Black, minion(1, "Prey", 1, 0, 5));
    let tired = s.on_board_fresh(Side::White, minion(2, "Tired", 2, 2, 2));
    let mut scn = s.start("exhausted");
    scn.expect_attack("rejected", tired, prey, false);
    scn
}

fn zero_attack_cannot_attack() -> Scenario {
    let mut s = Setup::new();
    let prey = s.on_board(Side::Black, minion(1, "Prey", 1, 0, 5));
    let pacifist = s.on_board(Side::White, minion(2, "Pacifist", 2, 0, 5));
    let mut scn = s.start("zero attack");
    scn.expect_attack("rejected", pacifist, prey, false);
    scn
}

fn summoning_sick_cannot_hit_hero() -> Scenario {
    let mut s = Setup::new();
    let idx = s.in_hand(Side::White, minion(1, "Raider", 1, 2, 2).with(MinionAttribute::Charge));
    let mut scn = s.start("summoning sick vs hero");
    scn.play(idx, None);
    let raider = scn.board(Side::White).battlefield[0].entity_id;
    let foe_hero = scn.hero_id(Side::Black);
    scn.expect_attack("rejected on the turn it landed", raider, foe_hero, false);
    scn.end_turn();
    scn.end_turn();
    scn.expect_attack("allowed a turn later", raider, foe_hero, true);
    scn
}

fn targeted_spell_needs_a_target() -> Scenario {
    let mut s = Setup::new();
    s.on_board(Side::Black, minion(1, "Ogre", 4, 4, 4));
    let card = s.spell_in_hand(
        Side::White,
        incantation(2, "Snipe", 0).does(on_play(vec![damage(pick_enemy_minion(), 1)])),
    );
    let mut scn = s.start("targeted needs target");
    scn.expect_play("rejected without a target", card, None, false);
    scn
}

fn deaths_go_to_graveyard() -> Scenario {
    let mut s = Setup::new();
    let doomed = s.on_board(Side::Black, minion(1, "Doomed", 1, 0, 2));
    let card = s.spell_in_hand(
        Side::White,
        incantation(2, "Bolt", 0).does(on_play(vec![damage(pick_enemy_minion(), 5)])),
    );
    let mut scn = s.start("graveyard");
    scn.play(card, Some(doomed));
    scn.expect("off the board", !scn.alive(doomed));
    scn.expect_eq("in the enemy graveyard", scn.graveyard_len(Side::Black), 1);
    scn
}

fn end_turn_upkeep() -> Scenario {
    let mut s = Setup::new();
    s.mana(Side::White, 3);
    s.embers(Side::White, 1);
    s.in_deck(Side::White, minion(1, "Fresh", 1, 1, 1));
    let tired = s.on_board_fresh(Side::White, minion(2, "Tired", 2, 2, 2));
    let mut scn = s.start("upkeep");
    scn.end_turn(); // to black
    scn.end_turn(); // back to white
    scn.expect_eq("drew a card", scn.hand_len(Side::White), 1);
    scn.expect_eq("mana went up", scn.mana(Side::White), 4);
    scn.expect_eq("ember went up", scn.embers(Side::White), 2);
    scn.expect("exhaustion cleared", !scn.exhausted(tired));
    scn
}

fn trade_bottoms_and_draws() -> Scenario {
    let mut s = Setup::new();
    s.mana(Side::White, 3);
    let traded = s.in_hand(Side::White, minion(1, "Swap", 1, 1, 1).with(MinionAttribute::Tradeable));
    let traded_id = s.hand_entity_id(Side::White, traded);
    let fresh = s.in_deck(Side::White, minion(2, "Fresh", 1, 1, 1));
    let mut scn = s.start("trade");
    scn.expect_trade("trade accepted", traded, true);
    scn.expect("drew the top card", scn.in_hand(Side::White, fresh));
    scn.expect("traded card left the hand", !scn.in_hand(Side::White, traded_id));
    scn.expect_eq("deck size unchanged", scn.deck_len(Side::White), 1);
    scn.expect_eq("trade cost a mana", scn.mana(Side::White), 2);
    scn
}
