use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub id: u64,
    pub name: String,
    pub cards: Vec<u32>,
}

const DECKS_FILE: &str = "decks.json";

pub fn load() -> Vec<Deck> {
    match std::fs::read_to_string(DECKS_FILE) {
        Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
        Err(_) => vec![],
    }
}

pub fn save(decks: &[Deck]) {
    if let Ok(s) = serde_json::to_string_pretty(decks) {
        let _ = std::fs::write(DECKS_FILE, s);
    }
}

pub fn now_id() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}

pub fn export_all(decks: &[Deck]) -> String {
    let json = serde_json::to_string(decks).unwrap_or_default();
    STANDARD.encode(json.as_bytes())
}

pub fn export_one(deck: &Deck) -> String {
    let json = serde_json::to_string(&[deck]).unwrap_or_default();
    STANDARD.encode(json.as_bytes())
}

pub fn import_from_base64(b64: &str) -> Result<Vec<Deck>, String> {
    let bytes = STANDARD.decode(b64.trim()).map_err(|e| e.to_string())?;
    let s = String::from_utf8(bytes).map_err(|e| e.to_string())?;
    serde_json::from_str::<Vec<Deck>>(&s).map_err(|e| e.to_string())
}
