use crate::broadcast;
use crate::engine::Game;
use crate::settings;
use shared::types::*;
use socketioxide::SocketIo;

pub const BOT_ID: &str = "bot";
pub const HERO_ID: u32 = 105; // Clown Snake

/// The Gunslinger list from decks.json, kept in sync by hand.
pub fn default_deck() -> Vec<u32> {
    vec![
        25, 25, 25, // Quick Shot (1 cost, tradeable, 1 to an enemy, quickdraw deals 2 more)
        42, 42,     // Gunsmith (1 cost 1/3, +1+1 on every quickdraw)
        41,         // Dust Jackal (1 cost 1/1 beast, blitz)
        43,         // Toad (1 cost 1/1 beast, deathwish random omen)
        44,         // Rat Cage (1 cost 0/3, deathwish three rats)
        27, 27, 27, // Knife Thrower (2 cost 2/2, tradeable, quickdraw 2 to an enemy minion)
        46, 46,     // Gunslinger (2 cost 2/4, pings all enemies on every quickdraw)
        32, 32,     // Hat Trick (2 cost, draw, quickdraw copies it)
        45,         // Card Shark (2 cost 2/2 guard, fanfare reverse your deck)
        47,         // Shiny Magpie (2 cost 2/2, +1+1 after an arrival was played)
        59,         // Bottom Deal (2 cost, reverse your deck then draw)
        49,         // Fox Courier (3 cost 2/3 beast, draws with another beast out)
        50,         // Bristleback (3 cost 3/3 beast, guard)
        51, 51,     // Fat Rat (4 cost 3/3 beast, tradeable, quickdraw summons a 2/2 blitz rat)
        52,         // Bear (5 cost 5/6 beast, tradeable)
        53,         // Gazelle (6 cost 4/4 beast, blitz, cheaper per quickdraw)
        54,         // The Dentist (7 cost 7/7, fanfare replays every quickdraw)
    ]
}

fn threat_score(m: &MinionEntity) -> i32 {
    m.stats.attack * 2 + m.stats.defence
}

/// Retaliation is an attack like any other, so it reads with the same rules reversed.
fn is_trade_lethal(attacker: &MinionEntity, defender: &MinionEntity) -> bool {
    would_kill(defender, attacker)
}

fn would_kill(attacker: &MinionEntity, defender: &MinionEntity) -> bool {
    if defender.ward_active {
        return false; // the ward eats the hit, poison with it
    }
    let poisonous = attacker.card.attributes.contains(&MinionAttribute::Poisonous);
    attacker.stats.attack >= defender.stats.defence || (poisonous && attacker.stats.attack > 0)
}

/// The engine's rush rule: a minion that arrived this turn may only attack minions.
fn can_hit_hero(m: &MinionEntity) -> bool {
    m.turns_on_board > 0
}

fn trade_score(attacker: &MinionEntity, defender: &MinionEntity) -> i32 {
    let mut score = 0;
    if would_kill(attacker, defender) {
        score += threat_score(defender) * 3;
    }
    if is_trade_lethal(attacker, defender) {
        score -= threat_score(attacker) * 2;
    }
    if would_kill(attacker, defender) && !is_trade_lethal(attacker, defender) {
        score += 10;
    }
    score
}

fn can_go_lethal(minions: &[MinionEntity], enemy_board: &Board) -> bool {
    if enemy_board.battlefield.iter().any(|m| m.card.attributes.contains(&MinionAttribute::Guard)) {
        return false;
    }
    minions.iter().filter(|m| !m.exhausted && can_hit_hero(m)).map(|m| m.stats.attack).sum::<i32>() >= enemy_board.hero.stats.defence
}

fn filter_minions<'a>(minions: &'a [MinionEntity], filters: &[Condition]) -> Vec<&'a MinionEntity> {
    minions.iter().filter(|m| filters.iter().all(|f| match f {
        Condition::IsRace { race } => m.card.races.contains(race),
        Condition::HasAttribute { attribute } => m.card.attributes.contains(attribute),
    })).collect()
}

fn pick_best_spell_target(bot_board: &Board, enemy_board: &Board, effect: &Effect) -> Option<u32> {
    match effect {
        Effect::Damage { target_spec, .. } => {
            let enemies = filter_minions(&enemy_board.battlefield, &target_spec.filters);
            if matches!(target_spec.side, TargetSide::Enemy | TargetSide::All) && !enemies.is_empty() {
                let killable: Vec<&&MinionEntity> = enemies.iter().filter(|m| m.stats.defence <= 4).collect();
                if !killable.is_empty() {
                    return killable.into_iter().max_by_key(|m| threat_score(m)).map(|m| m.entity_id);
                }
                return enemies.iter().max_by_key(|m| threat_score(m)).map(|m| m.entity_id);
            }
            if matches!(target_spec.side, TargetSide::Enemy | TargetSide::All)
                && matches!(target_spec.entity_type, EntityType::Hero | EntityType::All)
            {
                return Some(enemy_board.hero.entity_id);
            }
            None
        }
        Effect::Destroy { target_spec } => {
            let enemies = filter_minions(&enemy_board.battlefield, &target_spec.filters);
            let friendly = filter_minions(&bot_board.battlefield, &target_spec.filters);
            if matches!(target_spec.side, TargetSide::Enemy | TargetSide::All) && !enemies.is_empty() {
                return enemies.iter().max_by_key(|m| threat_score(m)).map(|m| m.entity_id);
            }
            if matches!(target_spec.side, TargetSide::Friendly | TargetSide::All) && !friendly.is_empty() {
                return friendly.iter().min_by_key(|m| threat_score(m)).map(|m| m.entity_id);
            }
            None
        }
        Effect::Buff { target_spec, .. } => {
            let friendly = filter_minions(&bot_board.battlefield, &target_spec.filters);
            friendly.iter().max_by_key(|m| m.stats.attack).map(|m| m.entity_id)
        }
        Effect::ReturnToHand { target_spec, .. } => {
            let friendly = filter_minions(&bot_board.battlefield, &target_spec.filters);
            friendly.iter().min_by_key(|m| threat_score(m)).map(|m| m.entity_id)
        }
        _ => None,
    }
}

fn resolve_card_target(card: &CardEntity, bot_board: &Board, enemy_board: &Board) -> Option<u32> {
    for ability in card.abilities() {
        if ability.trigger != Trigger::OnPlay {
            continue;
        }
        for effect in &ability.effects {
            let target_spec = match effect {
                Effect::Buff { target_spec, .. }
                | Effect::Damage { target_spec, .. }
                | Effect::ReturnToHand { target_spec, .. }
                | Effect::Destroy { target_spec } => target_spec,
                _ => continue,
            };
            if target_spec.target_mode != TargetMode::Targeted {
                continue;
            }
            return pick_best_spell_target(bot_board, enemy_board, effect);
        }
    }
    None
}

fn card_effects(card: &CardEntity) -> Vec<&Effect> {
    let deferred = match card {
        CardEntity::Incantation(i) => i.card.form.deferred_effects(),
        CardEntity::Minion(_) => &[],
    };
    if !deferred.is_empty() {
        return deferred.iter().collect();
    }
    card.abilities()
        .iter()
        .filter(|a| a.trigger == Trigger::OnPlay)
        .flat_map(|a| a.effects.iter())
        .collect()
}

/// Picks the trigger most likely to fire against the board in front of the bot.
fn pick_rite_trigger(armable: &[RiteTrigger], bot_board: &Board, enemy_board: &Board) -> usize {
    let enemy_has_minions = !enemy_board.battlefield.is_empty();
    let bot_has_minions = !bot_board.battlefield.is_empty();

    let likelihood = |t: &RiteTrigger| match t {
        RiteTrigger::EnemyEndsTurn => 2,
        RiteTrigger::EnemyPlaysMinion => 3,
        RiteTrigger::EnemyPlaysIncantation => 2,
        RiteTrigger::EnemyAttacksHero => if enemy_has_minions { 3 } else { 1 },
        RiteTrigger::EnemyAttacksMinion => if enemy_has_minions && bot_has_minions { 3 } else { 1 },
        RiteTrigger::FriendlyMinionDies => if bot_has_minions && enemy_has_minions { 3 } else { 1 },
        RiteTrigger::EnemyEndsTurnWithoutAttacking => if enemy_has_minions { 1 } else { 3 },
        RiteTrigger::EnemyBoardReachesThree => if enemy_board.battlefield.len() >= 2 { 3 } else { 1 },
    };

    let best = armable.iter().map(likelihood).max().unwrap_or(0);
    let tied: Vec<usize> = armable.iter().enumerate()
        .filter(|(_, t)| likelihood(t) == best)
        .map(|(i, _)| i)
        .collect();
    // the bluff dies if the bot always arms the same slot, so ties break at random
    tied[rand::random::<u32>() as usize % tied.len()]
}

fn card_play_score(card: &CardEntity, bot_board: &Board, enemy_board: &Board) -> i32 {
    let mut score = -card.cost();

    for effect in card_effects(card) {
        match effect {
            Effect::Damage { damage, .. } => {
                score += damage * 2;
                if enemy_board.battlefield.is_empty() {
                    score += 5;
                }
            }
            Effect::Buff { attack, defence, .. } => {
                score += (attack + defence) * 3 / 2;
                if bot_board.battlefield.is_empty() {
                    score -= 5;
                }
            }
            Effect::Draw { draw_amount, .. } => {
                score += *draw_amount as i32 * 3;
            }
            Effect::ReturnToHand { .. } => {
                score += 2;
            }
            Effect::Summon { summon_amount, .. } => {
                score += *summon_amount as i32 * 4;
            }
            Effect::Destroy { target_spec } => {
                score += 15;
                if bot_board.battlefield.is_empty() && matches!(target_spec.side, TargetSide::Friendly) {
                    score -= 20;
                }
            }
            Effect::TakeControl { .. } => {
                if !enemy_board.battlefield.is_empty() {
                    score += 20;
                }
            }
            Effect::Heal { heal, .. } => {
                score += heal;
            }
            // the bot reads no deck order, so flipping it is worth nothing to it
            Effect::ReverseDeck => {}
            Effect::AddRandomOmen => {
                score += 3;
            }
            Effect::ReplayQuickdraws => {
                score += 10;
            }
        }
    }

    if let CardEntity::Minion(m) = card {
        if m.card.attributes.contains(&MinionAttribute::Charge) {
            score += 8;
        }
    }

    // a waiting rite pays off a turn later at the earliest, and only if the trigger comes up
    if matches!(card, CardEntity::Incantation(i) if !i.card.form.armable().is_empty()) {
        score /= 2;
    }

    score
}

fn bot_boards(game: &Game, bot_is_white: bool) -> (&Board, &Board) {
    if bot_is_white {
        (&game.state.white, &game.state.black)
    } else {
        (&game.state.black, &game.state.white)
    }
}

fn compute_attack_target(attacker_id: u32, bot_board: &Board, enemy_board: &Board) -> Option<u32> {
    let attacker = bot_board.battlefield.iter().find(|m| m.entity_id == attacker_id)?;
    let guards: Vec<&MinionEntity> = enemy_board.battlefield.iter()
        .filter(|m| m.card.attributes.contains(&MinionAttribute::Guard))
        .collect();
    let candidates: &[&MinionEntity] = if !guards.is_empty() { &guards } else { &enemy_board.battlefield.iter().collect::<Vec<_>>() };

    if !candidates.is_empty() {
        if let Some(t) = candidates.iter().max_by_key(|e| trade_score(attacker, e)) {
            if trade_score(attacker, t) > 0 || !guards.is_empty() {
                return Some(t.entity_id);
            }
        }
    }
    // nothing worth trading into, so go face, unless it landed too recently to reach the hero
    match can_hit_hero(attacker) {
        true => Some(enemy_board.hero.entity_id),
        false => None,
    }
}

pub fn is_bot_turn(game: &Game) -> bool {
    let s = &game.state;
    (s.white_turn && s.white_player_id == BOT_ID) || (!s.white_turn && s.black_player_id == BOT_ID)
}

pub fn bot_is_white(game: &Game) -> bool {
    game.state.white_player_id == BOT_ID
}

pub async fn make_bot_move(game: &mut Game, bot_is_white: bool, io: &SocketIo) {
    // Phase 1: check for immediate lethal
    let lethal_result: Option<(Vec<u32>, u32)> = {
        let (bot_board, enemy_board) = bot_boards(game, bot_is_white);
        if can_go_lethal(&bot_board.battlefield, enemy_board) {
            let ids = bot_board.battlefield.iter().filter(|m| !m.exhausted && can_hit_hero(m)).map(|m| m.entity_id).collect();
            Some((ids, enemy_board.hero.entity_id))
        } else {
            None
        }
    };
    if let Some((ids, hero_id)) = lethal_result {
        for id in ids {
            game.attack(AttackData { origin_id: id, target_id: hero_id });
        }
        game.end_turn();
        return;
    }

    // Phase 2: play cards in priority order, re-evaluate each iteration
    loop {
        let play_action: Option<(usize, Option<u32>, Option<usize>)> = {
            let (bot_board, enemy_board) = bot_boards(game, bot_is_white);
            let board_full = bot_board.battlefield.len() >= settings::MAX_BOARD_SIZE;
            let rites_full = bot_board.rites.len() >= settings::MAX_RITES;
            bot_board.hand.iter().enumerate()
                .filter(|(_, c)| c.cost() <= bot_board.mana)
                .filter(|(_, c)| !(board_full && matches!(c, CardEntity::Minion(_))))
                .filter(|(_, c)| !(rites_full && matches!(c, CardEntity::Incantation(i) if i.card.form.lingers())))
                .max_by_key(|(_, c)| card_play_score(c, bot_board, enemy_board))
                .map(|(idx, card)| {
                    let trigger = match card {
                        CardEntity::Incantation(i) => match i.card.form.armable() {
                            [] => None,
                            armable => Some(pick_rite_trigger(armable, bot_board, enemy_board)),
                        },
                        CardEntity::Minion(_) => None,
                    };
                    (idx, resolve_card_target(card, bot_board, enemy_board), trigger)
                })
        };

        let (idx, target, rite_trigger) = match play_action {
            Some(a) => a,
            None => break,
        };

        if !game.play_card(idx, target, rite_trigger, None) {
            break;
        }

        broadcast(io, game).await;
        tokio::time::sleep(tokio::time::Duration::from_millis(settings::BOT_DELAY_MS)).await;

        // re-check lethal after each card
        let lethal_result: Option<(Vec<u32>, u32)> = {
            let (bot_board, enemy_board) = bot_boards(game, bot_is_white);
            if can_go_lethal(&bot_board.battlefield, enemy_board) {
                let ids = bot_board.battlefield.iter().filter(|m| !m.exhausted && can_hit_hero(m)).map(|m| m.entity_id).collect();
                Some((ids, enemy_board.hero.entity_id))
            } else {
                None
            }
        };
        if let Some((ids, hero_id)) = lethal_result {
            for id in ids {
                game.attack(AttackData { origin_id: id, target_id: hero_id });
                broadcast(io, game).await;
                tokio::time::sleep(tokio::time::Duration::from_millis(settings::BOT_DELAY_MS)).await;
            }
            game.end_turn();
            return;
        }
    }

    // Phase 3: attack with minions — prefer good trades, go face otherwise
    let attacker_ids: Vec<u32> = {
        let (bot_board, _) = bot_boards(game, bot_is_white);
        bot_board.battlefield.iter().filter(|m| !m.exhausted).map(|m| m.entity_id).collect()
    };

    for attacker_id in attacker_ids {
        let target_id = {
            let (bot_board, enemy_board) = bot_boards(game, bot_is_white);
            compute_attack_target(attacker_id, bot_board, enemy_board)
        };
        if let Some(target_id) = target_id {
            game.attack(AttackData { origin_id: attacker_id, target_id });
            broadcast(io, game).await;
            tokio::time::sleep(tokio::time::Duration::from_millis(settings::BOT_DELAY_MS)).await;
        }
    }

    game.end_turn();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenario::build::{MinionBuild, minion};
    use crate::scenario::{Setup, Side, starting_hp};

    /// Plays a blitz minion so it sits on the board un-exhausted with no turns behind it.
    fn blitzer(attack: i32, prey: Option<MinionCard>) -> (u32, crate::scenario::Scenario) {
        let mut s = Setup::new();
        if let Some(card) = prey {
            s.on_board(Side::Black, card);
        }
        let idx = s.in_hand(Side::White, minion(1, "Hound", 2, attack, 2).with(MinionAttribute::Charge));
        let mut scn = s.start("bot blitz");
        scn.play(idx, None);
        let id = scn.board(Side::White).battlefield[0].entity_id;
        (id, scn)
    }

    #[test]
    fn a_blitz_minion_is_never_aimed_at_the_hero() {
        let (hound, scn) = blitzer(2, None);
        assert_eq!(compute_attack_target(hound, scn.board(Side::White), scn.board(Side::Black)), None);
    }

    #[test]
    fn a_blitz_minion_takes_a_trade_it_wins() {
        let (hound, scn) = blitzer(2, Some(minion(2, "Prey", 1, 1, 2)));
        let prey = scn.board(Side::Black).battlefield[0].entity_id;
        assert_eq!(compute_attack_target(hound, scn.board(Side::White), scn.board(Side::Black)), Some(prey));
    }

    #[test]
    fn lethal_ignores_attack_that_cannot_reach_the_hero() {
        let (_, scn) = blitzer(starting_hp(), None);
        assert!(!can_go_lethal(&scn.board(Side::White).battlefield, scn.board(Side::Black)));
    }

    #[test]
    fn poison_makes_any_body_a_kill_but_a_ward_stops_it() {
        let mut s = Setup::new();
        let rat = s.on_board(Side::White, minion(1, "Rat", 3, 1, 2).with(MinionAttribute::Poisonous));
        let ogre = s.on_board(Side::Black, minion(2, "Ogre", 6, 6, 7));
        let squire = s.on_board(Side::Black, minion(3, "Squire", 1, 1, 1).with(MinionAttribute::Ward));
        let scn = s.start("bot poison");
        let (white, black) = (scn.board(Side::White), scn.board(Side::Black));
        let find = |id: u32, b: &Board| b.battlefield.iter().find(|m| m.entity_id == id).unwrap().clone();
        assert!(would_kill(&find(rat, white), &find(ogre, black)));
        assert!(!would_kill(&find(rat, white), &find(squire, black)));
    }
}
