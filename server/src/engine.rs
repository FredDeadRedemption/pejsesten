#![allow(unused)]
// engine.rs

use crate::types::*;
use rand::Rng;
use rand::seq::SliceRandom;

const STARTING_HP: i32 = 30;
const STARTING_MANA: i32 = 1;
const MAX_MANA: i32 = 10;
const STARTING_HAND_SIZE: usize = 3;

/// Identifies where an entity lives — used instead of references
/// so we can find targets immutably, then mutate separately.
#[derive(Debug, Clone)]
enum TargetRef {
    HeroSource,
    HeroEnemy,
    MinionSource(usize),
    MinionEnemy(usize),
}

#[derive(Clone)]
struct QueuedEffect {
    effect: Effect,
    target_id: Option<String>,
    self_id: Option<String>,
}

pub struct Game {
    pub state: GameState,
    effect_queue: Vec<QueuedEffect>,
}

/// Extracts the TargetSpec from an effect if it has one.
/// Effects like Draw have no target, so they return None.
fn effect_target_spec(effect: &Effect) -> Option<&TargetSpec> {
    match effect {
        Effect::Buff { target_spec, .. } => Some(target_spec),
        Effect::Damage { target_spec, .. } => Some(target_spec),
        Effect::ReturnToHand { target_spec, .. } => Some(target_spec),
        Effect::Destroy { target_spec } => Some(target_spec),
        Effect::Draw { .. } => None,
    }
}

impl Game {
    pub fn new(player1_id: String, player2_id: String, is_player1_white: bool, player1_deck: Vec<CardEntity>, player2_deck: Vec<CardEntity>) -> Self {
        let mut rng = rand::rng();

        let (mut white_deck, mut black_deck) = if is_player1_white {
            (player1_deck, player2_deck)
        } else {
            (player2_deck, player1_deck)
        };

        white_deck.shuffle(&mut rng);
        black_deck.shuffle(&mut rng);

        let white_hand: Vec<CardEntity> = white_deck.drain(0..STARTING_HAND_SIZE.min(white_deck.len())).collect();
        let black_hand: Vec<CardEntity> = black_deck.drain(0..(STARTING_HAND_SIZE + 1).min(black_deck.len())).collect();

        let (white_player_id, black_player_id) = if is_player1_white {
            (player1_id, player2_id)
        } else {
            (player2_id, player1_id)
        };

        Game {
            state: GameState {
                white: Board {
                    deck: white_deck,
                    hand: white_hand,
                    graveyard: vec![],
                    battlefield: vec![],
                    hero: Hero {
                        attack: 0,
                        defence: STARTING_HP,
                    },
                    base_mana: STARTING_MANA,
                    mana: STARTING_MANA,
                },
                black: Board {
                    deck: black_deck,
                    hand: black_hand,
                    graveyard: vec![],
                    battlefield: vec![],
                    hero: Hero {
                        attack: 0,
                        defence: STARTING_HP,
                    },
                    base_mana: STARTING_MANA - 1,
                    mana: STARTING_MANA - 1,
                },
                white_player_id,
                black_player_id,
                white_turn: true,
                turn_count: 0,
                cards_played_this_turn: 0,
            },
            effect_queue: vec![],
        }
    }

    // --- Board access ---
    // white and black are separate struct fields so Rust allows split mutable borrows

    fn source_board(&self) -> &Board {
        if self.state.white_turn { &self.state.white } else { &self.state.black }
    }

    fn enemy_board(&self) -> &Board {
        if self.state.white_turn { &self.state.black } else { &self.state.white }
    }

    fn source_board_mut(&mut self) -> &mut Board {
        if self.state.white_turn {
            &mut self.state.white
        } else {
            &mut self.state.black
        }
    }

    fn enemy_board_mut(&mut self) -> &mut Board {
        if self.state.white_turn {
            &mut self.state.black
        } else {
            &mut self.state.white
        }
    }

    fn boards(&self) -> (&Board, &Board) {
        if self.state.white_turn {
            (&self.state.white, &self.state.black)
        } else {
            (&self.state.black, &self.state.white)
        }
    }

    fn boards_mut(&mut self) -> (&mut Board, &mut Board) {
        if self.state.white_turn {
            (&mut self.state.white, &mut self.state.black)
        } else {
            (&mut self.state.black, &mut self.state.white)
        }
    }

    // --- Entity lookup ---

    fn find_target_ref(&self, id: &str) -> Option<TargetRef> {
        match id {
            "heroSelf" => return Some(TargetRef::HeroSource),
            "heroEnemy" => return Some(TargetRef::HeroEnemy),
            _ => {}
        }
        let (source, enemy) = self.boards();
        if let Some(i) = source.battlefield.iter().position(|m| m.base.entity_id == id) {
            return Some(TargetRef::MinionSource(i));
        }
        if let Some(i) = enemy.battlefield.iter().position(|m| m.base.entity_id == id) {
            return Some(TargetRef::MinionEnemy(i));
        }
        None
    }

    // --- Target resolution ---

    fn resolve_target_refs(spec: &TargetSpec, source: &Board, enemy: &Board, self_id: Option<&str>) -> Vec<TargetRef> {
        let mut refs = vec![];

        if matches!(spec.entity_type, EntityType::Minion | EntityType::All) {
            if matches!(spec.side, TargetSide::Friendly | TargetSide::All) {
                for (i, m) in source.battlefield.iter().enumerate() {
                    if self_id.map_or(true, |id| m.base.entity_id != id) {
                        refs.push(TargetRef::MinionSource(i));
                    }
                }
            }
            if matches!(spec.side, TargetSide::Enemy | TargetSide::All) {
                for (i, _) in enemy.battlefield.iter().enumerate() {
                    refs.push(TargetRef::MinionEnemy(i));
                }
            }
        }

        if matches!(spec.entity_type, EntityType::Hero | EntityType::All) {
            if matches!(spec.side, TargetSide::Friendly | TargetSide::All) {
                refs.push(TargetRef::HeroSource);
            }
            if matches!(spec.side, TargetSide::Enemy | TargetSide::All) {
                refs.push(TargetRef::HeroEnemy);
            }
        }

        match spec.target_mode {
            TargetMode::Targeted => refs.truncate(1),
            TargetMode::Auto => {}
        }
        refs
    }

    fn get_target_refs(&self, spec: &TargetSpec, target_id: Option<&str>, self_id: Option<&str>) -> Vec<TargetRef> {
        if let Some(id) = target_id {
            return self.find_target_ref(id).map(|t| vec![t]).unwrap_or_default();
        }
        let (source, enemy) = self.boards();
        Self::resolve_target_refs(spec, source, enemy, self_id)
    }

    // --- Requirements ---

    fn check_requirements(&self, requirements: &[Requirement], card: &CardEntity) -> bool {
        requirements.iter().all(|r| match r {
            Requirement::Combo => self.state.cards_played_this_turn > 0,
            Requirement::Quickdraw => card.just_drawn(),
        })
    }

    fn check_follow_up_requirements(requirements: &[FollowUpRequirement], card: &CardEntity) -> bool {
        requirements.iter().all(|r| match r {
            FollowUpRequirement::IsRace { race } => match card {
                CardEntity::Minion(m) => m.card.races.contains(race),
                CardEntity::Incantation(_) => false,
            },
        })
    }

    // --- Draw ---

    fn draw_cards(board: &mut Board, amount: usize) -> Vec<CardEntity> {
        let n = amount.min(board.deck.len());
        board.deck.drain(0..n).collect()
    }

    // --- Effect queue ---

    fn enqueue_trigger(&mut self, trigger: Trigger) {
        let mut to_queue: Vec<QueuedEffect> = vec![];

        let source_board = self.source_board();
        for minion in &source_board.battlefield {
            for ability in &minion.card.abilities {
                if ability.trigger != trigger {
                    continue;
                }
                for effect in &ability.effects {
                    to_queue.push(QueuedEffect {
                        effect: effect.clone(),
                        target_id: None,
                        self_id: None,
                    });
                }
            }
        }
        self.effect_queue.extend(to_queue);
    }

    fn process_effect_queue(&mut self) {
        // drain one at a time so effects enqueued by effects are processed in order
        while !self.effect_queue.is_empty() {
            let queued = self.effect_queue.remove(0);
            self.apply_effect(queued.effect, queued.target_id.as_deref(), queued.self_id.as_deref());
        }
    }

    // --- Deaths (flattened loop instead of recursion) ---

    fn check_for_deaths(&mut self) {
        loop {
            let mut any_died = false;
            let mut to_queue: Vec<QueuedEffect> = vec![];

            // Check both boards each iteration
            for white_is_board in [true, false] {
                let board = if white_is_board { &mut self.state.white } else { &mut self.state.black };

                let mut i = 0;
                while i < board.battlefield.len() {
                    if board.battlefield[i].defence <= 0 {
                        any_died = true;
                        let dead = board.battlefield.remove(i);

                        for ability in &dead.card.abilities {
                            if ability.trigger != Trigger::OnDeath {
                                continue;
                            }
                            for effect in &ability.effects {
                                to_queue.push(QueuedEffect {
                                    effect: effect.clone(),
                                    target_id: None,
                                    self_id: Some(dead.base.entity_id.clone()),
                                });
                            }
                        }

                        board.graveyard.push(dead);
                    } else {
                        i += 1;
                    }
                }
            }

            self.effect_queue.extend(to_queue);
            if !any_died {
                break;
            }
            self.process_effect_queue(); // process death triggers before checking again
        }
    }

    // --- Apply effect ---

    fn apply_effect(&mut self, effect: Effect, target_id: Option<&str>, self_id: Option<&str>) {
        match effect {
            Effect::Buff { target_spec, attack, defence } => {
                let refs = self.get_target_refs(&target_spec, target_id, self_id);
                for tr in refs {
                    let (source, enemy) = self.boards_mut();
                    match tr {
                        TargetRef::HeroSource => {
                            source.hero.attack += attack;
                            source.hero.defence += defence;
                        }
                        TargetRef::HeroEnemy => {
                            enemy.hero.attack += attack;
                            enemy.hero.defence += defence;
                        }
                        TargetRef::MinionSource(i) => {
                            source.battlefield[i].attack += attack;
                            source.battlefield[i].defence += defence;
                        }
                        TargetRef::MinionEnemy(i) => {
                            enemy.battlefield[i].attack += attack;
                            enemy.battlefield[i].defence += defence;
                        }
                    }
                }
            }

            Effect::Damage { target_spec, damage } => {
                let refs = self.get_target_refs(&target_spec, target_id, self_id);
                for tr in refs {
                    let (source, enemy) = self.boards_mut();
                    match tr {
                        TargetRef::HeroSource => source.hero.defence -= damage,
                        TargetRef::HeroEnemy => enemy.hero.defence -= damage,
                        TargetRef::MinionSource(i) => source.battlefield[i].defence -= damage,
                        TargetRef::MinionEnemy(i) => enemy.battlefield[i].defence -= damage,
                    }
                }
            }

            Effect::Draw { draw_amount, follow_up } => {
                // Step 1: draw (needs mut borrow)
                let mut drawn = {
                    let source_board = self.source_board_mut();
                    Self::draw_cards(source_board, draw_amount)
                };

                // Step 2: follow-up (collect battlefield len while immutably borrowed)
                if let Some(ref fu) = follow_up {
                    let battlefield_len = self.source_board().battlefield.len();
                    let mut extra: Vec<CardEntity> = vec![];

                    for card in drawn.iter_mut() {
                        if !Self::check_follow_up_requirements(&fu.follow_up_requirements, card) {
                            continue;
                        }
                        for fu_effect in &fu.follow_up_effects {
                            match fu_effect {
                                FollowUpEffect::Discount { scaled_amount } => {
                                    let discount = match &scaled_amount.scaled_by {
                                        Some(ScaledBy::MinionsOnBoard) => battlefield_len as i32 * scaled_amount.scalar,
                                        None => scaled_amount.scalar,
                                    };
                                    *card.cost_mut() = (*card.cost_mut() - discount).max(0);
                                }
                                FollowUpEffect::Copy { copy_amount } => {
                                    for _ in 0..*copy_amount {
                                        extra.push(card.clone());
                                    }
                                }
                            }
                        }
                    }
                    drawn.extend(extra);
                }

                // Step 3: push to hand
                let source_board = self.source_board_mut();
                source_board.hand.extend(drawn);
            }

            Effect::ReturnToHand { target_spec, cost_reduction } => {
                let refs = self.get_target_refs(&target_spec, target_id, self_id);
                // Reverse so removing by index doesn't shift remaining indices
                for tr in refs.into_iter().rev() {
                    let is_source = matches!(tr, TargetRef::MinionSource(_));
                    let (source, enemy) = self.boards_mut();

                    let minion = match &tr {
                        TargetRef::MinionSource(i) if *i < source.battlefield.len() => Some(source.battlefield.remove(*i)),
                        TargetRef::MinionEnemy(i) if *i < enemy.battlefield.len() => Some(enemy.battlefield.remove(*i)),
                        _ => None,
                    };

                    if let Some(mut minion) = minion {
                        if let Some(reduction) = cost_reduction {
                            minion.base.cost = (minion.card.base_cost - reduction).max(0);
                        }
                        minion.exhausted = false;
                        minion.attack = minion.card.base_attack;
                        minion.defence = minion.card.base_defence;

                        let (source, enemy) = self.boards_mut();
                        if is_source {
                            source.hand.push(CardEntity::Minion(minion));
                        } else {
                            enemy.hand.push(CardEntity::Minion(minion));
                        }
                    }
                }
            }

            Effect::Destroy { target_spec } => {
                let refs = self.get_target_refs(&target_spec, target_id, self_id);
                for tr in refs {
                    let (source, enemy) = self.boards_mut();
                    match tr {
                        TargetRef::MinionSource(i) => source.battlefield[i].defence = 0,
                        TargetRef::MinionEnemy(i) => enemy.battlefield[i].defence = 0,
                        _ => {}
                    }
                }
            }
        }
    }

    // --- Public API ---

    pub fn end_turn(&mut self) {
        self.state.turn_count += 1;
        self.state.white_turn = !self.state.white_turn;
        let white_is_source = self.state.white_turn;

        {
            let source_board = self.source_board_mut();
            for card in source_board.hand.iter_mut() {
                *card.turns_in_hand_mut() += 1;
                *card.just_drawn_mut() = false;
            }
            let mut drawn = Self::draw_cards(source_board, 1);
            for card in drawn.iter_mut() {
                *card.just_drawn_mut() = true;
            }
            source_board.hand.extend(drawn);
            source_board.base_mana = (source_board.base_mana + 1).min(MAX_MANA);
            source_board.mana = source_board.base_mana;
            for minion in source_board.battlefield.iter_mut() {
                minion.exhausted = false;
            }
        }

        self.state.cards_played_this_turn = 0;
        //self.enqueue_trigger(Trigger::OnDraw);
        //self.process_effect_queue();
    }

    pub fn play_card(&mut self, index: usize, target_id: Option<String>) -> bool {
        // Validate without consuming
        {
            let source_board = self.source_board_mut();
            let card = match source_board.hand.get(index) {
                Some(c) => c,
                None => return false,
            };
            if card.cost() > source_board.mana {
                return false;
            }
            let needs_target = card
    .abilities()
    .iter()
    .any(|a| a.effects.iter().filter_map(effect_target_spec).any(|ts| ts.target_mode == TargetMode::Targeted));
            if needs_target && target_id.is_none() {
                return false;
            }
        }

        // Consume from hand
        let source_board = self.source_board_mut();
        let consumed = source_board.hand.remove(index);
        let entity_id = consumed.entity_id().to_string();
        let cost = consumed.cost();
        let abilities = consumed.abilities().to_vec();

        // Place minion on battlefield
        if let CardEntity::Minion(mut minion) = consumed.clone() {
            minion.exhausted = !minion.card.attributes.contains(&MinionAttribute::Charge);
            let source_board = self.source_board_mut();
            source_board.battlefield.push(minion);
        }

        // Enqueue onPlay effects
        for ability in &abilities {
            if ability.trigger != Trigger::OnPlay {
                continue;
            }
            if !self.check_requirements(&ability.requirements, &consumed) {
                continue;
            }
            for effect in &ability.effects {
                let tid = effect_target_spec(effect).filter(|ts| ts.target_mode == TargetMode::Targeted).and_then(|_| target_id.clone());
                self.effect_queue.push(QueuedEffect {
                    effect: effect.clone(),
                    target_id: tid,
                    self_id: Some(entity_id.clone()),
                });
            }
        }

        self.process_effect_queue();
        self.check_for_deaths();

        let source_board = self.source_board_mut();
        source_board.mana -= cost;
        self.state.cards_played_this_turn += 1;
        true
    }

    pub fn attack(&mut self, data: AttackData) -> bool {
        let attacker_idx = match self.find_target_ref(&data.origin_id) {
            Some(TargetRef::MinionSource(i)) => i,
            _ => return false,
        };

        if self.source_board().battlefield[attacker_idx].exhausted {
            return false;
        }

        let target_ref = match self.find_target_ref(&data.target_id) {
            Some(t) => t,
            None => return false,
        };

        let attacker_attack = self.source_board().battlefield[attacker_idx].attack;

        {
            let (source, enemy) = self.boards_mut();
            source.battlefield[attacker_idx].exhausted = true;

            match &target_ref {
                TargetRef::HeroEnemy => enemy.hero.defence -= attacker_attack,
                TargetRef::HeroSource => source.hero.defence -= attacker_attack,
                TargetRef::MinionEnemy(i) => {
                    let target_attack = enemy.battlefield[*i].attack;
                    enemy.battlefield[*i].defence -= attacker_attack;
                    source.battlefield[attacker_idx].defence -= target_attack;
                }
                TargetRef::MinionSource(i) => {
                    // need to handle case where i == attacker_idx
                    let target_attack = source.battlefield[*i].attack;
                    source.battlefield[*i].defence -= attacker_attack;
                    source.battlefield[attacker_idx].defence -= target_attack;
                }
            }
        }

        self.check_for_deaths();
        true
    }

    pub fn trade_card(&mut self, index: usize) -> bool {
        {
            let source_board = self.source_board_mut();
            if source_board.mana == 0 || source_board.deck.is_empty() {
                return false;
            }
            match source_board.hand.get(index) {
                Some(c) if c.is_tradeable() => {}
                _ => return false,
            }
        }

        let source_board = self.source_board_mut();
        let card = source_board.hand.remove(index);
        source_board.deck.push(card);
        let drawn = Self::draw_cards(source_board, 1);
        source_board.hand.extend(drawn);
        source_board.mana -= 1;
        true
    }

    pub fn parse_client_state(&self, for_white: bool) -> GameStateClient {
        GameStateClient {
            self_board: if for_white { self.state.white.clone() } else { self.state.black.clone() },
            enemy_board: if for_white { self.state.black.clone() } else { self.state.white.clone() },
            white_player_id: self.state.white_player_id.clone(),
            black_player_id: self.state.black_player_id.clone(),
            your_turn: for_white == self.state.white_turn,
            turn_count: self.state.turn_count,
            cards_played_this_turn: self.state.cards_played_this_turn,
        }
    }
}
