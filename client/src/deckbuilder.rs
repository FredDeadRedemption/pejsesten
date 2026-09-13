use crate::clipboard;
use crate::decks::{self, Deck};
use shared::types::{Card, Color as CardColor};

pub const MAX_DECK_CARDS: usize = 50;

pub const RANDOM_NAMES: &[&str] = &[
    "Pixie's Rejected Tarot",
    "Rider-Waite Smackdown",
    "Moonlight Malpractice",
    "DMT Elf Encounter",
    "Demonology for Dummies",
    "Astral Chaos",
    "Bargain Bin Prophecy",
    "Certified Haunted",
];

#[derive(Debug, Clone, PartialEq)]
pub enum FilterType {
    All,
    Minions,
    Incantations,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColorFilter {
    All,
    White,
    Black,
}

pub enum Panel {
    DeckList,
    Editor { id: Option<u64>, name: String },
}

pub struct DeckBuilderState {
    pub all_cards: Vec<Card>,
    pub filtered: Vec<usize>,
    pub search: String,
    pub filter_type: FilterType,
    pub filter_color: ColorFilter,

    pub decks: Vec<Deck>,
    pub panel: Panel,
    pub editing_cards: Vec<u32>,

    // clipboard feedback timer
    pub clipboard_msg: Option<(String, f32)>,
    pub import_pending: bool,
    pub textures_preloaded: bool,
}

impl DeckBuilderState {
    pub fn new() -> Self {
        let all_cards = shared::cards::get_collectible_cards();
        let filtered: Vec<usize> = (0..all_cards.len()).collect();
        let decks = decks::load();
        Self {
            all_cards,
            filtered,
            search: String::new(),
            filter_type: FilterType::All,
            filter_color: ColorFilter::All,
            decks,
            panel: Panel::DeckList,
            editing_cards: vec![],
            clipboard_msg: None,
            import_pending: false,
            textures_preloaded: false,
        }
    }

    pub fn apply_filter(&mut self) {
        let search_lo = self.search.to_lowercase();
        self.filtered = self.all_cards.iter().enumerate().filter_map(|(i, card)| {
            let (name, color, is_minion) = card_meta(card);
            if !search_lo.is_empty() && !name.to_lowercase().contains(&search_lo) {
                return None;
            }
            match &self.filter_type {
                FilterType::Minions if !is_minion => return None,
                FilterType::Incantations if is_minion => return None,
                _ => {}
            }
            match &self.filter_color {
                ColorFilter::White if color != &CardColor::White => return None,
                ColorFilter::Black if color != &CardColor::Black => return None,
                _ => {}
            }
            Some(i)
        }).collect();
    }

    pub fn open_new_deck(&mut self) {
        let name = random_deck_name();
        self.editing_cards = vec![];
        self.panel = Panel::Editor { id: None, name };
    }

    pub fn open_deck(&mut self, id: u64) {
        if let Some(d) = self.decks.iter().find(|d| d.id == id) {
            let name = d.name.clone();
            self.editing_cards = d.cards.clone();
            self.panel = Panel::Editor { id: Some(id), name };
        }
    }

    pub fn save_deck(&mut self) {
        let (id, name) = match &self.panel {
            Panel::Editor { id, name } => (*id, name.clone()),
            _ => return,
        };
        let deck = Deck {
            id: id.unwrap_or_else(decks::now_id),
            name,
            cards: self.editing_cards.clone(),
        };
        let pos = self.decks.iter().position(|d| d.id == deck.id);
        match pos {
            Some(i) => self.decks[i] = deck,
            None => self.decks.push(deck),
        }
        decks::save(&self.decks);
        self.panel = Panel::DeckList;
    }

    pub fn delete_deck(&mut self) {
        if let Panel::Editor { id: Some(id), .. } = &self.panel {
            let id = *id;
            self.decks.retain(|d| d.id != id);
            decks::save(&self.decks);
        }
        self.panel = Panel::DeckList;
    }

    pub fn add_card(&mut self, card_id: u32) {
        if self.editing_cards.len() < MAX_DECK_CARDS {
            self.editing_cards.push(card_id);
        }
    }

    pub fn remove_one(&mut self, card_id: u32) {
        if let Some(pos) = self.editing_cards.iter().position(|&c| c == card_id) {
            self.editing_cards.remove(pos);
        }
    }

    pub fn unique_editing(&self) -> Vec<u32> {
        let mut seen = std::collections::HashSet::new();
        self.editing_cards.iter().filter(|&&id| seen.insert(id)).copied().collect()
    }

    pub fn count_of(&self, card_id: u32) -> usize {
        self.editing_cards.iter().filter(|&&c| c == card_id).count()
    }

    pub fn card_by_id(&self, id: u32) -> Option<&Card> {
        self.all_cards.iter().find(|c| card_id(c) == id)
    }

    pub fn set_clipboard_msg(&mut self, msg: String) {
        self.clipboard_msg = Some((msg, 2.5));
    }

    pub fn tick_clipboard(&mut self, dt: f32) {
        if let Some((_, t)) = &mut self.clipboard_msg {
            *t -= dt;
            if *t <= 0.0 {
                self.clipboard_msg = None;
            }
        }
    }

    /// Copies the deck under edit to the clipboard as the shareable base64 blob.
    pub fn export_current(&mut self) {
        let (id, name) = match &self.panel {
            Panel::Editor { id, name } => (*id, name.clone()),
            Panel::DeckList => return,
        };
        let deck = Deck { id: id.unwrap_or_else(decks::now_id), name, cards: self.editing_cards.clone() };
        clipboard::write(&decks::export_one(&deck));
        self.set_clipboard_msg("Deck copied!".to_string());
    }

    pub fn export_all(&mut self) {
        clipboard::write(&decks::export_all(&self.decks));
        self.set_clipboard_msg("All decks copied!".to_string());
    }

    pub fn request_import(&mut self) {
        clipboard::request_read();
        self.import_pending = true;
    }

    /// Clipboard reads resolve a frame or more later on web, so the result is picked up here.
    pub fn poll_import(&mut self) {
        if !self.import_pending {
            return;
        }
        let Some(result) = clipboard::take_read_result() else {
            return;
        };
        self.import_pending = false;

        match result.and_then(|clip| decks::import_from_base64(&clip)) {
            Ok(imported) => {
                for deck in imported {
                    match self.decks.iter().position(|d| d.id == deck.id) {
                        Some(i) => self.decks[i] = deck,
                        None => self.decks.push(deck),
                    }
                }
                decks::save(&self.decks);
                self.set_clipboard_msg("Decks imported!".to_string());
            }
            Err(e) => {
                eprintln!("[import] failed: {}", e);
                self.set_clipboard_msg("Import failed!".to_string());
            }
        }
    }
}

pub fn card_id(card: &Card) -> u32 {
    match card {
        Card::Minion(m) => m.id,
        Card::Incantation(i) => i.id,
    }
}

pub fn card_meta(card: &Card) -> (&str, &CardColor, bool) {
    match card {
        Card::Minion(m) => (m.name.as_str(), &m.color, true),
        Card::Incantation(i) => (i.name.as_str(), &i.color, false),
    }
}

pub fn card_image_url(card: &Card) -> &str {
    match card {
        Card::Minion(m) => m.image_url.as_str(),
        Card::Incantation(i) => i.image_url.as_str(),
    }
}

pub fn card_cost(card: &Card) -> i32 {
    match card {
        Card::Minion(m) => m.base_cost,
        Card::Incantation(i) => i.base_cost,
    }
}

fn random_deck_name() -> String {
    let idx = (decks::now_id() as usize) % RANDOM_NAMES.len();
    RANDOM_NAMES[idx].to_string()
}

