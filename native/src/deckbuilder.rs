use crate::decks::{self, Deck};
use crate::types::{Card, Color as CardColor};

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
    pub catalog_scroll: f32,
    pub deck_scroll: f32,

    pub decks: Vec<Deck>,
    pub panel: Panel,
    pub editing_cards: Vec<u32>,

    // text input cursor for name
    pub name_cursor: usize,

    // clipboard feedback timer
    pub clipboard_msg: Option<(String, f32)>,
    pub textures_preloaded: bool,
}

impl DeckBuilderState {
    pub fn new(server_url: &str) -> Self {
        let all_cards = fetch_cards(server_url);
        let filtered: Vec<usize> = (0..all_cards.len()).collect();
        let decks = decks::load();
        Self {
            all_cards,
            filtered,
            search: String::new(),
            filter_type: FilterType::All,
            filter_color: ColorFilter::All,
            catalog_scroll: 0.0,
            deck_scroll: 0.0,
            decks,
            panel: Panel::DeckList,
            editing_cards: vec![],
            name_cursor: 0,
            clipboard_msg: None,
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
        self.catalog_scroll = 0.0;
    }

    pub fn open_new_deck(&mut self) {
        let name = random_deck_name();
        self.editing_cards = vec![];
        self.name_cursor = name.len();
        self.panel = Panel::Editor { id: None, name };
        self.deck_scroll = 0.0;
    }

    pub fn open_deck(&mut self, id: u64) {
        if let Some(d) = self.decks.iter().find(|d| d.id == id) {
            let name = d.name.clone();
            self.editing_cards = d.cards.clone();
            self.name_cursor = name.len();
            self.panel = Panel::Editor { id: Some(id), name };
            self.deck_scroll = 0.0;
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
        if self.editing_cards.len() < 50 {
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

fn fetch_cards(server_url: &str) -> Vec<Card> {
    let http_url = server_url
        .replace("ws://", "http://")
        .replace("wss://", "https://");
    let url = format!("{}/cards", http_url);
    match ureq::get(&url).call() {
        Ok(resp) => resp.into_json::<Vec<Card>>().unwrap_or_default(),
        Err(e) => {
            eprintln!("[deckbuilder] failed to fetch cards: {}", e);
            vec![]
        }
    }
}
