use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub id: u64,
    pub name: String,
    pub cards: Vec<u32>,
}

#[cfg(not(target_arch = "wasm32"))]
mod backend {
    use super::Deck;
    use std::time::{SystemTime, UNIX_EPOCH};

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
}

#[cfg(target_arch = "wasm32")]
mod backend {
    use super::Deck;

    const KEY: &str = "pejsesten.decks";

    unsafe extern "C" {
        fn app_storage_get_len(key_ptr: *const u8, key_len: u32) -> i32;
        fn app_storage_get_take(buf_ptr: *mut u8) -> u32;
        fn app_storage_set(key_ptr: *const u8, key_len: u32, val_ptr: *const u8, val_len: u32);
    }

    fn storage_get(key: &str) -> Option<String> {
        let bytes = key.as_bytes();
        let len = unsafe { app_storage_get_len(bytes.as_ptr(), bytes.len() as u32) };
        if len < 0 {
            return None;
        }
        let mut buf = vec![0u8; len as usize];
        let actual = unsafe { app_storage_get_take(buf.as_mut_ptr()) };
        buf.truncate(actual as usize);
        String::from_utf8(buf).ok()
    }

    fn storage_set(key: &str, value: &str) {
        let kb = key.as_bytes();
        let vb = value.as_bytes();
        unsafe {
            app_storage_set(kb.as_ptr(), kb.len() as u32, vb.as_ptr(), vb.len() as u32);
        }
    }

    pub fn load() -> Vec<Deck> {
        match storage_get(KEY) {
            Some(json) => serde_json::from_str(&json).unwrap_or_default(),
            None => vec![],
        }
    }

    pub fn save(decks: &[Deck]) {
        if let Ok(json) = serde_json::to_string(decks) {
            storage_set(KEY, &json);
        }
    }

    pub fn now_id() -> u64 {
        (macroquad::miniquad::date::now() * 1000.0) as u64
    }
}

pub use backend::{load, now_id, save};

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
