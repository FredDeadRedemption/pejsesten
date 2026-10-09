use macroquad::prelude::*;
use std::collections::{HashMap, VecDeque};

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
    queue: VecDeque<(String, FilterMode)>,
}

impl TextureCache {
    pub fn new() -> Self {
        Self { map: HashMap::new(), queue: VecDeque::new() }
    }

    /// Lines up what the state needs. Nothing is awaited: a card with no texture yet
    /// draws as a plain panel, which beats a blank screen while the art is on the wire.
    pub fn queue_for_state(&mut self, state: &GameStateClient) {
        self.queue_frames();
        self.queue("cards/missing-texture.png");
        for board in [&state.self_board, &state.enemy_board] {
            self.queue_board(board);
        }
    }

    pub fn queue_cards(&mut self, cards: &[Card]) {
        self.queue_frames();
        self.queue("cards/missing-texture.png");
        for card in cards {
            self.queue_art(card_image_url(card));
        }
    }

    /// Loads one queued texture. Call it after drawing: the await costs the rest of the frame.
    pub async fn pump(&mut self) {
        let Some((key, filter)) = self.queue.pop_front() else {
            return;
        };
        let path = format!("{}{}", MEDIA_ROOT, key);
        match load_texture(&path).await {
            Ok(tex) => {
                tex.set_filter(filter);
                self.map.insert(key, tex);
            }
            // a miss is permanent, so it is dropped rather than retried every frame
            Err(e) => log!("[textures] failed to load {}: {}", path, e),
        }
    }

    /// Only what is actually painted. The deck is always face down, and on wasm every
    /// miss is a network round trip.
    fn queue_board(&mut self, board: &Board) {
        if !board.hero.card.image_url.is_empty() {
            self.queue(&board.hero.card.image_url);
        }
        for card in &board.hand {
            self.queue_art(card_entity_image_url(card));
        }
        for minion in board.battlefield.iter().chain(board.graveyard.iter()) {
            self.queue_art(&minion.card.image_url);
        }
        for omen in &board.omens {
            self.queue_art(&omen.card.image_url);
        }
    }

    fn queue_art(&mut self, image_url: &str) {
        if !image_url.is_empty() {
            self.queue(image_url);
        }
    }

    fn queue_frames(&mut self) {
        for color in [CardColor::White, CardColor::Black] {
            self.queue_filtered(frame_key(&color), FilterMode::Nearest);
        }
    }

    fn queue(&mut self, key: &str) {
        self.queue_filtered(key, FilterMode::Linear);
    }

    fn queue_filtered(&mut self, key: &str, filter: FilterMode) {
        if self.map.contains_key(key) || self.queue.iter().any(|(k, _)| k == key) {
            return;
        }
        self.queue.push_back((key.to_string(), filter));
    }

    pub fn frame(&self, color: &CardColor) -> Option<&Texture2D> {
        self.map.get(frame_key(color))
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

fn frame_key(color: &CardColor) -> &'static str {
    match color {
        CardColor::White => "cards/card-frame-white.png",
        CardColor::Black => "cards/card-frame-black.png",
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

fn card_entity_image_url(card: &CardEntity) -> &str {
    match card {
        CardEntity::Minion(m) => &m.card.image_url,
        CardEntity::Incantation(i) => &i.card.image_url,
        CardEntity::Omen(o) => &o.card.image_url,
    }
}

fn card_image_url(card: &Card) -> &str {
    match card {
        Card::Minion(m) => &m.image_url,
        Card::Incantation(i) => &i.image_url,
        Card::Omen(o) => &o.image_url,
        Card::Hero(h) => &h.image_url,
    }
}
