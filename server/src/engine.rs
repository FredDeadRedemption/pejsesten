#![allow(unused)]
// engine.rs

use crate::cards;
use crate::settings;
use crate::types::*;
use rand::Rng;
use rand::seq::SliceRandom;

/// Identifies where an entity lives — used instead of references
/// so we can find targets immutably, then mutate separately.
#[derive(Debug, Clone, PartialEq, Eq)]
enum TargetRef {
    HeroSource,
    HeroEnemy,
    MinionSource(usize),
    MinionEnemy(usize),
}

#[derive(Clone, Copy)]
enum PlayerSide {
    White,
    Black,
}

#[derive(Clone)]
struct QueuedEffect {
    effect: Effect,
    owner: PlayerSide,
    target_id: Option<u32>,
    self_id: Option<u32>,
}

pub struct IdGenerator {
    next: u32,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self { next: 0 }
    }
    pub fn next_id(&mut self) -> u32 {
        let id = self.next;
        self.next += 1;
        id
    }
}

pub struct Game {
    pub state: GameStateServer,
    effect_queue: Vec<QueuedEffect>,
    ids: IdGenerator,
}

/// Extracts the TargetSpec from an effect if it has one.
/// Effects like Draw have no target, so they return None.
fn effect_target_spec(effect: &Effect) -> Option<&TargetSpec> {
    match effect {
        Effect::Buff { target_spec, .. } => Some(target_spec),
        Effect::Damage { target_spec, .. } => Some(target_spec),
        Effect::Heal { target_spec, .. } => Some(target_spec),
        Effect::ReturnToHand { target_spec, .. } => Some(target_spec),
        Effect::Destroy { target_spec } => Some(target_spec),
        Effect::Draw { .. } => None,
        Effect::Summon { .. } => None,
    }
}

impl Game {
    pub fn new(
        player1_id: String,
        player2_id: String,
        is_player1_white: bool,
        player1_deck: Vec<CardEntity>,
        player2_deck: Vec<CardEntity>,
        mut ids: IdGenerator,
    ) -> Self {
        let mut rng = rand::rng();

        let (mut white_deck, mut black_deck) = if is_player1_white {
            (player1_deck, player2_deck)
        } else {
            (player2_deck, player1_deck)
        };

        white_deck.shuffle(&mut rng);
        black_deck.shuffle(&mut rng);

        let white_hand: Vec<CardEntity> = white_deck.drain(0..settings::STARTING_HAND_SIZE.min(white_deck.len())).collect();
        let black_hand: Vec<CardEntity> = black_deck.drain(0..(settings::STARTING_HAND_SIZE + 1).min(black_deck.len())).collect();

        let (white_player_id, black_player_id) = if is_player1_white {
            (player1_id, player2_id)
        } else {
            (player2_id, player1_id)
        };

        Game {
            state: GameStateServer {
                white: Board {
                    deck: white_deck,
                    hand: white_hand,
                    graveyard: vec![],
                    battlefield: vec![],
                    hero: Hero {
                        entity_id: ids.next_id(),
                        attack: 0,
                        defence: settings::STARTING_HP,
                    },
                    base_mana: settings::STARTING_MANA,
                    mana: settings::STARTING_MANA,
                },
                black: Board {
                    deck: black_deck,
                    hand: black_hand,
                    graveyard: vec![],
                    battlefield: vec![],
                    hero: Hero {
                        entity_id: ids.next_id(),
                        attack: 0,
                        defence: settings::STARTING_HP,
                    },
                    base_mana: settings::STARTING_MANA - 1,
                    mana: settings::STARTING_MANA - 1,
                },
                white_player_id,
                black_player_id,
                white_turn: true,
                turn_count: 0,
                cards_played_this_turn: 0,
                phase: if settings::MULLIGAN { GamePhase::Mulligan } else { GamePhase::Playing },
                mulligan_white_done: !settings::MULLIGAN,
                mulligan_black_done: !settings::MULLIGAN,
            },
            effect_queue: vec![],
            ids: ids,
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

    fn boards_for(&self, owner: PlayerSide) -> (&Board, &Board) {
        match owner {
            PlayerSide::White => (&self.state.white, &self.state.black),
            PlayerSide::Black => (&self.state.black, &self.state.white),
        }
    }

    fn boards_for_mut(&mut self, owner: PlayerSide) -> (&mut Board, &mut Board) {
        match owner {
            PlayerSide::White => (&mut self.state.white, &mut self.state.black),
            PlayerSide::Black => (&mut self.state.black, &mut self.state.white),
        }
    }

    fn source_board_for(&self, owner: PlayerSide) -> (&Board) {
        match owner {
            PlayerSide::White => (&self.state.white),
            PlayerSide::Black => (&self.state.black),
        }
    }

    fn source_board_for_mut(&mut self, owner: PlayerSide) -> (&mut Board) {
        match owner {
            PlayerSide::White => (&mut self.state.white),
            PlayerSide::Black => (&mut self.state.black),
        }
    }

    fn enemy_board_for(&self, owner: PlayerSide) -> (&Board) {
        match owner {
            PlayerSide::White => (&self.state.black),
            PlayerSide::Black => (&self.state.white),
        }
    }

    fn enemy_board_for_mut(&mut self, owner: PlayerSide) -> (&mut Board) {
        match owner {
            PlayerSide::White => (&mut self.state.black),
            PlayerSide::Black => (&mut self.state.white),
        }
    }

    // --- Entity lookup ---

    fn find_target_ref(&self, id: u32, owner: PlayerSide) -> Option<TargetRef> {
        let (source, enemy) = self.boards_for(owner);
        if source.hero.entity_id == id {
            return Some(TargetRef::HeroSource);
        }
        if enemy.hero.entity_id == id {
            return Some(TargetRef::HeroEnemy);
        }
        if let Some(i) = source.battlefield.iter().position(|m| m.entity_id == id) {
            return Some(TargetRef::MinionSource(i));
        }
        if let Some(i) = enemy.battlefield.iter().position(|m| m.entity_id == id) {
            return Some(TargetRef::MinionEnemy(i));
        }
        None
    }

    // --- Target resolution ---

    fn filters_match(filters: &[Condition], m: &MinionEntity) -> bool {
        filters.iter().all(|f| match f {
            Condition::IsRace { race } => m.card.races.contains(race),
            Condition::HasAttribute { attribute } => m.card.attributes.contains(attribute),
        })
    }

    fn resolve_target_refs(spec: &TargetSpec, source: &Board, enemy: &Board, self_id: Option<u32>) -> Vec<TargetRef> {
        let mut refs = vec![];

        if matches!(spec.entity_type, EntityType::Minion | EntityType::All) {
            if matches!(spec.side, TargetSide::Friendly | TargetSide::All) {
                for (i, m) in source.battlefield.iter().enumerate() {
                    if self_id.map_or(true, |id| m.entity_id != id) && Self::filters_match(&spec.filters, m) {
                        refs.push(TargetRef::MinionSource(i));
                    }
                }
            }
            if matches!(spec.side, TargetSide::Enemy | TargetSide::All) {
                for (i, m) in enemy.battlefield.iter().enumerate() {
                    let hidden = m.stealth_active && spec.target_mode == TargetMode::Targeted;
                    if !hidden && Self::filters_match(&spec.filters, m) {
                        refs.push(TargetRef::MinionEnemy(i));
                    }
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

    fn get_target_refs(&self, spec: &TargetSpec, owner: PlayerSide, target_id: Option<u32>, self_id: Option<u32>) -> Vec<TargetRef> {
        if let Some(id) = target_id {
            let target_ref = self.find_target_ref(id, owner);
            let (source, enemy) = self.boards_for(owner);
            let valid = target_ref.as_ref().map_or(false, |tr| match tr {
                TargetRef::MinionSource(i) => {
                    matches!(spec.side, TargetSide::Friendly | TargetSide::All) && Self::filters_match(&spec.filters, &source.battlefield[*i])
                }
                TargetRef::MinionEnemy(i) => {
                    matches!(spec.side, TargetSide::Enemy | TargetSide::All) && !enemy.battlefield[*i].stealth_active && Self::filters_match(&spec.filters, &enemy.battlefield[*i])
                }
                TargetRef::HeroSource => matches!(spec.side, TargetSide::Friendly | TargetSide::All) && spec.filters.is_empty(),
                TargetRef::HeroEnemy => matches!(spec.side, TargetSide::Enemy | TargetSide::All) && spec.filters.is_empty(),
            });
            return if valid { target_ref.map(|t| vec![t]).unwrap_or_default() } else { vec![] };
        }
        let (source, enemy) = self.boards_for(owner);
        Self::resolve_target_refs(spec, source, enemy, self_id)
    }

    // --- Requirements ---

    fn check_requirements(&self, requirements: &[Requirement], card: &CardEntity) -> bool {
        requirements.iter().all(|r| match r {
            Requirement::Combo => self.state.cards_played_this_turn > 0,
            Requirement::Quickdraw => card.just_drawn(),
            Requirement::IsHolding { filters } => {
                let source_board = self.source_board();
                source_board.hand.iter().any(|c| match c {
                    CardEntity::Minion(m) => Self::filters_match(filters, m),
                    CardEntity::Incantation(_) => false,
                })
            }
        })
    }

    fn check_card_must_match(requirements: &[Condition], card: &CardEntity) -> bool {
        requirements.iter().all(|r| match r {
            Condition::IsRace { race } => match card {
                CardEntity::Minion(m) => m.card.races.contains(race),
                CardEntity::Incantation(_) => false,
            },
            Condition::HasAttribute { attribute } => match card {
                CardEntity::Minion(m) => m.card.attributes.contains(attribute),
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
                let owner = if self.state.white_turn { PlayerSide::White } else { PlayerSide::Black };
                for effect in &ability.effects {
                    to_queue.push(QueuedEffect {
                        effect: effect.clone(),
                        owner: owner,
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
            self.apply_effect(queued.effect, queued.owner, queued.target_id, queued.self_id);
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
                                let owner = if white_is_board { PlayerSide::White } else { PlayerSide::Black };

                                to_queue.push(QueuedEffect {
                                    effect: effect.clone(),
                                    owner: owner,
                                    target_id: None,
                                    self_id: Some(dead.entity_id.clone()),
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

    fn apply_effect(&mut self, effect: Effect, owner: PlayerSide, target_id: Option<u32>, self_id: Option<u32>) {
        match effect {
            Effect::Buff { target_spec, attack, defence } => {
                let refs = self.get_target_refs(&target_spec, owner, target_id, self_id);
                for tr in refs {
                    let (source, enemy) = self.boards_for_mut(owner);
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
                            source.battlefield[i].max_defence += defence;
                        }
                        TargetRef::MinionEnemy(i) => {
                            enemy.battlefield[i].attack += attack;
                            enemy.battlefield[i].defence += defence;
                            enemy.battlefield[i].max_defence += defence;
                        }
                    }
                }
            }

            Effect::Heal { target_spec, heal } => {
                let refs = self.get_target_refs(&target_spec, owner, target_id, self_id);
                for tr in refs {
                    let (source, enemy) = self.boards_for_mut(owner);
                    match tr {
                        TargetRef::HeroSource => source.hero.defence = (source.hero.defence + heal).min(settings::STARTING_HP),
                        TargetRef::HeroEnemy => enemy.hero.defence = (enemy.hero.defence + heal).min(settings::STARTING_HP),
                        TargetRef::MinionSource(i) => {
                            let m = &mut source.battlefield[i];
                            m.defence = (m.defence + heal).min(m.max_defence);
                        }
                        TargetRef::MinionEnemy(i) => {
                            let m = &mut enemy.battlefield[i];
                            m.defence = (m.defence + heal).min(m.max_defence);
                        }
                    }
                }
            }

            Effect::Damage { target_spec, damage, lifesteal } => {
                let refs = self.get_target_refs(&target_spec, owner, target_id, self_id);
                let hit_count = refs.len() as i32;
                for tr in refs {
                    let (source, enemy) = self.boards_for_mut(owner);
                    match tr {
                        TargetRef::HeroSource => source.hero.defence -= damage,
                        TargetRef::HeroEnemy => enemy.hero.defence -= damage,
                        TargetRef::MinionSource(i) => {
                            let m = &mut source.battlefield[i];
                            if m.ward_active { m.ward_active = false; } else { m.defence -= damage; }
                        }
                        TargetRef::MinionEnemy(i) => {
                            let m = &mut enemy.battlefield[i];
                            if m.ward_active { m.ward_active = false; } else { m.defence -= damage; }
                        }
                    }
                }
                if lifesteal {
                    let (source, _) = self.boards_for_mut(owner);
                    source.hero.defence = (source.hero.defence + damage * hit_count).min(settings::STARTING_HP);
                }
            }

            Effect::Draw { draw_amount, follow_up } => {
                // Step 1: draw (needs mut borrow)
                let mut drawn = {
                    let (source, _) = self.boards_for_mut(owner);
                    Self::draw_cards(source, draw_amount)
                };

                // Step 2: follow-up (collect battlefield len while immutably borrowed)
                if let Some(ref fu) = follow_up {
                    let battlefield_len = self.source_board_for(owner).battlefield.len();
                    let mut extra: Vec<CardEntity> = vec![];

                    for card in drawn.iter_mut() {
                        if !Self::check_card_must_match(&fu.card_must_match, card) {
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
                let source_board = self.source_board_for_mut(owner);
                source_board.hand.extend(drawn);
            }

            Effect::ReturnToHand { target_spec, cost_reduction } => {
                let refs = self.get_target_refs(&target_spec, owner, target_id, self_id);

                let (source, enemy) = self.boards_for_mut(owner);

                for tr in refs.into_iter().rev() {
                    let minion = match tr {
                        TargetRef::MinionSource(i) if i < source.battlefield.len() => Some(source.battlefield.remove(i)),
                        TargetRef::MinionEnemy(i) if i < enemy.battlefield.len() => Some(enemy.battlefield.remove(i)),
                        _ => None,
                    };

                    if let Some(mut minion) = minion {
                        if let Some(reduction) = cost_reduction {
                            minion.cost = (minion.card.base_cost - reduction).max(0);
                        }

                        minion.exhausted = false;
                        minion.turns_on_board = 0;
                        minion.turns_in_hand = 0;
                        minion.attack = minion.card.base_attack;
                        minion.defence = minion.card.base_defence;
                        minion.max_defence = minion.card.base_defence;
                        minion.ward_active = minion.card.attributes.contains(&MinionAttribute::Ward);
                        minion.stealth_active = minion.card.attributes.contains(&MinionAttribute::Stealth);

                        match tr {
                            TargetRef::MinionSource(_) => {
                                source.hand.push(CardEntity::Minion(minion));
                            }
                            TargetRef::MinionEnemy(_) => {
                                enemy.hand.push(CardEntity::Minion(minion));
                            }
                            _ => {}
                        }
                    }
                }
            }

            Effect::Destroy { target_spec } => {
                let refs = self.get_target_refs(&target_spec, owner, target_id, self_id);

                let (source, enemy) = self.boards_for_mut(owner);

                for tr in refs {
                    match tr {
                        TargetRef::MinionSource(i) => source.battlefield[i].defence = 0,
                        TargetRef::MinionEnemy(i) => enemy.battlefield[i].defence = 0,
                        _ => {}
                    }
                }
            }

            Effect::Summon { minion_card_id, summon_amount } => {
                let mut new_ids = Vec::with_capacity(summon_amount as usize);
                for _ in 0..summon_amount {
                    new_ids.push(self.ids.next_id());
                }

                let (source_board, _) = self.boards_for_mut(owner);

                for id in new_ids {
                    if let Some(minion) = cards::instantiate_minion_by_id(minion_card_id, id) {
                        source_board.battlefield.push(minion);
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
            source_board.base_mana = (source_board.base_mana + 1).min(settings::MAX_MANA);
            source_board.mana = source_board.base_mana;
            for minion in source_board.battlefield.iter_mut() {
                minion.exhausted = false;
                minion.turns_on_board += 1;
            }
        }

        self.state.cards_played_this_turn = 0;
        //self.enqueue_trigger(Trigger::OnDraw);
        //self.process_effect_queue();
    }

    pub fn play_card(&mut self, index: usize, target_id: Option<u32>) -> bool {
        // Validate without consuming — clone card so mutable borrow can be released before check_requirements
        let card_clone = {
            let source_board = self.source_board_mut();
            let card = match source_board.hand.get(index) {
                Some(c) => c,
                None => return false,
            };
            if card.cost() > source_board.mana {
                return false;
            }
            card.clone()
        };
        let is_minion = matches!(card_clone, CardEntity::Minion(_));
        let owner = if self.state.white_turn { PlayerSide::White } else { PlayerSide::Black };

        let targeted_specs: Vec<TargetSpec> = card_clone.abilities().iter()
            .filter(|a| self.check_requirements(&a.requirements, &card_clone))
            .flat_map(|a| a.effects.iter())
            .filter_map(|e| effect_target_spec(e).cloned())
            .filter(|ts| ts.target_mode == TargetMode::Targeted)
            .collect();

        if !targeted_specs.is_empty() {
            match target_id {
                Some(tid) => {
                    // Reject if the provided target doesn't match any spec (e.g. non-beast for Houndmaster)
                    let valid = targeted_specs.iter().any(|spec| {
                        !self.get_target_refs(spec, owner, Some(tid), None).is_empty()
                    });
                    if !valid {
                        return false;
                    }
                }
                None => {
                    if !is_minion {
                        return false;
                    }
                    // Minion with no target provided: only allow if there are no valid targets to pick from
                    let has_valid_targets = targeted_specs.iter().any(|spec| {
                        !self.get_target_refs(spec, owner, None, None).is_empty()
                    });
                    if has_valid_targets {
                        return false;
                    }
                }
            }
        }

        // Consume from hand
        let source_board = self.source_board_mut();
        let consumed = source_board.hand.remove(index);
        let entity_id = consumed.entity_id();
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
                let tid = effect_target_spec(effect)
                    .filter(|ts| ts.target_mode == TargetMode::Targeted)
                    .and_then(|_| target_id.clone());

                let owner = if self.state.white_turn { PlayerSide::White } else { PlayerSide::Black };
                self.effect_queue.push(QueuedEffect {
                    effect: effect.clone(),
                    owner: owner,
                    target_id: tid,
                    self_id: Some(entity_id),
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
        let owner = if self.state.white_turn { PlayerSide::White } else { PlayerSide::Black };
        let attacker_idx = match self.find_target_ref(data.origin_id, owner) {
            Some(TargetRef::MinionSource(i)) => i,
            _ => return false,
        };

        if self.source_board().battlefield[attacker_idx].exhausted {
            return false;
        }

        if self.source_board().battlefield[attacker_idx].attack <= 0 {
            return false;
        }

        let target_ref = match self.find_target_ref(data.target_id, owner) {
            Some(t) => t,
            None => return false,
        };

        if (target_ref == TargetRef::HeroEnemy) && self.source_board().battlefield[attacker_idx].turns_on_board == 0 {
            return false; // can't attack hero on the turn a minion is summoned (rush)
        }

        if let TargetRef::MinionEnemy(i) = &target_ref {
            if self.enemy_board().battlefield[*i].stealth_active {
                return false;
            }
        }

        let attacker_has_stealth = self.source_board().battlefield[attacker_idx].stealth_active;
        let enemy_has_guard = self
            .enemy_board()
            .battlefield
            .iter()
            .any(|m| !m.stealth_active && m.card.attributes.contains(&MinionAttribute::Guard));
        if !attacker_has_stealth && enemy_has_guard {
            let target_is_guard = match &target_ref {
                TargetRef::MinionEnemy(i) => self.enemy_board().battlefield[*i].card.attributes.contains(&MinionAttribute::Guard),
                _ => false,
            };
            if !target_is_guard {
                return false;
            }
        }

        let attacker_attack = self.source_board().battlefield[attacker_idx].attack;

        {
            let (source, enemy) = self.boards_mut();
            source.battlefield[attacker_idx].exhausted = true;
            source.battlefield[attacker_idx].stealth_active = false;

            match &target_ref {
                TargetRef::HeroEnemy => enemy.hero.defence -= attacker_attack,
                TargetRef::HeroSource => source.hero.defence -= attacker_attack,
                TargetRef::MinionEnemy(i) => {
                    let target_attack = enemy.battlefield[*i].attack;
                    let target = &mut enemy.battlefield[*i];
                    if target.ward_active { target.ward_active = false; } else { target.defence -= attacker_attack; }
                    let attacker = &mut source.battlefield[attacker_idx];
                    if attacker.ward_active { attacker.ward_active = false; } else { attacker.defence -= target_attack; }
                }
                TargetRef::MinionSource(i) => {
                    let target_attack = source.battlefield[*i].attack;
                    let attacker_attack_for_target = attacker_attack;
                    if *i == attacker_idx {
                        let m = &mut source.battlefield[*i];
                        if m.ward_active { m.ward_active = false; } else { m.defence -= attacker_attack_for_target; }
                    } else {
                        let target = &mut source.battlefield[*i];
                        if target.ward_active { target.ward_active = false; } else { target.defence -= attacker_attack_for_target; }
                        let attacker = &mut source.battlefield[attacker_idx];
                        if attacker.ward_active { attacker.ward_active = false; } else { attacker.defence -= target_attack; }
                    }
                }
            }

            if source.battlefield[attacker_idx].card.attributes.contains(&MinionAttribute::Lifesteal) {
                source.hero.defence = (source.hero.defence + attacker_attack).min(settings::STARTING_HP);
            }

            let attacker_is_poisonous = source.battlefield[attacker_idx].card.attributes.contains(&MinionAttribute::Poisonous);
            if attacker_is_poisonous {
                match &target_ref {
                    TargetRef::MinionEnemy(i) => enemy.battlefield[*i].defence = 0,
                    TargetRef::MinionSource(i) => source.battlefield[*i].defence = 0,
                    _ => {}
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
            phase: self.state.phase.clone(),
            mulligan_submitted: if for_white { self.state.mulligan_white_done } else { self.state.mulligan_black_done },
        }
    }

    pub fn submit_mulligan(&mut self, player_id: &str, mut indices: Vec<usize>) -> bool {
        if self.state.phase != GamePhase::Mulligan {
            return false;
        }

        let is_white = player_id == self.state.white_player_id;
        let is_black = player_id == self.state.black_player_id;

        if !is_white && !is_black {
            return false;
        }

        if is_white && self.state.mulligan_white_done { return false; }
        if is_black && self.state.mulligan_black_done { return false; }

        let board = if is_white { &mut self.state.white } else { &mut self.state.black };

        // Sort descending so removals don't shift remaining indices
        indices.sort_unstable_by(|a, b| b.cmp(a));
        indices.dedup();
        indices.retain(|&i| i < board.hand.len());

        let cards_to_return: Vec<CardEntity> = indices.iter()
            .map(|&i| board.hand.remove(i))
            .collect();

        // Draw replacements first so they can't contain the returned cards
        let draw_count = cards_to_return.len();
        let new_cards: Vec<CardEntity> = board.deck.drain(0..draw_count.min(board.deck.len())).collect();
        board.hand.extend(new_cards);

        // Shuffle returned cards back into the deck
        let mut rng = rand::rng();
        board.deck.extend(cards_to_return);
        board.deck.shuffle(&mut rng);

        if is_white { self.state.mulligan_white_done = true; }
        else { self.state.mulligan_black_done = true; }

        if self.state.mulligan_white_done && self.state.mulligan_black_done {
            self.state.phase = GamePhase::Playing;
        }

        true
    }
}
