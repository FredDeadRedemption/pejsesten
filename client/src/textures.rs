use macroquad::prelude::*;
use std::collections::HashMap;

use shared::types::{Board, Card, CardEntity, Color as CardColor, GameStateClient};

#[cfg(target_arch = "wasm32")]
const DEFAULT_MEDIA_ROOT: &str = "/media/";
#[cfg(not(target_arch = "wasm32"))]
const DEFAULT_MEDIA_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/media/");

const MEDIA_ROOT: &str = match option_env!("MEDIA_ROOT") {
    Some(r) => r,
    None => DEFAULT_MEDIA_ROOT,
};

pub struct TextureCache {
    map: HashMap<String, Texture2D>,
}

impl TextureCache {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub async fn preload_for_state(&mut self, state: &GameStateClient) {
        // Card backgrounds
        self.load("cards/card-bg-white.png").await;
        self.load("cards/card-bg-black.png").await;
        self.load("cards/missing-texture.png").await;

        // Art from all visible cards
        for board in [&state.self_board, &state.enemy_board] {
            self.load_board(board).await;
        }
    }

    async fn load_board(&mut self, board: &Board) {
        for card in board.hand.iter().chain(board.deck.iter()) {
            self.load_card_art(card).await;
        }
        for minion in board.battlefield.iter().chain(board.graveyard.iter()) {
            self.load_art(&minion.card.image_url).await;
        }
    }

    async fn load_card_art(&mut self, card: &CardEntity) {
        let url = match card {
            CardEntity::Minion(m) => &m.card.image_url,
            CardEntity::Incantation(i) => &i.card.image_url,
        };
        self.load_art(url).await;
    }

    async fn load_art(&mut self, image_url: &str) {
        let key = if image_url.is_empty() {
            "cards/missing-texture.png".to_string()
        } else {
            image_url.to_string()
        };
        self.load(&key).await;
    }

    async fn load(&mut self, key: &str) {
        if self.map.contains_key(key) {
            return;
        }
        let path = format!("{}{}", MEDIA_ROOT, key);
        match load_texture(&path).await {
            Ok(tex) => {
                tex.set_filter(FilterMode::Linear);
                self.map.insert(key.to_string(), tex);
            }
            Err(e) => eprintln!("[textures] failed to load {}: {}", path, e),
        }
    }

    pub async fn preload_cards(&mut self, cards: &[Card]) {
        self.load("cards/card-bg-white.png").await;
        self.load("cards/card-bg-black.png").await;
        self.load("cards/missing-texture.png").await;
        for card in cards {
            let url = match card {
                Card::Minion(m) => m.image_url.as_str(),
                Card::Incantation(i) => i.image_url.as_str(),
            };
            self.load_art(url).await;
        }
    }

    pub fn bg(&self, color: &CardColor) -> Option<&Texture2D> {
        let key = match color {
            CardColor::White => "cards/card-bg-white.png",
            CardColor::Black => "cards/card-bg-black.png",
        };
        self.map.get(key)
    }

    pub fn art(&self, image_url: &str) -> Option<&Texture2D> {
        let key = if image_url.is_empty() {
            "cards/missing-texture.png"
        } else {
            image_url
        };
        self.map.get(key)
    }
}

/// Draw a texture cropped to cover (fill) the destination rect, centered.
pub fn draw_texture_cover(tex: &Texture2D, x: f32, y: f32, w: f32, h: f32, tint: Color) {
    let tex_aspect = tex.width() / tex.height();
    let frame_aspect = w / h;

    let source = if tex_aspect > frame_aspect {
        let src_w = tex.height() * frame_aspect;
        let src_x = (tex.width() - src_w) / 2.0;
        Rect::new(src_x, 0.0, src_w, tex.height())
    } else {
        let src_h = tex.width() / frame_aspect;
        let src_y = (tex.height() - src_h) / 2.0;
        Rect::new(0.0, src_y, tex.width(), src_h)
    };

    draw_texture_ex(
        tex,
        x, y,
        tint,
        DrawTextureParams {
            dest_size: Some(Vec2::new(w, h)),
            source: Some(source),
            ..Default::default()
        },
    );
}
