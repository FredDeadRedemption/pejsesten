use macroquad::prelude::*;

pub const CARD_W: f32 = 110.0;
pub const CARD_H: f32 = 160.0;
pub const CARD_GAP: f32 = 8.0;

pub const MINION_W: f32 = 100.0;
pub const MINION_H: f32 = 145.0;
pub const MINION_GAP: f32 = 8.0;

pub const HERO_W: f32 = 64.0;
pub const HERO_H: f32 = 64.0;

/// Dead zone — dropping a card here cancels the play. Extends well above the actual cards.
pub const HAND_ZONE_PADDING: f32 = 60.0;

pub fn hand_zone_rect(w: f32, h: f32) -> Rect {
    let zone_h = CARD_H + HAND_ZONE_PADDING;
    Rect::new(0.0, h - zone_h, w, zone_h)
}

pub fn hand_rects(count: usize, w: f32, h: f32) -> Vec<Rect> {
    let y = h - CARD_H - 10.0;
    centered_rects(count, CARD_W, CARD_H, CARD_GAP, w / 2.0, y)
}

pub fn self_minion_rects(count: usize, w: f32, h: f32) -> Vec<Rect> {
    let y = h / 2.0 + 18.0;
    centered_rects(count, MINION_W, MINION_H, MINION_GAP, w / 2.0, y)
}

pub fn enemy_minion_rects(count: usize, w: f32, h: f32) -> Vec<Rect> {
    let y = h / 2.0 - MINION_H - 18.0;
    centered_rects(count, MINION_W, MINION_H, MINION_GAP, w / 2.0, y)
}

pub fn self_hero_rect(w: f32, h: f32) -> Rect {
    Rect::new(w - 100.0, h / 2.0 + 18.0, HERO_W, HERO_H)
}

pub fn enemy_hero_rect(w: f32, h: f32) -> Rect {
    Rect::new(w - 100.0, h / 2.0 - HERO_H - 18.0, HERO_W, HERO_H)
}

pub const MULLIGAN_CARD_W: f32 = 130.0;
pub const MULLIGAN_CARD_H: f32 = 190.0;
pub const MULLIGAN_SPACING: f32 = 18.0;

pub fn mulligan_card_rects(count: usize, w: f32, h: f32) -> Vec<Rect> {
    let total_w = count as f32 * (MULLIGAN_CARD_W + MULLIGAN_SPACING) - MULLIGAN_SPACING;
    let start_x = w / 2.0 - total_w / 2.0;
    let card_y = h / 2.0 - MULLIGAN_CARD_H / 2.0;
    (0..count)
        .map(|i| Rect::new(start_x + i as f32 * (MULLIGAN_CARD_W + MULLIGAN_SPACING), card_y, MULLIGAN_CARD_W, MULLIGAN_CARD_H))
        .collect()
}

pub fn mulligan_confirm_rect(w: f32, h: f32) -> Rect {
    Rect::new(w / 2.0 - 90.0, h - 80.0, 180.0, 44.0)
}

pub fn end_turn_rect(w: f32, h: f32) -> Rect {
    let mid = h / 2.0;
    Rect::new(w - 124.0, mid + 14.0, 104.0, 36.0)
}

pub fn lobby_queue_human_rect(w: f32, h: f32) -> Rect {
    Rect::new(w / 2.0 - 115.0, h / 2.0, 230.0, 46.0)
}

pub fn lobby_queue_bot_rect(w: f32, h: f32) -> Rect {
    Rect::new(w / 2.0 - 115.0, h / 2.0 + 62.0, 230.0, 46.0)
}

pub fn lobby_reset_rect(w: f32, h: f32) -> Rect {
    Rect::new(w / 2.0 - 115.0, h / 2.0 + 130.0, 230.0, 36.0)
}

fn centered_rects(count: usize, rw: f32, rh: f32, gap: f32, cx: f32, y: f32) -> Vec<Rect> {
    if count == 0 {
        return vec![];
    }
    let total = count as f32 * (rw + gap) - gap;
    let start_x = cx - total / 2.0;
    (0..count)
        .map(|i| Rect::new(start_x + i as f32 * (rw + gap), y, rw, rh))
        .collect()
}
