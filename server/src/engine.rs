#![allow(unused)]
// engine.rs

use crate::settings;
use rand::Rng;
use rand::RngExt;
use rand::seq::{IndexedRandom, SliceRandom};
use shared::cards;
use shared::cards::{instantiate_incantation, instantiate_minion};
use shared::types::*;
use std::collections::HashMap;

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

pub fn instantiate_omen(c: &OmenCard, entity_id: u32) -> OmenEntity {
    OmenEntity {
        entity_id,
        cost: c.base_cost,
        discount: 0,
        arrived_on_turn: None,
        card: c.clone(),
        armed_trigger: None,
    }
}

pub fn instantiate_hero(c: &HeroCard, entity_id: u32) -> Hero {
    let hp = settings::starting_hp(c);
    Hero {
        entity_id,
        card: c.clone(),
        base_hp: hp,
        damage: 0,
        buff: Buff::default(),
        stats: Stats::new(0, hp),
    }
}

pub fn deck_to_cards(deck: &[u32], ids: &mut IdGenerator) -> Vec<CardEntity> {
    deck.iter()
        .filter_map(|id| cards::get_card_by_id(*id))
        // a hero id in the deck list is not a card to draw, so it is dropped
        .filter_map(|card| match card {
            Card::Minion(c) => Some(CardEntity::Minion(instantiate_minion(c, ids.next_id()))),
            Card::Incantation(c) => Some(CardEntity::Incantation(instantiate_incantation(c, ids.next_id()))),
            Card::Omen(c) => Some(CardEntity::Omen(instantiate_omen(c, ids.next_id()))),
            Card::Hero(_) => None,
        })
        .collect()
}

/// Identifies where an entity lives — used instead of references
/// so we can find targets immutably, then mutate separately.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TargetRef {
    HeroSource,
    HeroEnemy,
    MinionSource(usize),
    MinionEnemy(usize),
}

#[derive(Clone)]
struct QueuedEffect {
    effect: Effect,
    owner: PlayerSide,
    target_id: Option<u32>,
    self_id: Option<u32>,
    // first effect of an ability, so the recorded marker lands in resolution order
    starts_block: bool,
}

pub struct Game {
    pub state: GameStateServer,
    /// Visible changes since the last broadcast, in the order they resolved.
    events: Vec<GameEvent>,
    effect_queue: Vec<QueuedEffect>,
    ids: IdGenerator,
    // omen effects must not arm further omens, or a death cascade chains the whole zone
    omens_firing: bool,
    // summon looks here before the global card registry, so scenarios can summon cards that aren't in cards.rs
    summon_pool: HashMap<u32, MinionCard>,
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
        Effect::ReverseDeck => None,
        Effect::AddRandomOmen => None,
        Effect::ReplayQuickdraws => None,
    }
}

/// Everything one player brings to a game, before the shuffle and the opening deal.
pub struct PlayerSetup {
    pub id: String,
    pub hero: HeroCard,
    pub deck: Vec<CardEntity>,
}

impl Game {
    pub fn new(player1: PlayerSetup, player2: PlayerSetup, is_player1_white: bool, mut ids: IdGenerator) -> Self {
        let mut rng = rand::rng();

        let (mut white, mut black) = if is_player1_white { (player1, player2) } else { (player2, player1) };

        white.deck.shuffle(&mut rng);
        black.deck.shuffle(&mut rng);

        let white_hand: Vec<CardEntity> = white.deck.drain(0..settings::STARTING_HAND_SIZE.min(white.deck.len())).collect();
        let black_hand: Vec<CardEntity> = black.deck.drain(0..(settings::STARTING_HAND_SIZE + 1).min(black.deck.len())).collect();

        Game {
            state: GameStateServer {
                white: Board {
                    deck: white.deck,
                    hand: white_hand,
                    graveyard: vec![],
                    battlefield: vec![],
                    omens: vec![],
                    hero: instantiate_hero(&white.hero, ids.next_id()),
                    base_mana: settings::STARTING_MANA,
                    mana: settings::STARTING_MANA,
                    embers: settings::STARTING_EMBERS,
                    counters: Counters::default(),
                    fatigue: 0,
                    quickdraw_log: vec![],
                },
                black: Board {
                    deck: black.deck,
                    hand: black_hand,
                    graveyard: vec![],
                    battlefield: vec![],
                    omens: vec![],
                    hero: instantiate_hero(&black.hero, ids.next_id()),
                    base_mana: settings::STARTING_MANA - 1,
                    mana: settings::STARTING_MANA - 1,
                    embers: settings::STARTING_EMBERS,
                    counters: Counters::default(),
                    fatigue: 0,
                    quickdraw_log: vec![],
                },
                white_player_id: white.id,
                black_player_id: black.id,
                white_turn: true,
                turn_count: 0,
                cards_played_this_turn: 0,
                arrivals_played_this_turn: 0,
                attacked_this_turn: false,
                phase: if settings::MULLIGAN { GamePhase::Mulligan } else { GamePhase::Playing },
                mulligan_white_done: !settings::MULLIGAN,
                mulligan_black_done: !settings::MULLIGAN,
            },
            events: vec![],
            effect_queue: vec![],
            ids: ids,
            omens_firing: false,
            summon_pool: HashMap::new(),
        }
    }

    /// Builds a game from an exact state. Bypasses the shuffle and opening deal in `new`.
    /// Caller owns id continuity: pass a generator already past the state's entity ids.
    pub fn from_state(state: GameStateServer, ids: IdGenerator) -> Self {
        let mut game = Game { state, events: vec![], effect_queue: vec![], ids, omens_firing: false, summon_pool: HashMap::new() };
        game.refresh_stats();
        game.refresh_costs();
        game
    }

    /// Makes a card summonable by id without it existing in the global registry.
    pub fn register_summonable(&mut self, card: MinionCard) {
        self.summon_pool.insert(card.id, card);
    }

    // --- Events ---

    fn record(&mut self, event: GameEvent) {
        self.events.push(event);
    }

    /// Hands the recorded events to the caller and starts a fresh log.
    pub fn drain_events(&mut self) -> Vec<GameEvent> {
        std::mem::take(&mut self.events)
    }

    /// The entity behind a target ref, so an event can name what an effect actually hit.
    fn entity_id_at(&self, owner: PlayerSide, tr: TargetRef) -> Option<u32> {
        let (source, enemy) = self.boards_for(owner);
        match tr {
            TargetRef::HeroSource => Some(source.hero.entity_id),
            TargetRef::HeroEnemy => Some(enemy.hero.entity_id),
            TargetRef::MinionSource(i) => source.battlefield.get(i).map(|m| m.entity_id),
            TargetRef::MinionEnemy(i) => enemy.battlefield.get(i).map(|m| m.entity_id),
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

    /// A targeted effect with nobody picked lands on one body, never the whole board.
    fn resolve_target_refs(spec: &TargetSpec, source: &Board, enemy: &Board, self_id: Option<u32>) -> Vec<TargetRef> {
        let mut refs = Self::candidate_target_refs(spec, source, enemy, self_id);
        if spec.target_mode == TargetMode::Targeted {
            refs.truncate(1);
        }
        refs
    }

    /// Every target the spec allows, in board order.
    fn candidate_target_refs(spec: &TargetSpec, source: &Board, enemy: &Board, self_id: Option<u32>) -> Vec<TargetRef> {
        let mut refs = vec![];

        if spec.target_mode == TargetMode::SelfOnly {
            let own = self_id.and_then(|id| source.battlefield.iter().position(|m| m.entity_id == id));
            return own.map(|i| vec![TargetRef::MinionSource(i)]).unwrap_or_default();
        }

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

        refs
    }

    fn get_target_refs(&self, spec: &TargetSpec, owner: PlayerSide, target_id: Option<u32>, self_id: Option<u32>) -> Vec<TargetRef> {
        if spec.target_mode == TargetMode::SelfOnly {
            let (source, enemy) = self.boards_for(owner);
            return Self::resolve_target_refs(spec, source, enemy, self_id);
        }
        if let Some(id) = target_id {
            let target_ref = self.find_target_ref(id, owner);
            let (source, enemy) = self.boards_for(owner);
            let hits_minions = matches!(spec.entity_type, EntityType::Minion | EntityType::All);
            let hits_heroes = matches!(spec.entity_type, EntityType::Hero | EntityType::All);
            let valid = target_ref.as_ref().map_or(false, |tr| match tr {
                TargetRef::MinionSource(i) => {
                    hits_minions && matches!(spec.side, TargetSide::Friendly | TargetSide::All) && Self::filters_match(&spec.filters, &source.battlefield[*i])
                }
                TargetRef::MinionEnemy(i) => {
                    hits_minions && matches!(spec.side, TargetSide::Enemy | TargetSide::All) && !enemy.battlefield[*i].stealth_active && Self::filters_match(&spec.filters, &enemy.battlefield[*i])
                }
                TargetRef::HeroSource => hits_heroes && matches!(spec.side, TargetSide::Friendly | TargetSide::All) && spec.filters.is_empty(),
                TargetRef::HeroEnemy => hits_heroes && matches!(spec.side, TargetSide::Enemy | TargetSide::All) && spec.filters.is_empty(),
            });
            return if valid { target_ref.map(|t| vec![t]).unwrap_or_default() } else { vec![] };
        }
        let (source, enemy) = self.boards_for(owner);
        Self::resolve_target_refs(spec, source, enemy, self_id)
    }

    // --- Replay ---

    /// Casts a card again from nowhere: never in hand, never paid for, and its quickdraw is
    /// always live. Targets are rolled because the player never picked any.
    fn replay_card(&mut self, card: &Card, owner: PlayerSide) {
        let entity_id = self.ids.next_id();
        let mut entity = match card {
            Card::Minion(m) => CardEntity::Minion(instantiate_minion(m, entity_id)),
            Card::Incantation(i) => CardEntity::Incantation(instantiate_incantation(i, entity_id)),
            Card::Omen(_) | Card::Hero(_) => return,
        };
        *entity.arrived_on_turn_mut() = Some(self.state.turn_count);
        self.record(GameEvent::CardReplayed { entity_id, card_id: card.id(), white: owner == PlayerSide::White });

        // no room means no body, but the ability still goes off
        if let CardEntity::Minion(minion) = &entity {
            let board = self.source_board_for_mut(owner);
            if board.battlefield.len() < settings::MAX_BOARD_SIZE {
                let mut minion = minion.clone();
                minion.exhausted = !minion.card.attributes.contains(&MinionAttribute::Charge);
                board.battlefield.push(minion);
                self.record(GameEvent::MinionSummoned { entity_id });
            }
        }

        for ability in entity.abilities().to_vec() {
            if ability.trigger != Trigger::OnPlay || !self.check_requirements(&ability.requirements, &entity) {
                continue;
            }
            self.record(GameEvent::TriggerFired { source_id: entity_id });
            for effect in &ability.effects {
                let target_id = effect_target_spec(effect)
                    .filter(|ts| ts.target_mode == TargetMode::Targeted)
                    .and_then(|ts| self.random_target_id(ts, owner));
                self.apply_effect(effect.clone(), owner, target_id, Some(entity_id));
            }
        }
    }

    /// Rolls over every allowed target, not the one body a picked effect would settle on.
    fn random_target_id(&self, spec: &TargetSpec, owner: PlayerSide) -> Option<u32> {
        let (source, enemy) = self.boards_for(owner);
        let refs = Self::candidate_target_refs(spec, source, enemy, None);
        let tr = *refs.choose(&mut rand::rng())?;
        match tr {
            TargetRef::MinionSource(i) => source.battlefield.get(i).map(|m| m.entity_id),
            TargetRef::MinionEnemy(i) => enemy.battlefield.get(i).map(|m| m.entity_id),
            TargetRef::HeroSource => Some(source.hero.entity_id),
            TargetRef::HeroEnemy => Some(enemy.hero.entity_id),
        }
    }

    // --- Requirements ---

    fn check_requirements(&self, requirements: &[Requirement], card: &CardEntity) -> bool {
        requirements.iter().all(|r| match r {
            Requirement::Combo => self.state.cards_played_this_turn > 0,
            Requirement::Quickdraw => card.arrived_this_turn(self.state.turn_count),
            Requirement::CounterAtLeast { counter, amount } => self.source_board().counters.get(*counter) >= *amount,
            Requirement::ArrivalPlayedThisTurn => self.state.arrivals_played_this_turn > 0,
            Requirement::ControlsOther { filters } => {
                let self_id = card.entity_id();
                self.source_board().battlefield.iter().any(|m| m.entity_id != self_id && Self::filters_match(filters, m))
            }
            Requirement::IsHolding { filters } => {
                let source_board = self.source_board();
                source_board.hand.iter().any(|c| match c {
                    CardEntity::Minion(m) => Self::filters_match(filters, m),
                    CardEntity::Incantation(_) | CardEntity::Omen(_) => false,
                })
            }
        })
    }

    /// A passive is live while any friendly minion carrying it is on the board.
    fn has_passive(board: &Board, passive: PassiveEffect) -> bool {
        board.battlefield.iter().any(|m| m.card.passives.contains(&passive))
    }

    /// A follow-up with unmet requirements is dropped, leaving the effect it rides on intact.
    /// Hat Trick style: the quickdraw gate sits on the draw's follow-up rather than on the ability.
    fn follow_up_quickdraw_lands(&self, effect: &Effect, card: &CardEntity) -> bool {
        match effect {
            Effect::Draw { follow_up: Some(fu), .. } => {
                fu.requirements.contains(&Requirement::Quickdraw)
                    && self.check_requirements(&fu.requirements, card)
            }
            _ => false,
        }
    }

    fn resolve_follow_up(&self, effect: &Effect, card: &CardEntity) -> Effect {
        match effect {
            Effect::Draw { draw_amount, follow_up: Some(fu) } if !self.check_requirements(&fu.requirements, card) => {
                Effect::Draw { draw_amount: *draw_amount, follow_up: None }
            }
            other => other.clone(),
        }
    }

    fn check_card_must_match(requirements: &[Condition], card: &CardEntity) -> bool {
        requirements.iter().all(|r| match r {
            Condition::IsRace { race } => match card {
                CardEntity::Minion(m) => m.card.races.contains(race),
                CardEntity::Incantation(_) | CardEntity::Omen(_) => false,
            },
            Condition::HasAttribute { attribute } => match card {
                CardEntity::Minion(m) => m.card.attributes.contains(attribute),
                CardEntity::Incantation(_) | CardEntity::Omen(_) => false,
            },
        })
    }

    // --- Draw ---

    fn draw_cards(board: &mut Board, amount: usize) -> Vec<CardEntity> {
        let n = amount.min(board.deck.len());
        board.deck.drain(0..n).collect()
    }

    /// Cards off the top, and a fatigue hit for every one the deck could not pay for. Each hit
    /// costs one more than the last, so an empty deck is a clock the hero cannot outrun.
    fn draw_for(&mut self, owner: PlayerSide, amount: usize) -> Vec<CardEntity> {
        let (board, _) = self.boards_for_mut(owner);
        let drawn = Self::draw_cards(board, amount);
        for _ in 0..amount - drawn.len() {
            let (board, _) = self.boards_for_mut(owner);
            board.fatigue += 1;
            let hit = board.fatigue;
            self.deal_damage(owner, TargetRef::HeroSource, hit);
        }
        drawn
    }

    // cards that arrive at a full hand are burned
    // `arrival` is None for the pre-game deal, Some(turn) for anything that lands during play
    // returns what landed, so a burned card is never reported as drawn
    fn add_to_hand(board: &mut Board, arrival: Option<u32>, cards: impl IntoIterator<Item = CardEntity>) -> Vec<u32> {
        let mut landed = vec![];
        for mut card in cards {
            if board.hand.len() >= settings::MAX_HAND_SIZE {
                return landed;
            }
            *card.arrived_on_turn_mut() = arrival;
            landed.push(card.entity_id());
            board.hand.push(card);
        }
        landed
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
                for (i, effect) in ability.effects.iter().enumerate() {
                    to_queue.push(QueuedEffect {
                        effect: effect.clone(),
                        owner: owner,
                        target_id: None,
                        self_id: Some(minion.entity_id),
                        starts_block: i == 0,
                    });
                }
            }
        }
        self.effect_queue.extend(to_queue);
    }

    /// Minions that react to an omen resolving, on both boards whoever it belonged to.
    fn fire_omen_watchers(&mut self) {
        for side in [PlayerSide::White, PlayerSide::Black] {
            let reactions: Vec<(u32, Effect, bool)> = self
                .source_board_for(side)
                .battlefield
                .iter()
                .flat_map(|m| {
                    let id = m.entity_id;
                    m.card
                        .abilities
                        .iter()
                        .filter(|a| a.trigger == Trigger::OnOmenFired)
                        .flat_map(move |a| a.effects.iter().enumerate().map(move |(i, e)| (id, e.clone(), i == 0)))
                })
                .collect();

            for (id, effect, starts_block) in reactions {
                if starts_block {
                    self.record(GameEvent::TriggerFired { source_id: id });
                }
                self.apply_effect(effect, side, None, Some(id));
            }
        }
    }

    /// A board can widen from a play or from a deathwish summon, so both paths check it.
    fn fire_wide_board_omens(&mut self, actor: PlayerSide) {
        if self.source_board_for(actor).battlefield.len() >= 3 {
            self.fire_omens(OmenTrigger::EnemyBoardReachesThree, actor.other());
        }
    }

    /// Resolves every armed omen of `owner` watching `trigger`. Omens are consumed when they fire.
    fn fire_omens(&mut self, trigger: OmenTrigger, owner: PlayerSide) {
        if self.omens_firing {
            return;
        }

        let board = self.source_board_for_mut(owner);
        let mut fired: Vec<OmenEntity> = vec![];
        let mut i = 0;
        while i < board.omens.len() {
            if board.omens[i].armed_trigger == Some(trigger) {
                fired.push(board.omens.remove(i));
            } else {
                i += 1;
            }
        }
        if fired.is_empty() {
            return;
        }

        self.omens_firing = true;
        for omen in &fired {
            self.record(GameEvent::OmenFired { entity_id: omen.entity_id });
            for effect in &omen.card.effects {
                self.apply_effect(effect.clone(), owner, None, None);
            }
            self.fire_omen_watchers();
        }
        self.check_for_deaths();
        self.omens_firing = false;
    }

    fn process_effect_queue(&mut self) {
        // drain one at a time so effects enqueued by effects are processed in order
        while !self.effect_queue.is_empty() {
            let queued = self.effect_queue.remove(0);
            if queued.starts_block {
                // a body killed by the last block is gone, and its deathwish has resolved, before
                // the next one reads the board
                self.check_for_deaths();
                if let Some(source_id) = queued.self_id {
                    self.record(GameEvent::TriggerFired { source_id });
                }
            }
            self.apply_effect(queued.effect, queued.owner, queued.target_id, queued.self_id);
        }
    }

    // --- Deaths (flattened loop instead of recursion) ---

    /// The one point where derived stats are refreshed, so every public action that damages,
    /// buffs or moves a minion has to end here.
    fn check_for_deaths(&mut self) {
        let mut died: [bool; 2] = [false, false];
        loop {
            self.refresh_stats();
            let mut any_died = false;
            let mut to_queue: Vec<QueuedEffect> = vec![];
            let mut dead_ids: Vec<u32> = vec![];

            // Check both boards each iteration
            for white_is_board in [true, false] {
                let board = if white_is_board { &mut self.state.white } else { &mut self.state.black };

                let mut i = 0;
                while i < board.battlefield.len() {
                    if board.battlefield[i].stats.defence <= 0 || board.battlefield[i].doomed {
                        any_died = true;
                        died[if white_is_board { 0 } else { 1 }] = true;
                        let dead = board.battlefield.remove(i);
                        dead_ids.push(dead.entity_id);

                        for ability in &dead.card.abilities {
                            if ability.trigger != Trigger::OnDeath {
                                continue;
                            }
                            for (n, effect) in ability.effects.iter().enumerate() {
                                let owner = if white_is_board { PlayerSide::White } else { PlayerSide::Black };

                                to_queue.push(QueuedEffect {
                                    effect: effect.clone(),
                                    owner: owner,
                                    target_id: None,
                                    self_id: Some(dead.entity_id.clone()),
                                    starts_block: n == 0,
                                });
                            }
                        }

                        board.graveyard.push(dead);
                    } else {
                        i += 1;
                    }
                }
            }

            for entity_id in dead_ids {
                self.record(GameEvent::Died { entity_id });
            }
            // in front of whatever is still queued, so a deathwish resolves before the next trigger
            self.effect_queue.splice(0..0, to_queue);
            if !any_died {
                break;
            }
            self.process_effect_queue(); // process death triggers before checking again
        }
        self.refresh_costs();

        for (side, lost) in [(PlayerSide::White, died[0]), (PlayerSide::Black, died[1])] {
            if lost {
                self.fire_omens(OmenTrigger::FriendlyMinionDies, side);
            }
        }
        self.check_for_defeat();
    }

    /// A hero at zero ends the game where it stands. Stats are fresh here, and every public
    /// action ends in the death pass, so no damage can land without passing this.
    fn check_for_defeat(&mut self) {
        if !self.playing() {
            return;
        }
        let down = (self.state.white.hero.stats.defence <= 0, self.state.black.hero.stats.defence <= 0);
        let winner = match down {
            (false, false) => return,
            (true, true) => None,
            (true, false) => Some(PlayerSide::Black),
            (false, true) => Some(PlayerSide::White),
        };
        self.state.phase = GamePhase::Over { winner };
    }

    // --- Stats ---

    /// Damage taken. Every effect that moves numbers goes through here or [`Self::buff_mut`],
    /// so hero and minion cannot drift apart.
    /// Writing either leaves `stats` stale until [`Self::refresh_stats`] runs, so an action that
    /// touches them must end in [`Self::check_for_deaths`] before anything reads the board.
    fn damage_mut(&mut self, owner: PlayerSide, tr: TargetRef) -> &mut i32 {
        let (source, enemy) = self.boards_for_mut(owner);
        match tr {
            TargetRef::HeroSource => &mut source.hero.damage,
            TargetRef::HeroEnemy => &mut enemy.hero.damage,
            TargetRef::MinionSource(i) => &mut source.battlefield[i].damage,
            TargetRef::MinionEnemy(i) => &mut enemy.battlefield[i].damage,
        }
    }

    fn buff_mut(&mut self, owner: PlayerSide, tr: TargetRef) -> &mut Buff {
        let (source, enemy) = self.boards_for_mut(owner);
        match tr {
            TargetRef::HeroSource => &mut source.hero.buff,
            TargetRef::HeroEnemy => &mut enemy.hero.buff,
            TargetRef::MinionSource(i) => &mut source.battlefield[i].buff,
            TargetRef::MinionEnemy(i) => &mut enemy.battlefield[i].buff,
        }
    }

    fn heal(&mut self, owner: PlayerSide, tr: TargetRef, amount: i32) {
        let entity_id = self.entity_id_at(owner, tr);
        // clamped at zero, so what was asked for is not what landed
        let healed = {
            let damage = self.damage_mut(owner, tr);
            let before = *damage;
            *damage = (*damage - amount).max(0);
            before - *damage
        };
        if healed > 0 && let Some(entity_id) = entity_id {
            self.record(GameEvent::Healed { entity_id, amount: healed });
        }
    }

    /// Damage applied where the amount is known, so nothing lands on the board unrecorded.
    /// [`Self::damage_mut`] hands out a bare handle and cannot see the delta.
    fn deal_damage(&mut self, owner: PlayerSide, tr: TargetRef, amount: i32) {
        if amount == 0 {
            return;
        }
        let Some(entity_id) = self.entity_id_at(owner, tr) else { return };
        *self.damage_mut(owner, tr) += amount;
        self.record(GameEvent::Damaged { entity_id, amount });
    }

    /// A hero cannot be destroyed, only brought to zero.
    fn doom(&mut self, owner: PlayerSide, tr: TargetRef) {
        let (source, enemy) = self.boards_for_mut(owner);
        match tr {
            TargetRef::MinionSource(i) => source.battlefield[i].doomed = true,
            TargetRef::MinionEnemy(i) => enemy.battlefield[i].doomed = true,
            TargetRef::HeroSource | TargetRef::HeroEnemy => {}
        }
    }

    /// What a minion gets from the auras on its own board, itself included when it matches.
    fn aura_for(board: &Board, minion: &MinionEntity) -> Buff {
        let mut total = Buff::default();
        for passive in board.battlefield.iter().flat_map(|m| m.card.passives.iter()) {
            if let PassiveEffect::Aura { filters, attack, defence } = passive {
                if Self::filters_match(filters, minion) {
                    total.attack += attack;
                    total.defence += defence;
                }
            }
        }
        total
    }

    /// Rebuilds the derived stats of everything in play. Cards in hand and deck keep the
    /// printed numbers they were minted with.
    /// Called from [`Self::check_for_deaths`], not from the writers, so stats are only correct
    /// between actions; nothing may read `stats` mid-action and expect a fresh number.
    fn refresh_stats(&mut self) {
        for side in [PlayerSide::White, PlayerSide::Black] {
            let board = self.source_board_for(side);
            let auras: Vec<Buff> = board.battlefield.iter().map(|m| Self::aura_for(board, m)).collect();

            let board = self.source_board_for_mut(side);
            for (minion, aura) in board.battlefield.iter_mut().zip(auras) {
                minion.stats.attack = minion.card.base_attack + minion.buff.attack + aura.attack;
                minion.stats.max_defence = minion.card.base_defence + minion.buff.defence + aura.defence;
                minion.stats.defence = minion.stats.max_defence - minion.damage;
            }

            let hero = &mut board.hero;
            hero.stats.attack = hero.buff.attack;
            hero.stats.max_defence = hero.base_hp + hero.buff.defence;
            hero.stats.defence = hero.stats.max_defence - hero.damage;
        }
    }

    /// Hand costs are derived like stats: printed cost, less the discount the card carries,
    /// less whatever its scale is worth right now. Runs wherever the hand or a scale moved.
    fn refresh_costs(&mut self) {
        for side in [PlayerSide::White, PlayerSide::Black] {
            let board = self.source_board_for(side);
            let counters = board.counters.clone();
            let battlefield_len = board.battlefield.len() as i32;

            for card in self.source_board_for_mut(side).hand.iter_mut() {
                let scaled = match card.cost_scaling().cloned() {
                    Some(s) => match s.scaled_by {
                        Some(ScaledBy::MinionsOnBoard) => battlefield_len * s.scalar,
                        Some(ScaledBy::Counter { counter }) => counters.get(counter) as i32 * s.scalar,
                        None => s.scalar,
                    },
                    None => 0,
                };
                let discount = card.discount();
                *card.cost_mut() = (card.base_cost() - discount - scaled).max(0);
            }
        }
    }

    /// Ward eats one hit and breaks. A hero has none, so nothing is absorbed.
    fn absorb_ward(&mut self, owner: PlayerSide, tr: TargetRef) -> bool {
        let entity_id = self.entity_id_at(owner, tr);
        let (source, enemy) = self.boards_for_mut(owner);
        let minion = match tr {
            TargetRef::MinionSource(i) => &mut source.battlefield[i],
            TargetRef::MinionEnemy(i) => &mut enemy.battlefield[i],
            TargetRef::HeroSource | TargetRef::HeroEnemy => return false,
        };
        let absorbed = minion.ward_active;
        minion.ward_active = false;
        if absorbed && let Some(entity_id) = entity_id {
            self.record(GameEvent::WardBroken { entity_id });
        }
        absorbed
    }

    // --- Apply effect ---

    fn apply_effect(&mut self, effect: Effect, owner: PlayerSide, target_id: Option<u32>, self_id: Option<u32>) {
        match effect {
            Effect::Buff { target_spec, attack, defence } => {
                for tr in self.get_target_refs(&target_spec, owner, target_id, self_id) {
                    let entity_id = self.entity_id_at(owner, tr);
                    let buff = self.buff_mut(owner, tr);
                    buff.attack += attack;
                    buff.defence += defence;
                    if let Some(entity_id) = entity_id {
                        self.record(GameEvent::Buffed { entity_id, attack, defence });
                    }
                }
            }

            Effect::Heal { target_spec, heal } => {
                for tr in self.get_target_refs(&target_spec, owner, target_id, self_id) {
                    self.heal(owner, tr, heal);
                }
            }

            Effect::Damage { target_spec, damage, lifesteal } => {
                let refs = self.get_target_refs(&target_spec, owner, target_id, self_id);
                let mut dealt = 0;
                for tr in refs {
                    if self.absorb_ward(owner, tr) {
                        continue;
                    }
                    self.deal_damage(owner, tr, damage);
                    dealt += damage;
                }
                if lifesteal {
                    self.heal(owner, TargetRef::HeroSource, dealt);
                }
            }

            Effect::Draw { draw_amount, follow_up } => {
                // Step 1: draw (needs mut borrow)
                let mut drawn = self.draw_for(owner, draw_amount);

                // Step 2: follow-up (collect battlefield len while immutably borrowed)
                let mut created: Vec<u32> = vec![];
                if let Some(ref fu) = follow_up {
                    let battlefield_len = self.source_board_for(owner).battlefield.len();
                    let counters = self.source_board_for(owner).counters.clone();
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
                                        Some(ScaledBy::Counter { counter }) => counters.get(*counter) as i32 * scaled_amount.scalar,
                                        None => scaled_amount.scalar,
                                    };
                                    *card.discount_mut() += discount;
                                }
                                FollowUpEffect::Copy { copy_amount } => {
                                    for _ in 0..*copy_amount {
                                        let mut copy = card.clone();
                                        // its own entity, or the two move and highlight as one
                                        *copy.entity_id_mut() = self.ids.next_id();
                                        created.push(copy.entity_id());
                                        extra.push(copy);
                                    }
                                }
                            }
                        }
                    }
                    drawn.extend(extra);
                }

                // Step 3: push to hand
                let turn = self.state.turn_count;
                let source_board = self.source_board_for_mut(owner);
                let landed = Self::add_to_hand(source_board, Some(turn), drawn);
                for entity_id in landed {
                    if created.contains(&entity_id) {
                        self.record(GameEvent::CardCreated { entity_id });
                    } else {
                        self.record(GameEvent::CardDrawn { entity_id });
                    }
                }
            }

            Effect::ReturnToHand { target_spec, cost_reduction } => {
                let refs = self.get_target_refs(&target_spec, owner, target_id, self_id);

                let turn = self.state.turn_count;
                let mut returned: Vec<u32> = vec![];
                let (source, enemy) = self.boards_for_mut(owner);

                for tr in refs.into_iter().rev() {
                    let minion = match tr {
                        TargetRef::MinionSource(i) if i < source.battlefield.len() => Some(source.battlefield.remove(i)),
                        TargetRef::MinionEnemy(i) if i < enemy.battlefield.len() => Some(enemy.battlefield.remove(i)),
                        _ => None,
                    };

                    if let Some(mut minion) = minion {
                        if let Some(reduction) = cost_reduction {
                            minion.discount = reduction;
                        }

                        minion.exhausted = false;
                        minion.turns_on_board = 0;
                        minion.damage = 0;
                        minion.buff = Buff::default();
                        minion.doomed = false;
                        minion.stats = Stats::new(minion.card.base_attack, minion.card.base_defence);
                        minion.ward_active = minion.card.attributes.contains(&MinionAttribute::Ward);
                        minion.stealth_active = minion.card.attributes.contains(&MinionAttribute::Stealth);

                        match tr {
                            TargetRef::MinionSource(_) => {
                                returned.extend(Self::add_to_hand(source, Some(turn), [CardEntity::Minion(minion)]));
                            }
                            TargetRef::MinionEnemy(_) => {
                                returned.extend(Self::add_to_hand(enemy, Some(turn), [CardEntity::Minion(minion)]));
                            }
                            _ => {}
                        }
                    }
                }

                for entity_id in returned {
                    self.record(GameEvent::ReturnedToHand { entity_id });
                }
            }

            Effect::Destroy { target_spec } => {
                let refs = self.get_target_refs(&target_spec, owner, target_id, self_id);

                let (source, enemy) = self.boards_for_mut(owner);

                for tr in refs {
                    self.doom(owner, tr);
                }
            }

            Effect::ReverseDeck => {
                let (source, _) = self.boards_for_mut(owner);
                source.deck.reverse();
            }

            Effect::AddRandomOmen => {
                let omens: Vec<OmenCard> = cards::get_collectible_cards()
                    .into_iter()
                    .filter_map(|c| match c {
                        Card::Omen(o) => Some(o),
                        _ => None,
                    })
                    .collect();
                if let Some(pick) = omens.get(rand::rng().random_range(0..omens.len().max(1))) {
                    let omen = instantiate_omen(pick, self.ids.next_id());
                    let turn = self.state.turn_count;
                    let board = self.source_board_for_mut(owner);
                    let landed = Self::add_to_hand(board, Some(turn), [CardEntity::Omen(omen)]);
                    for entity_id in landed {
                        self.record(GameEvent::CardCreated { entity_id });
                    }
                }
            }

            Effect::ReplayQuickdraws => {
                for card in self.source_board_for(owner).quickdraw_log.clone() {
                    self.replay_card(&card, owner);
                }
            }

            Effect::Summon { minion_card_id, summon_amount } => {
                let mut summoned = Vec::with_capacity(summon_amount);
                for _ in 0..summon_amount {
                    let id = self.ids.next_id();
                    let minion = match self.summon_pool.get(&minion_card_id) {
                        Some(c) => Some(instantiate_minion(c, id)),
                        None => cards::instantiate_minion_by_id(minion_card_id, id),
                    };
                    if let Some(m) = minion {
                        summoned.push(m);
                    }
                }

                let mut placed: Vec<u32> = vec![];
                let (source_board, _) = self.boards_for_mut(owner);
                for mut minion in summoned {
                    if source_board.battlefield.len() >= settings::MAX_BOARD_SIZE {
                        break;
                    }
                    minion.exhausted = !minion.card.attributes.contains(&MinionAttribute::Charge);
                    placed.push(minion.entity_id);
                    source_board.battlefield.push(minion);
                }

                for entity_id in placed {
                    self.record(GameEvent::MinionSummoned { entity_id });
                }
            }
        }
    }

    // --- Public API ---

    pub fn end_turn(&mut self) {
        if !self.playing() {
            return;
        }
        let ender = if self.state.white_turn { PlayerSide::White } else { PlayerSide::Black };
        self.fire_omens(OmenTrigger::EnemyEndsTurn, ender.other());
        if !self.state.attacked_this_turn {
            self.fire_omens(OmenTrigger::EnemyEndsTurnWithoutAttacking, ender.other());
        }

        self.record(GameEvent::TurnEnded);

        self.state.turn_count += 1;
        self.state.white_turn = !self.state.white_turn;
        let side = if self.state.white_turn { PlayerSide::White } else { PlayerSide::Black };

        let drawn = self.draw_for(side, 1);
        let landed = {
            let turn = self.state.turn_count;
            let source_board = self.source_board_mut();
            let landed = Self::add_to_hand(source_board, Some(turn), drawn);
            source_board.base_mana = (source_board.base_mana + 1).min(settings::MAX_MANA);
            source_board.mana = source_board.base_mana;
            source_board.embers = (source_board.embers + 1).min(settings::MAX_EMBERS);
            for minion in source_board.battlefield.iter_mut() {
                minion.exhausted = false;
                minion.turns_on_board += 1;
            }
            landed
        };
        for entity_id in landed {
            self.record(GameEvent::CardDrawn { entity_id });
        }

        self.state.cards_played_this_turn = 0;
        self.state.arrivals_played_this_turn = 0;
        self.state.attacked_this_turn = false;
        // fatigue can take the last point of hp, so the turn ends in the death pass like any action
        self.check_for_deaths();
        //self.enqueue_trigger(Trigger::OnDraw);
        //self.process_effect_queue();
    }

    pub fn play_card(&mut self, index: usize, target_id: Option<u32>, omen_trigger: Option<usize>) -> bool {
        if !self.playing() {
            return false;
        }
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
            if matches!(card, CardEntity::Minion(_)) && source_board.battlefield.len() >= settings::MAX_BOARD_SIZE {
                return false;
            }
            if let CardEntity::Omen(o) = card {
                if source_board.omens.len() >= settings::MAX_OMENS {
                    return false;
                }
                // the trigger is picked at play time, so an omen without one is not a legal play
                match omen_trigger {
                    Some(i) if i < o.card.triggers.len() => {}
                    _ => return false,
                }
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
        self.record(GameEvent::CardPlayed { entity_id });

        // Place minion on battlefield
        if let CardEntity::Minion(mut minion) = consumed.clone() {
            minion.exhausted = !minion.card.attributes.contains(&MinionAttribute::Charge);
            let source_board = self.source_board_mut();
            source_board.battlefield.push(minion);
        }

        // Arm omen face down
        if let CardEntity::Omen(mut omen) = consumed.clone() {
            omen.armed_trigger = omen_trigger.map(|i| omen.card.triggers[i]);
            let source_board = self.source_board_mut();
            source_board.omens.push(omen);
        }

        // Enqueue onPlay effects
        let quickdraw_doubled = Self::has_passive(self.source_board(), PassiveEffect::QuickdrawTriggersTwice);
        let mut quickdraw_fired = false;
        for ability in &abilities {
            if ability.trigger != Trigger::OnPlay {
                continue;
            }
            if !self.check_requirements(&ability.requirements, &consumed) {
                continue;
            }
            let is_quickdraw = ability.requirements.contains(&Requirement::Quickdraw);
            // a follow-up gate is still a quickdraw landed, even though only the follow-up doubles
            quickdraw_fired |= is_quickdraw
                || ability.effects.iter().any(|e| self.follow_up_quickdraw_lands(e, &consumed));
            let copies = if is_quickdraw && quickdraw_doubled { 2 } else { 1 };
            for _ in 0..copies {
                for (n, effect) in ability.effects.iter().enumerate() {
                    let effect = self.resolve_follow_up(effect, &consumed);
                    let tid = effect_target_spec(&effect)
                        .filter(|ts| ts.target_mode == TargetMode::Targeted)
                        .and_then(|_| target_id.clone());

                    let owner = if self.state.white_turn { PlayerSide::White } else { PlayerSide::Black };
                    self.effect_queue.push(QueuedEffect {
                        effect,
                        owner: owner,
                        target_id: tid,
                        self_id: Some(entity_id),
                        starts_block: n == 0,
                    });
                }
            }
        }

        // a doubled quickdraw is still one quickdraw played
        if quickdraw_fired {
            let played = consumed.card();
            let board = self.source_board_mut();
            board.counters.quickdraws_played += 1;
            board.quickdraw_log.push(played);
            self.enqueue_trigger(Trigger::OnQuickdraw);
        }

        self.process_effect_queue();
        self.check_for_deaths();

        let source_board = self.source_board_mut();
        source_board.mana -= cost;
        self.state.cards_played_this_turn += 1;
        if consumed.arrived_this_turn(self.state.turn_count) {
            self.state.arrivals_played_this_turn += 1;
        }

        match consumed {
            CardEntity::Minion(_) => {
                self.fire_omens(OmenTrigger::EnemyPlaysMinion, owner.other());
                self.fire_wide_board_omens(owner);
            }
            CardEntity::Incantation(_) => self.fire_omens(OmenTrigger::EnemyPlaysIncantation, owner.other()),
            CardEntity::Omen(_) => {}
        }
        true
    }

    pub fn attack(&mut self, data: AttackData) -> bool {
        if !self.playing() {
            return false;
        }
        let owner = if self.state.white_turn { PlayerSide::White } else { PlayerSide::Black };
        let attacker_idx = match self.find_target_ref(data.origin_id, owner) {
            Some(TargetRef::MinionSource(i)) => i,
            _ => return false,
        };

        if self.source_board().battlefield[attacker_idx].exhausted {
            return false;
        }

        if self.source_board().battlefield[attacker_idx].stats.attack <= 0 {
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

        let attacker_attack = self.source_board().battlefield[attacker_idx].stats.attack;

        let attacker_ref = TargetRef::MinionSource(attacker_idx);
        // only a minion strikes back, and never itself
        let retaliation = match &target_ref {
            TargetRef::MinionEnemy(i) => self.enemy_board().battlefield[*i].stats.attack,
            TargetRef::MinionSource(i) if *i != attacker_idx => self.source_board().battlefield[*i].stats.attack,
            _ => 0,
        };
        let attacker_attributes = &self.source_board().battlefield[attacker_idx].card.attributes;
        let lifesteal = attacker_attributes.contains(&MinionAttribute::Lifesteal);
        let poisonous = attacker_attributes.contains(&MinionAttribute::Poisonous);

        {
            let attacker = &mut self.source_board_mut().battlefield[attacker_idx];
            attacker.exhausted = true;
            attacker.stealth_active = false;
        }

        self.record(GameEvent::Attacked { attacker_id: data.origin_id, target_id: data.target_id });

        let landed = !self.absorb_ward(owner, target_ref);
        if landed {
            self.deal_damage(owner, target_ref, attacker_attack);
        }
        // a harmless defender must not spend the attacker's ward
        if retaliation > 0 && !self.absorb_ward(owner, attacker_ref) {
            self.deal_damage(owner, attacker_ref, retaliation);
        }

        if lifesteal && landed {
            self.heal(owner, TargetRef::HeroSource, attacker_attack);
        }

        if poisonous && landed {
            self.doom(owner, target_ref);
        }

        self.state.attacked_this_turn = true;
        self.check_for_deaths();
        self.fire_wide_board_omens(owner);

        match target_ref {
            TargetRef::HeroEnemy => self.fire_omens(OmenTrigger::EnemyAttacksHero, owner.other()),
            TargetRef::MinionEnemy(_) => self.fire_omens(OmenTrigger::EnemyAttacksMinion, owner.other()),
            _ => {}
        }
        true
    }

    pub fn trade_card(&mut self, index: usize) -> bool {
        if !self.playing() {
            return false;
        }
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

        let turn = self.state.turn_count;
        let source_board = self.source_board_mut();
        let card = source_board.hand.remove(index);
        let traded_id = card.entity_id();
        source_board.deck.push(card);
        let drawn = Self::draw_cards(source_board, 1);
        let landed = Self::add_to_hand(source_board, Some(turn), drawn);
        source_board.mana -= 1;
        self.record(GameEvent::CardTraded { entity_id: traded_id });
        for entity_id in landed {
            self.record(GameEvent::CardDrawn { entity_id });
        }
        self.enqueue_trigger(Trigger::OnTrade);
        self.process_effect_queue();
        self.check_for_deaths();
        self.refresh_costs();
        true
    }

    pub fn playing(&self) -> bool {
        self.state.phase == GamePhase::Playing
    }

    /// A finished game stays around so the result survives a reload; it just refuses every action.
    pub fn is_over(&self) -> bool {
        matches!(self.state.phase, GamePhase::Over { .. })
    }

    pub fn parse_client_state(&self, for_white: bool) -> GameStateClient {
        GameStateClient {
            self_board: if for_white { self.state.white.clone() } else { self.state.black.clone() },
            enemy_board: Self::hide_armed_triggers(if for_white { &self.state.black } else { &self.state.white }),
            white_player_id: self.state.white_player_id.clone(),
            black_player_id: self.state.black_player_id.clone(),
            your_turn: for_white == self.state.white_turn,
            turn_count: self.state.turn_count,
            cards_played_this_turn: self.state.cards_played_this_turn,
            arrivals_played_this_turn: self.state.arrivals_played_this_turn,
            phase: self.state.phase.clone(),
            you_are_white: for_white,
            mulligan_submitted: if for_white { self.state.mulligan_white_done } else { self.state.mulligan_black_done },
            open_cards: settings::OPEN_CARDS,
            hand_hints: self.hand_hints(for_white),
        }
    }

    /// An omen's effect is public but its armed trigger is not.
    fn hide_armed_triggers(board: &Board) -> Board {
        let mut board = board.clone();
        for omen in board.omens.iter_mut() {
            omen.armed_trigger = None;
        }
        board
    }

    /// Play affordances for a hand. Requirements are turn scoped, so off-turn hands get none.
    fn hand_hints(&self, for_white: bool) -> Vec<CardHint> {
        let board = if for_white { &self.state.white } else { &self.state.black };
        if for_white != self.state.white_turn {
            return board.hand.iter().map(|_| CardHint::default()).collect();
        }

        board.hand.iter().map(|card| {
            let has_room = match card {
                CardEntity::Minion(_) => board.battlefield.len() < settings::MAX_BOARD_SIZE,
                CardEntity::Omen(_) => board.omens.len() < settings::MAX_OMENS,
                CardEntity::Incantation(_) => true,
            };
            CardHint {
                playable: card.cost() <= board.mana && has_room,
                condition_met: Self::gated_requirements(card)
                    .any(|reqs| self.check_requirements(reqs, card)),
            }
        }).collect()
    }

    /// Every requirement set a card could satisfy by being played. A follow-up carries its own,
    /// so a card like Hat Trick gates its quickdraw there and not on the ability.
    fn gated_requirements(card: &CardEntity) -> impl Iterator<Item = &Vec<Requirement>> {
        card.abilities().iter().flat_map(|a| {
            let follow_ups = a.effects.iter().filter_map(|e| match e {
                Effect::Draw { follow_up: Some(f), .. } => Some(&f.requirements),
                _ => None,
            });
            std::iter::once(&a.requirements).chain(follow_ups)
        }).filter(|reqs| !reqs.is_empty())
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
        Self::add_to_hand(board, None, new_cards);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenario::build::{
        IncantationBuild, MinionBuild, damage, incantation, minion, on_death, on_play, pick_enemy_minion, replay_quickdraws, requiring, summon,
    };
    use crate::scenario::{Setup, Side};

    /// Three beats, in the order a client has to play them: the lunge, each hit, then the body.
    #[test]
    fn an_attack_records_the_strike_then_both_hits_then_the_death() {
        let mut s = Setup::new();
        let hound = s.on_board(Side::White, minion(1, "Hound", 2, 3, 3));
        let prey = s.on_board(Side::Black, minion(2, "Prey", 2, 2, 2));
        let mut scn = s.start("attack events");

        assert!(scn.attack(hound, prey));
        assert_eq!(
            scn.events(),
            vec![
                GameEvent::Attacked { attacker_id: hound, target_id: prey },
                GameEvent::Damaged { entity_id: prey, amount: 3 },
                GameEvent::Damaged { entity_id: hound, amount: 2 },
                GameEvent::Died { entity_id: prey },
            ]
        );
    }

    /// A ward reports itself breaking and no damage, so the client never animates a hit that
    /// never landed.
    #[test]
    fn a_ward_eating_a_hit_is_recorded_instead_of_the_damage() {
        let mut s = Setup::new();
        let hound = s.on_board(Side::White, minion(1, "Hound", 2, 3, 3));
        let guard = s.on_board(Side::Black, minion(2, "Guard", 2, 1, 1).with(MinionAttribute::Ward));
        let mut scn = s.start("ward events");

        assert!(scn.attack(hound, guard));
        assert_eq!(
            scn.events(),
            vec![
                GameEvent::Attacked { attacker_id: hound, target_id: guard },
                GameEvent::WardBroken { entity_id: guard },
                GameEvent::Damaged { entity_id: hound, amount: 1 },
            ]
        );
    }

    /// The marker is what lets the client read the summon as caused by the death.
    #[test]
    fn a_deathwish_is_marked_as_the_cause_of_what_it_summons() {
        let rat = minion(3, "Rat", 1, 2, 2);
        let mut s = Setup::new();
        s.summonable(rat.clone());
        let hound = s.on_board(Side::White, minion(1, "Hound", 2, 3, 3));
        let cage = s.on_board(Side::Black, minion(2, "Cage", 2, 0, 1).does(on_death(vec![summon(&rat, 1)])));
        let mut scn = s.start("deathwish events");

        assert!(scn.attack(hound, cage));
        let events = scn.events();
        let at = |want: &GameEvent| events.iter().position(|e| e == want);
        let died = at(&GameEvent::Died { entity_id: cage }).expect("cage died");
        let fired = at(&GameEvent::TriggerFired { source_id: cage }).expect("deathwish fired");
        let summoned = events.iter().position(|e| matches!(e, GameEvent::MinionSummoned { .. })).expect("rat summoned");
        assert!(died < fired && fired < summoned, "{events:?}");
    }

    /// A played card leads its own ability, so the card lands before its effect goes off.
    #[test]
    fn a_played_card_comes_before_the_ability_it_fires() {
        let mut s = Setup::new();
        let target = s.on_board(Side::Black, minion(2, "Target", 2, 0, 5));
        let index = s.in_hand(Side::White, minion(1, "Archer", 1, 1, 1).does(on_play(vec![damage(pick_enemy_minion(), 2)])));
        let played = s.hand_entity_id(Side::White, index);
        let mut scn = s.start("play events");

        assert!(scn.play(index, Some(target)));
        assert_eq!(
            scn.events(),
            vec![
                GameEvent::CardPlayed { entity_id: played },
                GameEvent::TriggerFired { source_id: played },
                GameEvent::Damaged { entity_id: target, amount: 2 },
            ]
        );
    }

    /// A replayed card is in no zone, so the event is the only place its card can be named.
    #[test]
    fn a_replayed_card_names_the_card_and_the_side_that_cast_it() {
        let mut s = Setup::new();
        let ogre = s.on_board(Side::Black, minion(1, "Ogre", 0, 3, 7));
        let bolt = incantation(2, "Bolt", 0).does(requiring(on_play(vec![damage(pick_enemy_minion(), 2)]), Requirement::Quickdraw));
        let bolt_card_id = bolt.id;
        s.spell_in_hand(Side::White, bolt);
        s.mark_arrived_this_turn(Side::White, 0);
        s.in_hand(Side::White, minion(3, "Dentist", 0, 7, 7).does(on_play(vec![replay_quickdraws()])));
        let mut scn = s.start("replay events");

        assert!(scn.play(0, Some(ogre)));
        scn.events();
        assert!(scn.play(0, None));

        let events = scn.events();
        let replayed: Vec<&GameEvent> = events.iter().filter(|e| matches!(e, GameEvent::CardReplayed { .. })).collect();
        let [GameEvent::CardReplayed { card_id, white, .. }] = replayed.as_slice() else {
            panic!("one replay, got {replayed:?}");
        };
        assert_eq!((*card_id, *white), (bolt_card_id, true));
    }

    /// Nothing recorded means nothing to animate, which is how a refused move must look.
    #[test]
    fn a_refused_move_records_nothing() {
        let mut s = Setup::new();
        let hound = s.on_board(Side::White, minion(1, "Hound", 2, 3, 3));
        let prey = s.on_board(Side::Black, minion(2, "Prey", 2, 2, 2));
        let mut scn = s.start("refused events");

        assert!(!scn.attack(prey, hound), "black cannot attack on white's turn");
        assert_eq!(scn.events(), vec![]);
    }
}
