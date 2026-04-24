use macroquad::prelude::*;

pub const CARD_W: f32 = 110.0;
pub const CARD_H: f32 = 160.0;
pub const CARD_GAP: f32 = 8.0;

pub const MINION_W: f32 = 100.0;
pub const MINION_H: f32 = 145.0;
pub const MINION_GAP: f32 = 8.0;

pub const HERO_W: f32 = 64.0;
pub const HERO_H: f32 = 64.0;

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
