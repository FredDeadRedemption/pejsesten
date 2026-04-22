use crate::engine::Game;
use crate::settings;
use crate::types::*;
use socketioxide::SocketIo;

async fn broadcast(io: &SocketIo, game: &Game) {
    let white = game.parse_client_state(true);
    let black = game.parse_client_state(false);
    io.to(game.state.white_player_id.clone()).emit("newGameState", &white).await.ok();
    io.to(game.state.black_player_id.clone()).emit("newGameState", &black).await.ok();
}

pub const BOT_ID: &str = "bot";

pub fn default_deck() -> Vec<u32> {
    vec![
        1, 1,   // Machine Elf (1 cost, charge)
        6, 6,   // Black Cat (1 cost, tradeable)
        7, 7,   // Barry the Hexblade (2 cost, buff all)
        2, 2,   // Overzealous Priest (3 cost, draw)
        3, 3,   // Silverguard Knight (3 cost, 5/3, deathrattle draw)
        4, 4,   // Radiant Sentinel (4 cost, deal 2 to all minions)
        12, 12, // Good Friday (3 cost, deal 4 targeted)
        15, 15, // Smite (2 cost, deal 2/4 targeted)
        11, 11, // Divine Power (4 cost, +4+4 targeted)
        13, 13, // Pot of Greed (3 cost, draw 1/2 with combo)
    ]
}

fn threat_score(m: &MinionEntity) -> i32 {
    m.attack * 2 + m.defence
}

fn is_trade_lethal(attacker: &MinionEntity, defender: &MinionEntity) -> bool {
    defender.attack >= attacker.defence
}

fn would_kill(attacker: &MinionEntity, defender: &MinionEntity) -> bool {
    attacker.attack >= defender.defence
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
    minions.iter().filter(|m| !m.exhausted).map(|m| m.attack).sum::<i32>() >= enemy_board.hero.defence
}

fn filter_minions<'a>(minions: &'a [MinionEntity], filters: &[TargetFilter]) -> Vec<&'a MinionEntity> {
    minions.iter().filter(|m| filters.iter().all(|f| match f {
        TargetFilter::IsRace { race } => m.card.races.contains(race),
        TargetFilter::HasAttribute { attribute } => m.card.attributes.contains(attribute),
    })).collect()
}

fn pick_best_spell_target(bot_board: &Board, enemy_board: &Board, effect: &Effect) -> Option<u32> {
    match effect {
        Effect::Damage { target_spec, .. } => {
            let enemies = filter_minions(&enemy_board.battlefield, &target_spec.filters);
            if matches!(target_spec.side, TargetSide::Enemy | TargetSide::All) && !enemies.is_empty() {
                let killable: Vec<&&MinionEntity> = enemies.iter().filter(|m| m.defence <= 4).collect();
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
            friendly.iter().max_by_key(|m| m.attack).map(|m| m.entity_id)
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

fn card_play_score(card: &CardEntity, bot_board: &Board, enemy_board: &Board) -> i32 {
    let mut score = -card.cost();

    for ability in card.abilities() {
        if ability.trigger != Trigger::OnPlay {
            continue;
        }
        for effect in &ability.effects {
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
                Effect::Heal { heal, .. } => {
                    score += heal;
                }
            }
        }
    }

    if let CardEntity::Minion(m) = card {
        if m.card.attributes.contains(&MinionAttribute::Charge) {
            score += 8;
        }
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
        match candidates.iter().max_by_key(|e| trade_score(attacker, e)) {
            Some(t) if trade_score(attacker, t) > 0 || !guards.is_empty() => Some(t.entity_id),
            _ => Some(enemy_board.hero.entity_id),
        }
    } else {
        Some(enemy_board.hero.entity_id)
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
            let ids = bot_board.battlefield.iter().filter(|m| !m.exhausted).map(|m| m.entity_id).collect();
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
        let play_action: Option<(usize, Option<u32>)> = {
            let (bot_board, enemy_board) = bot_boards(game, bot_is_white);
            bot_board.hand.iter().enumerate()
                .filter(|(_, c)| c.cost() <= bot_board.mana)
                .max_by_key(|(_, c)| card_play_score(c, bot_board, enemy_board))
                .map(|(idx, card)| (idx, resolve_card_target(card, bot_board, enemy_board)))
        };

        let (idx, target) = match play_action {
            Some(a) => a,
            None => break,
        };

        if !game.play_card(idx, target) {
            break;
        }

        broadcast(io, game).await;
        tokio::time::sleep(tokio::time::Duration::from_millis(settings::BOT_DELAY_MS)).await;

        // re-check lethal after each card
        let lethal_result: Option<(Vec<u32>, u32)> = {
            let (bot_board, enemy_board) = bot_boards(game, bot_is_white);
            if can_go_lethal(&bot_board.battlefield, enemy_board) {
                let ids = bot_board.battlefield.iter().filter(|m| !m.exhausted).map(|m| m.entity_id).collect();
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
