use macroquad::prelude::*;
use std::collections::HashSet;

use crate::deckbuilder::{card_cost, card_image_url, card_meta, DeckBuilderState, Panel};
use crate::layout::{
    self, CARD_GAP, CARD_H, CARD_W, HERO_H, HERO_W, MINION_H, MINION_W,
};
use crate::textures::{draw_texture_cover, TextureCache};
use shared::types::{Board, Card, CardEntity, Color as CardColor, GameStateClient, MinionEntity};

const ART_FRAC: f32 = 0.38;
const TOP_BAR: f32 = 28.0;
const BOTTOM_BAR: f32 = 26.0;

const COL_SELF: Color = Color::new(0.2, 0.5, 0.8, 1.0);
const COL_ENEMY: Color = Color::new(0.7, 0.2, 0.2, 1.0);
const COL_CARD_BG: Color = Color::new(0.15, 0.15, 0.25, 1.0);
const COL_MANA: Color = Color::new(0.34, 0.60, 0.80, 1.0);
const COL_MANA_EMPTY: Color = Color::new(0.12, 0.12, 0.22, 1.0);
const COL_EMBER: Color = Color::new(0.95, 0.45, 0.18, 1.0);
const COL_EMBER_EMPTY: Color = Color::new(0.22, 0.10, 0.06, 1.0);
const EMBER_MAX: i32 = 5;
const COL_DIVIDER: Color = Color::new(0.5, 0.5, 0.5, 0.3);
const COL_ATK: Color = Color::new(0.90, 0.70, 0.20, 1.0);
const COL_DEF: Color = Color::new(0.70, 0.25, 0.25, 1.0);
const COL_DESC_BG: Color = Color::new(0.82, 0.76, 0.67, 0.9);
const COL_TARGET: Color = Color::new(0.0, 1.0, 0.4, 0.55);
const COL_DRAG_SHADOW: Color = Color::new(0.0, 0.0, 0.0, 0.35);

// ── Public entry points ────────────────────────────────────────────────────────

pub struct DragRender<'a> {
    pub card: Option<&'a CardEntity>,   // hand card being dragged
    pub minion_id: Option<u32>,         // battlefield minion being dragged for attack
    pub mx: f32,
    pub my: f32,
    pub targetable_ids: HashSet<u32>,
    pub trade_drop_active: bool,
    pub hovered_minion_id: Option<u32>,
    pub anim_offsets: Vec<(u32, Vec2)>, // (entity_id, pixel offset) for bump anims
    pub hand_flips: Vec<(u32, f32)>,    // (entity_id, cos(rotation_y)) for draw-from-deck flip
}

pub fn draw_game(state: &GameStateClient, cache: &TextureCache, drag: &DragRender) {
    let w = screen_width();
    let h = screen_height();
    let mid = h / 2.0;

    draw_line(0.0, mid, w, mid, 1.5, COL_DIVIDER);
    draw_turn_indicator(state, w, mid);

    draw_board_half(&state.enemy_board, w, h, false, cache, false, &drag.anim_offsets, &drag.hand_flips);
    draw_board_half(&state.self_board, w, h, true, cache, drag.trade_drop_active, &drag.anim_offsets, &drag.hand_flips);

    // Debug: hand zone boundary
    let hz = layout::hand_zone_rect(w, h);
    draw_rectangle_lines(hz.x, hz.y, hz.w, hz.h, 1.0, Color::new(1.0, 1.0, 0.0, 0.4));

    if state.your_turn {
        draw_end_turn_button(w, mid);
        draw_drop_targets(state, drag, w, h);
    }

    if let Some(hid) = drag.hovered_minion_id {
        draw_minion_preview(hid, state, cache, w, h);
    }

    let targeting_active = drag.card.is_some()
        && !drag.targetable_ids.is_empty()
        && !layout::hand_zone_rect(w, h).contains(Vec2::new(drag.mx, drag.my));

    // Dragged card floats above everything — hidden when crosshair takes over
    if let Some(card) = drag.card {
        if !targeting_active {
            let cx = drag.mx - CARD_W / 2.0;
            let cy = drag.my - CARD_H * 0.6;
            draw_rectangle(cx + 4.0, cy + 6.0, CARD_W, CARD_H, COL_DRAG_SHADOW);
            draw_card(card, cx, cy, CARD_W, CARD_H, cache, 1.0);
        }
    }
    if let Some(mid_id) = drag.minion_id {
        draw_attack_arrow(drag.mx, drag.my, mid_id, state, w, h);
    }

    if targeting_active {
        draw_crosshair(drag.mx, drag.my);
    }
}

pub fn draw_mulligan(state: &GameStateClient, selected: &[usize], cache: &TextureCache) {
    let w = screen_width();
    let h = screen_height();

    let title = "Choose cards to replace";
    let dims = measure_text(title, None, 28, 1.0);
    draw_text(title, w / 2.0 - dims.width / 2.0, 60.0, 28.0, WHITE);

    let hand = &state.self_board.hand;
    let full_w = 130.0;
    let full_h = 190.0;
    let spacing = 18.0;
    let total_w = hand.len() as f32 * (full_w + spacing) - spacing;
    let start_x = w / 2.0 - total_w / 2.0;
    let card_y = h / 2.0 - full_h / 2.0;

    for (i, card) in hand.iter().enumerate() {
        let x = start_x + i as f32 * (full_w + spacing);
        let is_selected = selected.contains(&i);
        draw_card(card, x, card_y, full_w, full_h, cache, 1.0);

        if is_selected {
            draw_rectangle(x, card_y, full_w, full_h, Color::new(0.6, 0.0, 0.0, 0.45));
            draw_rectangle_lines(x, card_y, full_w, full_h, 3.0, RED);
            let xd = measure_text("X", None, 36, 1.0);
            draw_text("X", x + full_w / 2.0 - xd.width / 2.0, card_y + full_h / 2.0 + 12.0, 36.0, RED);
        }

        let hint = format!("[{}]", i + 1);
        let hd = measure_text(&hint, None, 14, 1.0);
        draw_text(&hint, x + full_w / 2.0 - hd.width / 2.0, card_y + full_h + 16.0, 14.0, GRAY);
    }

    if !state.mulligan_submitted {
        draw_button("Confirm  [Enter]", w / 2.0 - 90.0, h - 80.0, 180.0, 44.0);
    } else {
        let msg = "Waiting for opponent...";
        let md = measure_text(msg, None, 18, 1.0);
        draw_text(msg, w / 2.0 - md.width / 2.0, h - 60.0, 18.0, GRAY);
    }
}

pub fn draw_lobby(username: &str) {
    let w = screen_width();
    let h = screen_height();

    let title = "pejsesten";
    let td = measure_text(title, None, 52, 1.0);
    draw_text(title, w / 2.0 - td.width / 2.0, h / 3.0, 52.0, WHITE);

    let user_label = format!("Playing as: {}", username);
    let ud = measure_text(&user_label, None, 18, 1.0);
    draw_text(&user_label, w / 2.0 - ud.width / 2.0, h / 3.0 + 44.0, 18.0, LIGHTGRAY);

    draw_button("Play vs Human  [H]", w / 2.0 - 115.0, h / 2.0, 230.0, 46.0);
    draw_button("Play vs Bot    [B]", w / 2.0 - 115.0, h / 2.0 + 62.0, 230.0, 46.0);
    draw_button_danger("Reset Server  [R]", w / 2.0 - 115.0, h / 2.0 + 130.0, 230.0, 36.0);
    draw_button("Deck Builder  [D]", w / 2.0 - 115.0, h / 2.0 + 180.0, 230.0, 46.0);
    draw_button("Card Flip Test  [T]", w / 2.0 - 115.0, h / 2.0 + 242.0, 230.0, 46.0);
}

pub fn draw_connecting() {
    let w = screen_width();
    let h = screen_height();
    let text = "Connecting...";
    let dims = measure_text(text, None, 28, 1.0);
    draw_text(text, w / 2.0 - dims.width / 2.0, h / 2.0, 28.0, GRAY);
}

// ── Drop target highlights ─────────────────────────────────────────────────────

fn draw_drop_targets(state: &GameStateClient, drag: &DragRender, w: f32, h: f32) {
    let ids = &drag.targetable_ids;
    if ids.is_empty() { return; }

    let enemy_rects = layout::enemy_minion_rects(state.enemy_board.battlefield.len(), w, h);
    for (m, r) in state.enemy_board.battlefield.iter().zip(enemy_rects.iter()) {
        if ids.contains(&m.entity_id) {
            draw_rectangle(r.x, r.y, r.w, r.h, COL_TARGET);
            draw_rectangle_lines(r.x, r.y, r.w, r.h, 2.0, GREEN);
        }
    }
    if ids.contains(&state.enemy_board.hero.entity_id) {
        let er = layout::enemy_hero_rect(w, h);
        draw_rectangle(er.x, er.y, er.w, er.h, COL_TARGET);
        draw_rectangle_lines(er.x, er.y, er.w, er.h, 2.0, GREEN);
    }

    let self_rects = layout::self_minion_rects(state.self_board.battlefield.len(), w, h);
    for (m, r) in state.self_board.battlefield.iter().zip(self_rects.iter()) {
        if ids.contains(&m.entity_id) {
            draw_rectangle(r.x, r.y, r.w, r.h, COL_TARGET);
            draw_rectangle_lines(r.x, r.y, r.w, r.h, 2.0, GREEN);
        }
    }
    if ids.contains(&state.self_board.hero.entity_id) {
        let sr = layout::self_hero_rect(w, h);
        draw_rectangle(sr.x, sr.y, sr.w, sr.h, COL_TARGET);
        draw_rectangle_lines(sr.x, sr.y, sr.w, sr.h, 2.0, GREEN);
    }
}

// ── Board halves ──────────────────────────────────────────────────────────────

fn draw_board_half(board: &Board, w: f32, h: f32, is_self: bool, cache: &TextureCache, deck_glow: bool, anim_offsets: &[(u32, Vec2)], hand_flips: &[(u32, f32)]) {
    let mid = h / 2.0;
    if is_self {
        let hand_y = h - CARD_H - 10.0;
        let hero_y = mid + 18.0;
        let mana_y = h - 22.0;
        let deck_r = layout::self_deck_rect(w, h);

        draw_hero(&board.hero, w - 100.0, hero_y, true);
        draw_battlefield(&board.battlefield, w, h, true, cache, anim_offsets);
        draw_hand(&board.hand, w, hand_y, true, cache, hand_flips);
        draw_mana(board.mana, board.base_mana, 20.0, mana_y);
        draw_embers(board.embers, 20.0, mana_y - 24.0);
        draw_deck(board.deck.len(), deck_r.x, deck_r.y, deck_glow);
    } else {
        let hand_y = 10.0;
        let hero_y = mid - HERO_H - 18.0;
        let mana_y = 18.0;
        let deck_r = layout::enemy_deck_rect(w, h);

        draw_hero(&board.hero, w - 100.0, hero_y, false);
        draw_battlefield(&board.battlefield, w, h, false, cache, anim_offsets);
        draw_hand(&board.hand, w, hand_y, false, cache, hand_flips);
        draw_mana(board.mana, board.base_mana, 20.0, mana_y);
        draw_embers(board.embers, 20.0, mana_y + 32.0);
        draw_deck(board.deck.len(), deck_r.x, deck_r.y, false);
    }
}

// ── Card rendering ─────────────────────────────────────────────────────────────

fn draw_card(card: &CardEntity, x: f32, y: f32, w: f32, h: f32, cache: &TextureCache, flip_cos: f32) {
    let scale = flip_cos.abs().clamp(0.0, 1.0);
    let scaled_w = (w * scale).max(1.0);
    let sx = x + (w - scaled_w) / 2.0;

    if flip_cos < 0.0 {
        draw_card_back(sx, y, scaled_w, h);
        return;
    }

    let (color, image_url, name, cost, description) = match card {
        CardEntity::Minion(m) => (
            &m.card.color,
            m.card.image_url.as_str(),
            m.card.name.as_str(),
            m.cost,
            m.card.description.as_deref().unwrap_or(""),
        ),
        CardEntity::Incantation(i) => (
            &i.card.color,
            i.card.image_url.as_str(),
            i.card.name.as_str(),
            i.cost,
            i.card.description.as_deref().unwrap_or(""),
        ),
    };

    draw_card_layers(sx, y, scaled_w, h, color, image_url, name, cost, description, cache, WHITE, scale, true);

    if let CardEntity::Minion(m) = card {
        draw_stat_badge(m.attack, sx + 6.0 * scale, y + h - 14.0, COL_ATK, scale);
        draw_stat_badge(m.defence, sx + scaled_w - 18.0 * scale, y + h - 14.0, COL_DEF, scale);
    } else if scale > 0.6 {
        let sl = "incantation";
        let font = (10.0 * scale) as u16;
        let sd = measure_text(sl, None, font, 1.0);
        draw_text(sl, sx + scaled_w / 2.0 - sd.width / 2.0, y + h - 6.0, font as f32, DARKGRAY);
    }
}

fn draw_minion_card(minion: &MinionEntity, x: f32, y: f32, w: f32, h: f32, cache: &TextureCache) {
    let tint = if minion.exhausted { Color::new(0.5, 0.5, 0.55, 1.0) } else { WHITE };

    draw_card_layers(
        x, y, w, h,
        &minion.card.color,
        &minion.card.image_url,
        &minion.card.name,
        minion.cost,
        minion.card.description.as_deref().unwrap_or(""),
        cache,
        tint,
        1.0,
        false,
    );

    let border = if minion.ward_active {
        GOLD
    } else if minion.stealth_active {
        Color::new(0.5, 0.5, 0.85, 1.0)
    } else if minion.exhausted {
        Color::new(0.4, 0.4, 0.4, 0.6)
    } else {
        Color::new(1.0, 1.0, 1.0, 0.7)
    };
    draw_rectangle_lines(x, y, w, h, 2.0, border);

    draw_stat_badge(minion.attack, x + 6.0, y + h - 14.0, COL_ATK, 1.0);
    draw_stat_badge(minion.defence, x + w - 18.0, y + h - 14.0, COL_DEF, 1.0);
}

fn draw_card_layers(
    x: f32, y: f32, w: f32, h: f32,
    color: &CardColor, image_url: &str, name: &str, cost: i32, description: &str,
    cache: &TextureCache, tint: Color,
    scale: f32,
    show_text: bool,
) {
    if let Some(bg) = cache.bg(color) {
        draw_texture_cover(bg, x, y, w, h, tint);
    } else {
        let bg_col = match color {
            CardColor::White => Color::new(0.75, 0.70, 0.55, 1.0),
            CardColor::Black => Color::new(0.18, 0.14, 0.22, 1.0),
        };
        draw_rectangle(x, y, w, h, bg_col);
    }

    let gem_size = (TOP_BAR - 4.0) * scale;
    draw_rectangle(x + 2.0 * scale, y + 2.0, gem_size, gem_size, COL_MANA);
    if scale > 0.6 {
        let cost_font = (14.0 * scale).round();
        draw_text_centered(&cost.to_string(), x + 2.0 * scale + gem_size / 2.0, y + 2.0 + gem_size * 0.72, cost_font, WHITE);

        if show_text {
            let name_font = (11.0 * scale).round();
            let name_str = fit_text(name, (w - gem_size - 8.0 * scale).max(0.0), name_font);
            draw_text(&name_str, x + gem_size + 6.0 * scale, y + 2.0 + gem_size * 0.72, name_font, BLACK);
        }
    }

    let art_y = y + TOP_BAR;
    let art_h = h * ART_FRAC;
    if let Some(art) = cache.art(image_url) {
        draw_texture_cover(art, x + 2.0 * scale, art_y, (w - 4.0 * scale).max(0.0), art_h, tint);
    } else {
        draw_rectangle(x + 2.0 * scale, art_y, (w - 4.0 * scale).max(0.0), art_h, COL_CARD_BG);
    }

    let desc_y = art_y + art_h;
    let desc_h = h - TOP_BAR - art_h - BOTTOM_BAR;
    draw_rectangle(x + 2.0 * scale, desc_y, (w - 4.0 * scale).max(0.0), desc_h, COL_DESC_BG);
    if scale > 0.6 && show_text {
        let desc_font = (14.0 * scale).round();
        draw_wrapped_text(
            &strip_html(description),
            x + 5.0 * scale,
            desc_y + desc_font,
            (w - 10.0 * scale).max(0.0),
            desc_font,
            BLACK,
        );
    }

    draw_rectangle(x + 2.0 * scale, y + h - BOTTOM_BAR, (w - 4.0 * scale).max(0.0), BOTTOM_BAR, COL_DESC_BG);
}

// ── Sub-components ─────────────────────────────────────────────────────────────

const PREVIEW_W: f32 = 195.0;
const PREVIEW_H: f32 = 285.0;

fn draw_attack_arrow(mx: f32, my: f32, minion_id: u32, state: &GameStateClient, w: f32, h: f32) {
    let rects = layout::self_minion_rects(state.self_board.battlefield.len(), w, h);
    let Some(rect) = state.self_board.battlefield.iter().zip(rects.iter())
        .find(|(m, _)| m.entity_id == minion_id)
        .map(|(_, r)| *r) else { return; };

    let x0 = rect.x + rect.w / 2.0;
    let y0 = rect.y + rect.h / 2.0;
    let dx = mx - x0;
    let dy = my - y0;
    let dist = (dx * dx + dy * dy).sqrt();
    if dist < 1.0 { return; }

    let bend = dist * 0.1;
    let cpx = (x0 + mx) / 2.0 - (dy / dist) * bend;
    let cpy = (y0 + my) / 2.0 + (dx / dist) * bend;

    let col_start = Color::new(0.23, 0.07, 0.05, 1.0); // #3B130C
    let col_end   = Color::new(0.88, 0.32, 0.21, 1.0); // #E05236

    let bezier = |t: f32| -> (f32, f32) {
        let mt = 1.0 - t;
        (mt*mt*x0 + 2.0*mt*t*cpx + t*t*mx, mt*mt*y0 + 2.0*mt*t*cpy + t*t*my)
    };

    // Dashed segments with gradient
    let n = 80;
    let dash = 6.0;
    let gap  = 3.0;
    let mut drawing = true;
    let mut budget = dash;
    let mut prev = bezier(0.0);

    for i in 1..=n {
        let t = i as f32 / n as f32;
        let p = bezier(t);
        let seg = ((p.0-prev.0).powi(2) + (p.1-prev.1).powi(2)).sqrt();
        if drawing {
            let col = Color::new(
                col_start.r + (col_end.r - col_start.r) * t,
                col_start.g + (col_end.g - col_start.g) * t,
                col_start.b + (col_end.b - col_start.b) * t,
                1.0,
            );
            draw_line(prev.0, prev.1, p.0, p.1, 2.5, col);
        }
        budget -= seg;
        if budget <= 0.0 {
            drawing = !drawing;
            budget = if drawing { dash } else { gap };
        }
        prev = p;
    }

    // Arrowhead pointing in the tangent direction at t=1
    let dir = Vec2::new(mx - cpx, my - cpy).normalize();
    let perp = Vec2::new(-dir.y, dir.x);
    let size = 14.0;
    let tip  = Vec2::new(mx, my);
    let base = tip - dir * size;
    draw_triangle(tip, base + perp * (size * 0.5), base - perp * (size * 0.5), col_end);
}

fn draw_minion_preview(entity_id: u32, state: &GameStateClient, cache: &TextureCache, w: f32, h: f32) {
    let found = {
        let rects = layout::self_minion_rects(state.self_board.battlefield.len(), w, h);
        state.self_board.battlefield.iter().zip(rects.iter())
            .find(|(m, _)| m.entity_id == entity_id)
            .map(|(m, r)| (m, *r))
    }.or_else(|| {
        let rects = layout::enemy_minion_rects(state.enemy_board.battlefield.len(), w, h);
        state.enemy_board.battlefield.iter().zip(rects.iter())
            .find(|(m, _)| m.entity_id == entity_id)
            .map(|(m, r)| (m, *r))
    });

    let Some((minion, rect)) = found else { return; };

    let pw = PREVIEW_W;
    let ph = PREVIEW_H;
    let px = if rect.x + rect.w + pw + 10.0 <= w {
        rect.x + rect.w + 10.0
    } else {
        rect.x - pw - 10.0
    };
    let py = (rect.y + rect.h / 2.0 - ph / 2.0).clamp(0.0, h - ph);

    draw_rectangle(px + 5.0, py + 8.0, pw, ph, Color::new(0.0, 0.0, 0.0, 0.55));
    draw_card_preview(minion, px, py, pw, ph, cache);
}

fn draw_card_preview(minion: &MinionEntity, x: f32, y: f32, w: f32, h: f32, cache: &TextureCache) {
    let card = &minion.card;
    let color = &card.color;

    if let Some(bg) = cache.bg(color) {
        draw_texture_cover(bg, x, y, w, h, WHITE);
    } else {
        let bg_col = match color {
            shared::types::Color::White => Color::new(0.75, 0.70, 0.55, 1.0),
            shared::types::Color::Black => Color::new(0.18, 0.14, 0.22, 1.0),
        };
        draw_rectangle(x, y, w, h, bg_col);
    }

    // cost gem
    let gem = 30.0;
    draw_rectangle(x + 2.0, y + 2.0, gem, gem, COL_MANA);
    draw_text_centered(&card.base_cost.to_string(), x + 2.0 + gem / 2.0, y + 2.0 + gem * 0.78, 22.0, WHITE);

    // name
    let name_fit = fit_text(&card.name, w - gem - 10.0, 18.0);
    draw_text(&name_fit, x + gem + 6.0, y + 2.0 + gem * 0.78, 18.0, BLACK);

    // art
    let art_y = y + gem + 4.0;
    let art_h = h * ART_FRAC;
    if let Some(art) = cache.art(&card.image_url) {
        draw_texture_cover(art, x + 2.0, art_y, w - 4.0, art_h, WHITE);
    } else {
        draw_rectangle(x + 2.0, art_y, w - 4.0, art_h, COL_CARD_BG);
    }

    // description
    let desc_y = art_y + art_h;
    let desc_h = h - (gem + 4.0) - art_h - 28.0;
    draw_rectangle(x + 2.0, desc_y, w - 4.0, desc_h, COL_DESC_BG);
    let desc = card.description.as_deref().unwrap_or("");
    draw_wrapped_text(&strip_html(desc), x + 6.0, desc_y + 18.0, w - 12.0, 18.0, BLACK);

    // bottom bar with current stats
    let bot_y = y + h - 26.0;
    draw_rectangle(x + 2.0, bot_y, w - 4.0, 24.0, COL_DESC_BG);
    draw_stat_badge(minion.attack, x + 8.0, bot_y + 18.0, COL_ATK, 1.0);
    draw_stat_badge(minion.defence, x + w - 22.0, bot_y + 18.0, COL_DEF, 1.0);

    draw_rectangle_lines(x, y, w, h, 1.5, Color::new(0.6, 0.6, 0.6, 0.7));
}

fn draw_battlefield(minions: &[MinionEntity], w: f32, h: f32, is_self: bool, cache: &TextureCache, anim_offsets: &[(u32, Vec2)]) {
    let rects = if is_self {
        layout::self_minion_rects(minions.len(), w, h)
    } else {
        layout::enemy_minion_rects(minions.len(), w, h)
    };
    for (minion, rect) in minions.iter().zip(rects.iter()) {
        let off = anim_offsets.iter()
            .find(|(id, _)| *id == minion.entity_id)
            .map(|(_, v)| *v)
            .unwrap_or(Vec2::ZERO);
        draw_minion_card(minion, rect.x + off.x, rect.y + off.y, rect.w, rect.h, cache);
    }
}

fn draw_hand(hand: &[CardEntity], w: f32, y: f32, is_self: bool, cache: &TextureCache, hand_flips: &[(u32, f32)]) {
    let count = hand.len() as f32;
    let total = count * (CARD_W + CARD_GAP) - CARD_GAP;
    let start_x = w / 2.0 - total / 2.0;

    for (i, card) in hand.iter().enumerate() {
        let x = start_x + i as f32 * (CARD_W + CARD_GAP);
        if is_self {
            let flip_cos = hand_flips.iter()
                .find(|(id, _)| *id == card.entity_id())
                .map(|(_, v)| *v)
                .unwrap_or(1.0);
            draw_card(card, x, y, CARD_W, CARD_H, cache, flip_cos);
        } else {
            draw_card_back(x, y, CARD_W, CARD_H);
        }
    }
}

fn draw_card_back(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::new(0.08, 0.08, 0.14, 1.0));
    draw_rectangle_lines(x, y, w, h, 1.5, Color::new(0.35, 0.18, 0.45, 1.0));
    let steps = 6;
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        draw_line(x, y + h * t, x + w, y + h * (1.0 - t), 0.5, Color::new(0.3, 0.15, 0.4, 0.4));
    }
}

fn draw_hero(hero: &shared::types::Hero, x: f32, y: f32, is_self: bool) {
    let color = if is_self { COL_SELF } else { COL_ENEMY };
    draw_rectangle(x, y, HERO_W, HERO_H, color);
    draw_rectangle_lines(x, y, HERO_W, HERO_H, 2.0, WHITE);
    let hp = hero.defence.to_string();
    let dims = measure_text(&hp, None, 24, 1.0);
    draw_text(&hp, x + HERO_W / 2.0 - dims.width / 2.0, y + HERO_H / 2.0 + 8.0, 24.0, WHITE);
    draw_text("HP", x + 4.0, y + HERO_H - 5.0, 11.0, LIGHTGRAY);
}

fn draw_embers(current: i32, x: f32, y: f32) {
    let r = 5.0;
    let gap = r * 2.4;
    for i in 0..EMBER_MAX {
        let cx = x + r + i as f32 * gap;
        let color = if i < current { COL_EMBER } else { COL_EMBER_EMPTY };
        draw_circle(cx, y + r, r, color);
        draw_circle_lines(cx, y + r, r, 1.0, Color::new(1.0, 0.7, 0.4, 0.5));
    }
}

fn draw_mana(current: i32, max: i32, x: f32, y: f32) {
    let r = 7.0;
    let gap = r * 2.4;
    for i in 0..max {
        let cx = x + r + i as f32 * gap;
        let color = if i < current { COL_MANA } else { COL_MANA_EMPTY };
        draw_circle(cx, y + r, r, color);
        draw_circle_lines(cx, y + r, r, 1.0, Color::new(1.0, 1.0, 1.0, 0.5));
    }
    draw_text(&format!("{}/{}", current, max), x, y + r * 2.0 + 14.0, 13.0, LIGHTGRAY);
}

fn draw_deck(count: usize, x: f32, y: f32, glow: bool) {
    let w = CARD_W;
    let h = CARD_H;
    // card back
    draw_rectangle(x, y, w, h, Color::new(0.08, 0.08, 0.14, 1.0));
    let border = if glow { Color::new(0.9, 0.85, 0.3, 1.0) } else { Color::new(0.35, 0.18, 0.45, 1.0) };
    draw_rectangle_lines(x, y, w, h, if glow { 3.0 } else { 1.5 }, border);
    // diagonal pattern
    let steps = 8;
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        draw_line(x, y + h * t, x + w, y + h * (1.0 - t), 0.5, Color::new(0.3, 0.15, 0.4, 0.4));
    }
    // count badge
    draw_rectangle(x + w / 2.0 - 16.0, y + h / 2.0 - 14.0, 32.0, 28.0, Color::new(0.0, 0.0, 0.0, 0.65));
    draw_text_centered(&count.to_string(), x + w / 2.0, y + h / 2.0 + 6.0, 20.0, WHITE);
}

fn draw_stat_badge(val: i32, x: f32, y: f32, color: Color, scale: f32) {
    let bw = 20.0 * scale;
    let bh = 18.0 * scale;
    draw_rectangle(x - 2.0 * scale, y - 13.0 * scale, bw, bh, color);
    draw_rectangle_lines(x - 2.0 * scale, y - 13.0 * scale, bw, bh, 1.0, BLACK);
    if scale > 0.6 {
        let font = (13.0 * scale).round();
        let s = val.to_string();
        let d = measure_text(&s, None, font as u16, 1.0);
        draw_text(&s, x + 8.0 * scale - d.width / 2.0, y, font, WHITE);
    }
}

fn draw_turn_indicator(state: &GameStateClient, w: f32, mid: f32) {
    let label = if state.your_turn { "YOUR TURN" } else { "OPPONENT'S TURN" };
    let color = if state.your_turn { GREEN } else { GRAY };
    let dims = measure_text(label, None, 18, 1.0);
    draw_text(label, w / 2.0 - dims.width / 2.0, mid - 4.0, 18.0, color);
    let turn = format!("Turn {}", state.turn_count);
    let td = measure_text(&turn, None, 13, 1.0);
    draw_text(&turn, w / 2.0 - td.width / 2.0, mid + 12.0, 13.0, GRAY);
}

fn draw_end_turn_button(w: f32, mid: f32) {
    let bx = w - 124.0;
    let by = mid + 14.0;
    let bw = 104.0;
    let bh = 36.0;
    draw_rectangle(bx, by, bw, bh, Color::new(0.1, 0.55, 0.18, 1.0));
    draw_rectangle_lines(bx, by, bw, bh, 1.5, GREEN);
    let label = "END TURN [E]";
    let d = measure_text(label, None, 14, 1.0);
    draw_text(label, bx + bw / 2.0 - d.width / 2.0, by + bh / 2.0 + 5.0, 14.0, WHITE);
}

pub fn draw_button(label: &str, x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::new(0.18, 0.28, 0.48, 1.0));
    draw_rectangle_lines(x, y, w, h, 1.5, Color::new(0.38, 0.56, 0.82, 1.0));
    let d = measure_text(label, None, 17, 1.0);
    draw_text(label, x + w / 2.0 - d.width / 2.0, y + h / 2.0 + 6.0, 17.0, WHITE);
}

pub fn draw_button_danger(label: &str, x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::new(0.35, 0.08, 0.08, 1.0));
    draw_rectangle_lines(x, y, w, h, 1.5, Color::new(0.70, 0.20, 0.20, 1.0));
    let d = measure_text(label, None, 15, 1.0);
    draw_text(label, x + w / 2.0 - d.width / 2.0, y + h / 2.0 + 5.0, 15.0, Color::new(0.9, 0.6, 0.6, 1.0));
}

// ── Deck Builder ──────────────────────────────────────────────────────────────

pub const DB_CATALOG_W: f32 = 760.0;
pub const DB_CARD_W: f32 = 95.0;
pub const DB_CARD_H: f32 = 138.0;
pub const DB_CARD_GAP: f32 = 6.0;
pub const DB_COLS: usize = 6;
pub const DB_FILTER_H: f32 = 38.0;
pub const DB_ROW_H: f32 = 38.0;

pub fn draw_deck_builder(state: &DeckBuilderState, cache: &TextureCache) {
    let w = screen_width();
    let h = screen_height();

    // Background panels
    draw_rectangle(0.0, 0.0, DB_CATALOG_W, h, Color::new(0.10, 0.10, 0.16, 1.0));
    draw_rectangle(DB_CATALOG_W, 0.0, w - DB_CATALOG_W, h, Color::new(0.13, 0.13, 0.20, 1.0));

    draw_filter_bar(state, h);
    draw_catalog(state, cache);
    draw_right_panel(state, cache, w, h);

    if let Some((msg, _)) = &state.clipboard_msg {
        let d = measure_text(msg, None, 16, 1.0);
        let bx = w / 2.0 - d.width / 2.0 - 10.0;
        let by = h - 50.0;
        draw_rectangle(bx, by, d.width + 20.0, 30.0, Color::new(0.1, 0.55, 0.18, 0.9));
        draw_text(msg, bx + 10.0, by + 21.0, 16.0, WHITE);
    }
}

fn draw_filter_bar(state: &DeckBuilderState, h: f32) {
    let bar_y = h - DB_FILTER_H - 4.0;
    draw_rectangle(0.0, bar_y, DB_CATALOG_W, DB_FILTER_H + 4.0, Color::new(0.08, 0.08, 0.14, 1.0));

    // Search label
    draw_text("Search:", 8.0, bar_y + 25.0, 14.0, LIGHTGRAY);

    // Search box (drawn but editing is handled externally; show current text)
    let sx = 68.0;
    draw_rectangle(sx, bar_y + 5.0, 160.0, 28.0, Color::new(0.18, 0.18, 0.28, 1.0));
    draw_rectangle_lines(sx, bar_y + 5.0, 160.0, 28.0, 1.0, Color::new(0.4, 0.4, 0.6, 1.0));
    let search_disp = if state.search.is_empty() { "type to search" } else { &state.search };
    let sc = if state.search.is_empty() { DARKGRAY } else { WHITE };
    draw_text(search_disp, sx + 6.0, bar_y + 23.0, 13.0, sc);

    // Type toggle
    let tx = 240.0;
    draw_toggle_btn("All", tx, bar_y + 5.0, 46.0, 28.0, state.filter_type == crate::deckbuilder::FilterType::All);
    draw_toggle_btn("Minions", tx + 50.0, bar_y + 5.0, 68.0, 28.0, state.filter_type == crate::deckbuilder::FilterType::Minions);
    draw_toggle_btn("Spells", tx + 122.0, bar_y + 5.0, 60.0, 28.0, state.filter_type == crate::deckbuilder::FilterType::Incantations);

    // Color toggle
    let cx2 = 410.0;
    draw_toggle_btn("All", cx2, bar_y + 5.0, 36.0, 28.0, state.filter_color == crate::deckbuilder::ColorFilter::All);
    draw_toggle_btn_colored("W", cx2 + 40.0, bar_y + 5.0, 28.0, 28.0,
        state.filter_color == crate::deckbuilder::ColorFilter::White,
        Color::new(0.80, 0.75, 0.55, 1.0));
    draw_toggle_btn_colored("B", cx2 + 72.0, bar_y + 5.0, 28.0, 28.0,
        state.filter_color == crate::deckbuilder::ColorFilter::Black,
        Color::new(0.30, 0.15, 0.40, 1.0));

    // Back button
    draw_button("[Esc] Back", DB_CATALOG_W - 124.0, bar_y + 5.0, 120.0, 28.0);
}

fn draw_toggle_btn(label: &str, x: f32, y: f32, w: f32, h: f32, active: bool) {
    let bg = if active { Color::new(0.25, 0.45, 0.72, 1.0) } else { Color::new(0.18, 0.18, 0.28, 1.0) };
    draw_rectangle(x, y, w, h, bg);
    draw_rectangle_lines(x, y, w, h, 1.0, Color::new(0.4, 0.5, 0.7, 1.0));
    let d = measure_text(label, None, 13, 1.0);
    draw_text(label, x + w / 2.0 - d.width / 2.0, y + h / 2.0 + 5.0, 13.0, WHITE);
}

fn draw_toggle_btn_colored(label: &str, x: f32, y: f32, w: f32, h: f32, active: bool, accent: Color) {
    let bg = if active { accent } else { Color::new(0.18, 0.18, 0.28, 1.0) };
    draw_rectangle(x, y, w, h, bg);
    draw_rectangle_lines(x, y, w, h, 1.0, accent);
    let d = measure_text(label, None, 13, 1.0);
    draw_text(label, x + w / 2.0 - d.width / 2.0, y + h / 2.0 + 5.0, 13.0, WHITE);
}

fn draw_catalog(state: &DeckBuilderState, cache: &TextureCache) {
    let h = screen_height();
    let area_h = h - DB_FILTER_H - 8.0;
    let cols = DB_COLS;
    let row_h = DB_CARD_H + DB_CARD_GAP;
    let start_x = (DB_CATALOG_W - cols as f32 * (DB_CARD_W + DB_CARD_GAP) + DB_CARD_GAP) / 2.0;

    let scroll_offset = state.catalog_scroll;

    for (grid_i, &card_idx) in state.filtered.iter().enumerate() {
        let col = grid_i % cols;
        let row = grid_i / cols;
        let cx = start_x + col as f32 * (DB_CARD_W + DB_CARD_GAP);
        let cy = row as f32 * row_h + 4.0 - scroll_offset;

        if cy + DB_CARD_H < 0.0 || cy > area_h {
            continue;
        }

        let card = &state.all_cards[card_idx];
        draw_catalog_card(card, cx, cy, cache);
    }
}

fn draw_catalog_card(card: &Card, x: f32, y: f32, cache: &TextureCache) {
    let (name, color, is_minion) = card_meta(card);
    let image_url = card_image_url(card);
    let cost = card_cost(card);

    // background
    if let Some(bg) = cache.bg(color) {
        draw_texture_cover(bg, x, y, DB_CARD_W, DB_CARD_H, WHITE);
    } else {
        let bg_col = match color {
            CardColor::White => Color::new(0.75, 0.70, 0.55, 1.0),
            CardColor::Black => Color::new(0.18, 0.14, 0.22, 1.0),
        };
        draw_rectangle(x, y, DB_CARD_W, DB_CARD_H, bg_col);
    }

    // art
    let art_y = y + 18.0;
    let art_h = DB_CARD_H * 0.40;
    if let Some(art) = cache.art(image_url) {
        draw_texture_cover(art, x + 1.0, art_y, DB_CARD_W - 2.0, art_h, WHITE);
    } else {
        draw_rectangle(x + 1.0, art_y, DB_CARD_W - 2.0, art_h, COL_CARD_BG);
    }

    // cost gem
    draw_rectangle(x + 2.0, y + 2.0, 16.0, 14.0, COL_MANA);
    draw_text_centered(&cost.to_string(), x + 10.0, y + 13.0, 11.0, WHITE);

    // name
    let name_fit = fit_text(name, DB_CARD_W - 22.0, 9.0);
    draw_text(&name_fit, x + 20.0, y + 13.0, 9.0, BLACK);

    // bottom bar
    let bot_y = y + DB_CARD_H - 16.0;
    draw_rectangle(x + 1.0, bot_y, DB_CARD_W - 2.0, 15.0, COL_DESC_BG);
    if is_minion {
        if let Card::Minion(m) = card {
            draw_text_centered(&m.base_attack.to_string(), x + 10.0, bot_y + 11.0, 10.0, Color::new(0.8, 0.6, 0.1, 1.0));
            draw_text_centered(&m.base_defence.to_string(), x + DB_CARD_W - 10.0, bot_y + 11.0, 10.0, Color::new(0.8, 0.25, 0.25, 1.0));
        }
    } else {
        let sl = "spell";
        let sd = measure_text(sl, None, 8, 1.0);
        draw_text(sl, x + DB_CARD_W / 2.0 - sd.width / 2.0, bot_y + 11.0, 8.0, DARKGRAY);
    }

    draw_rectangle_lines(x, y, DB_CARD_W, DB_CARD_H, 1.0, Color::new(0.5, 0.5, 0.5, 0.5));
}

fn draw_right_panel(state: &DeckBuilderState, cache: &TextureCache, w: f32, h: f32) {
    let px = DB_CATALOG_W + 6.0;
    let pw = w - px - 6.0;

    match &state.panel {
        Panel::DeckList => draw_deck_list(state, cache, px, pw, h),
        Panel::Editor { name, .. } => draw_deck_editor(state, cache, px, pw, h, name),
    }
}

fn draw_deck_list(state: &DeckBuilderState, cache: &TextureCache, px: f32, pw: f32, h: f32) {
    let title = "Your Decks";
    let td = measure_text(title, None, 18, 1.0);
    draw_text(title, px + pw / 2.0 - td.width / 2.0, 28.0, 18.0, WHITE);

    let scroll = state.deck_scroll;
    let list_y_start = 40.0;
    let list_h = h - 40.0 - 4.0 * (DB_ROW_H + 4.0) - 8.0;

    // clip indicator line
    draw_line(px, list_y_start + list_h, px + pw, list_y_start + list_h, 1.0, DARKGRAY);

    for (i, deck) in state.decks.iter().enumerate() {
        let ry = list_y_start + i as f32 * (DB_ROW_H + 3.0) - scroll;
        if ry + DB_ROW_H < list_y_start || ry > list_y_start + list_h {
            continue;
        }

        // art background from first card
        let art_painted = if let Some(first_id) = deck.cards.first() {
            if let Some(card) = state.card_by_id(*first_id) {
                let url = card_image_url(card);
                if let Some(tex) = cache.art(url) {
                    draw_texture_cover(tex, px, ry, pw, DB_ROW_H, Color::new(0.6, 0.6, 0.6, 1.0));
                    true
                } else { false }
            } else { false }
        } else { false };

        if !art_painted {
            draw_rectangle(px, ry, pw, DB_ROW_H, Color::new(0.18, 0.18, 0.28, 1.0));
        }
        // dark gradient overlay
        draw_rectangle(px, ry, pw * 0.6, DB_ROW_H, Color::new(0.0, 0.0, 0.0, 0.65));
        let name_fit = fit_text(&deck.name, pw - 20.0, 14.0);
        draw_text(&name_fit, px + 8.0, ry + DB_ROW_H / 2.0 + 6.0, 14.0, WHITE);
        let count_s = format!("{}", deck.cards.len());
        let cd = measure_text(&count_s, None, 13, 1.0);
        draw_text(&count_s, px + pw - cd.width - 8.0, ry + DB_ROW_H / 2.0 + 5.0, 13.0, LIGHTGRAY);
        draw_rectangle_lines(px, ry, pw, DB_ROW_H, 1.0, Color::new(0.3, 0.3, 0.4, 0.7));
    }

    // Bottom buttons
    let btn_y_base = h - 4.0 * (DB_ROW_H + 4.0) - 4.0;
    draw_button("New Deck  [N]", px, btn_y_base, pw, DB_ROW_H);
    draw_button("Import Decks  [I]", px, btn_y_base + DB_ROW_H + 4.0, pw, DB_ROW_H);
    draw_button("Export All  [X]", px, btn_y_base + (DB_ROW_H + 4.0) * 2.0, pw, DB_ROW_H);
    draw_button("[Esc] Play", px, btn_y_base + (DB_ROW_H + 4.0) * 3.0, pw, DB_ROW_H);
}

fn draw_deck_editor(state: &DeckBuilderState, cache: &TextureCache, px: f32, pw: f32, h: f32, name: &str) {
    // Name + count header
    draw_rectangle(px, 0.0, pw, 42.0, Color::new(0.18, 0.18, 0.30, 1.0));
    draw_rectangle_lines(px, 0.0, pw, 42.0, 1.0, Color::new(0.35, 0.35, 0.55, 1.0));

    let disp_name = if name.is_empty() { "Unnamed Deck" } else { name };
    let nd = measure_text(disp_name, None, 14, 1.0);
    draw_text(disp_name, px + pw / 2.0 - nd.width / 2.0, 26.0, 14.0, WHITE);

    let count_label = format!("{} / 50", state.editing_cards.len());
    let cd = measure_text(&count_label, None, 12, 1.0);
    draw_text(&count_label, px + pw - cd.width - 6.0, 16.0, 12.0, LIGHTGRAY);

    // Hint
    draw_text("Click card to add · Click row to remove", px + 4.0, 38.0, 9.0, DARKGRAY);

    let scroll = state.deck_scroll;
    let list_y_start = 44.0;
    let list_h = h - 44.0 - 3.0 * (DB_ROW_H + 4.0) - 8.0;

    draw_line(px, list_y_start + list_h, px + pw, list_y_start + list_h, 1.0, DARKGRAY);

    let uniques = state.unique_editing();
    for (i, &card_id_val) in uniques.iter().enumerate() {
        let ry = list_y_start + i as f32 * (DB_ROW_H + 2.0) - scroll;
        if ry + DB_ROW_H < list_y_start || ry > list_y_start + list_h {
            continue;
        }

        let art_painted = if let Some(card) = state.card_by_id(card_id_val) {
            let url = card_image_url(card);
            if let Some(tex) = cache.art(url) {
                draw_texture_cover(tex, px, ry, pw, DB_ROW_H, Color::new(0.55, 0.55, 0.55, 1.0));
                true
            } else { false }
        } else { false };

        if !art_painted {
            draw_rectangle(px, ry, pw, DB_ROW_H, Color::new(0.15, 0.15, 0.25, 1.0));
        }
        draw_rectangle(px, ry, pw * 0.65, DB_ROW_H, Color::new(0.0, 0.0, 0.0, 0.6));

        if let Some(card) = state.card_by_id(card_id_val) {
            let (name_s, _, _) = card_meta(card);
            let nf = fit_text(name_s, pw - 50.0, 13.0);
            draw_text(&nf, px + 6.0, ry + DB_ROW_H / 2.0 + 5.0, 13.0, WHITE);
        }

        let cnt = state.count_of(card_id_val);
        let cnt_s = format!("x{}", cnt);
        let csd = measure_text(&cnt_s, None, 13, 1.0);
        draw_text(&cnt_s, px + pw - csd.width - 6.0, ry + DB_ROW_H / 2.0 + 5.0, 13.0, LIGHTGRAY);

        draw_rectangle_lines(px, ry, pw, DB_ROW_H, 1.0, Color::new(0.3, 0.3, 0.4, 0.6));
    }

    let btn_y_base = h - 3.0 * (DB_ROW_H + 4.0) - 4.0;
    draw_button("Export Deck  [X]", px, btn_y_base, pw, DB_ROW_H);
    draw_button("Done  [Enter]", px, btn_y_base + DB_ROW_H + 4.0, pw, DB_ROW_H);
    draw_button_danger("Delete Deck  [Del]", px, btn_y_base + (DB_ROW_H + 4.0) * 2.0, pw, DB_ROW_H);
}

// ── Helpers ────────────────────────────────────────────────────────────────────

fn draw_text_centered(text: &str, cx: f32, cy: f32, size: f32, color: Color) {
    let d = measure_text(text, None, size as u16, 1.0);
    draw_text(text, cx - d.width / 2.0, cy, size, color);
}

fn draw_crosshair(mx: f32, my: f32) {
    let r = 11.0;
    let gap = 5.0;
    let len = 14.0;
    let thick = 1.5;
    let col = Color::new(0.95, 0.25, 0.15, 0.92);

    draw_circle_lines(mx, my, r, thick, col);
    draw_line(mx, my - r - gap, mx, my - r - gap - len, thick, col);
    draw_line(mx, my + r + gap, mx, my + r + gap + len, thick, col);
    draw_line(mx - r - gap, my, mx - r - gap - len, my, thick, col);
    draw_line(mx + r + gap, my, mx + r + gap + len, my, thick, col);
}

fn draw_wrapped_text(text: &str, x: f32, mut y: f32, max_w: f32, size: f32, color: Color) {
    let line_h = size + 3.0;
    let mut line = String::new();
    for word in text.split_whitespace() {
        let candidate = if line.is_empty() { word.to_string() } else { format!("{} {}", line, word) };
        if measure_text(&candidate, None, size as u16, 1.0).width > max_w && !line.is_empty() {
            draw_text(&line, x, y, size, color);
            y += line_h;
            line = word.to_string();
        } else {
            line = candidate;
        }
    }
    if !line.is_empty() {
        draw_text(&line, x, y, size, color);
    }
}

fn strip_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.replace("&nbsp;", " ")
}

fn fit_text(s: &str, max_w: f32, size: f32) -> String {
    if measure_text(s, None, size as u16, 1.0).width <= max_w {
        return s.to_string();
    }
    let mut result = String::new();
    for ch in s.chars() {
        let candidate = format!("{}{}…", result, ch);
        if measure_text(&candidate, None, size as u16, 1.0).width > max_w {
            return format!("{}…", result);
        }
        result.push(ch);
    }
    result
}
