use macroquad::prelude::*;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use shared::types::{Board, Card, CardEntity, Color as CardColor, GameStateClient};

#[cfg(target_arch = "wasm32")]
const DEFAULT_MEDIA_ROOT: &str = "/media/";
#[cfg(not(target_arch = "wasm32"))]
const DEFAULT_MEDIA_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/static/media/");

const MEDIA_ROOT: &str = match option_env!("MEDIA_ROOT") {
    Some(r) => r,
    None => DEFAULT_MEDIA_ROOT,
};

/// Where the bytes land when the load finishes, whenever that is.
type Slot = Rc<RefCell<Option<miniquad::fs::Response>>>;

pub struct TextureCache {
    map: HashMap<String, Texture2D>,
    queue: VecDeque<(String, FilterMode)>,
    loading: Option<(String, FilterMode, Slot)>,
}

impl TextureCache {
    pub fn new() -> Self {
        Self { map: HashMap::new(), queue: VecDeque::new(), loading: None }
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

    /// Finishes one load and starts the next. Never suspends the frame loop: awaiting
    /// a load costs the frame that was about to be presented, which reads as a black flash.
    pub fn pump(&mut self) {
        if let Some((key, filter, slot)) = self.loading.take() {
            let arrived = slot.borrow_mut().take();
            match arrived {
                Some(response) => self.store(&key, filter, response),
                None => self.loading = Some((key, filter, slot)),
            }
        }

        if self.loading.is_none() {
            if let Some((key, filter)) = self.queue.pop_front() {
                let slot: Slot = Rc::new(RefCell::new(None));
                let sink = slot.clone();
                miniquad::fs::load_file(&format!("{}{}", MEDIA_ROOT, key), move |response| {
                    *sink.borrow_mut() = Some(response);
                });
                self.loading = Some((key, filter, slot));
            }
        }
    }

    fn store(&mut self, key: &str, filter: FilterMode, response: miniquad::fs::Response) {
        let bytes = match response {
            Ok(bytes) => bytes,
            // a miss is permanent, so it is dropped rather than retried every frame
            Err(e) => return log!("[textures] cannot fetch {}: {:?}", key, e),
        };
        match Image::from_file_with_format(&bytes, None) {
            Ok(image) => {
                let texture = Texture2D::from_image(&image);
                texture.set_filter(filter);
                self.map.insert(key.to_string(), texture);
            }
            Err(e) => log!("[textures] cannot decode {}: {}", key, e),
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
