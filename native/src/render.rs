use macroquad::prelude::*;

use crate::textures::{draw_texture_cover, TextureCache};
use crate::types::{Board, CardEntity, Color as CardColor, GameStateClient, MinionEntity};

// ── Layout constants ───────────────────────────────────────────────────────────

const CARD_W: f32 = 110.0;
const CARD_H: f32 = 160.0;
const CARD_GAP: f32 = 8.0;

const MINION_W: f32 = 100.0;
const MINION_H: f32 = 145.0;
const MINION_GAP: f32 = 8.0;

const HERO_W: f32 = 64.0;
const HERO_H: f32 = 64.0;

// Fraction of card height that the art frame occupies
const ART_FRAC: f32 = 0.38;
// Pixel heights of top/bottom bars
const TOP_BAR: f32 = 28.0;
const BOTTOM_BAR: f32 = 26.0;

const COL_ENEMY: Color = Color::new(0.7, 0.2, 0.2, 1.0);
const COL_SELF: Color = Color::new(0.2, 0.5, 0.8, 1.0);
const COL_CARD_BG: Color = Color::new(0.15, 0.15, 0.25, 1.0);
const COL_EXHAUSTED: Color = Color::new(0.25, 0.25, 0.28, 1.0);
const COL_MANA: Color = Color::new(0.34, 0.60, 0.80, 1.0);
const COL_MANA_EMPTY: Color = Color::new(0.12, 0.12, 0.22, 1.0);
const COL_DIVIDER: Color = Color::new(0.5, 0.5, 0.5, 0.3);
const COL_ATK: Color = Color::new(0.90, 0.70, 0.20, 1.0);
const COL_DEF: Color = Color::new(0.70, 0.25, 0.25, 1.0);
const COL_DESC_BG: Color = Color::new(0.82, 0.76, 0.67, 0.9);

// ── Public entry points ────────────────────────────────────────────────────────

pub fn draw_game(state: &GameStateClient, cache: &TextureCache) {
    let w = screen_width();
    let h = screen_height();
    let mid = h / 2.0;

    draw_line(0.0, mid, w, mid, 1.5, COL_DIVIDER);
    draw_turn_indicator(state, w, mid);

    draw_board_half(&state.enemy_board, w, mid, false, cache);
    draw_board_half(&state.self_board, w, mid, true, cache);

    if state.your_turn {
        draw_end_turn_button(w, mid);
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
        draw_card(card, x, card_y, full_w, full_h, cache);

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
}

pub fn draw_connecting() {
    let w = screen_width();
    let h = screen_height();
    let text = "Connecting...";
    let dims = measure_text(text, None, 28, 1.0);
    draw_text(text, w / 2.0 - dims.width / 2.0, h / 2.0, 28.0, GRAY);
}

// ── Board halves ──────────────────────────────────────────────────────────────

fn draw_board_half(board: &Board, w: f32, mid: f32, is_self: bool, cache: &TextureCache) {
    let (battlefield_y, hand_y, hero_y, mana_y) = if is_self {
        (
            mid + 18.0,
            mid + MINION_H + 30.0,
            mid + MINION_H / 2.0 - HERO_H / 2.0 + 18.0,
            mid + MINION_H + CARD_H + 48.0,
        )
    } else {
        (
            mid - MINION_H - 18.0,
            mid - MINION_H - CARD_H - 30.0,
            mid - MINION_H / 2.0 - HERO_H / 2.0 - 18.0,
            mid - MINION_H - CARD_H - 52.0,
        )
    };

    draw_hero(&board.hero, w - 100.0, hero_y, is_self);
    draw_battlefield(&board.battlefield, w, battlefield_y, cache);
    draw_hand(&board.hand, w, hand_y, is_self, cache);
    draw_mana(board.mana, board.base_mana, 20.0, mana_y);
    draw_deck_count(board.deck.len(), w - 110.0, hero_y + HERO_H + 8.0);
}

// ── Card rendering ─────────────────────────────────────────────────────────────

/// Full card with background texture, art, name, cost, stats.
fn draw_card(card: &CardEntity, x: f32, y: f32, w: f32, h: f32, cache: &TextureCache) {
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

    draw_card_layers(x, y, w, h, color, image_url, name, cost, description, cache);

    // Attack/defense for minions
    if let CardEntity::Minion(m) = card {
        draw_stat_badge(m.attack, x + 6.0, y + h - 14.0, COL_ATK);
        draw_stat_badge(m.defence, x + w - 18.0, y + h - 14.0, COL_DEF);
    } else {
        let sl = "incantation";
        let sd = measure_text(sl, None, 10, 1.0);
        draw_text(sl, x + w / 2.0 - sd.width / 2.0, y + h - 6.0, 10.0, DARKGRAY);
    }
}

/// Renders a battlefield minion as a card (may be smaller).
fn draw_minion_card(minion: &MinionEntity, x: f32, y: f32, w: f32, h: f32, cache: &TextureCache) {
    let tint = if minion.exhausted {
        Color::new(0.5, 0.5, 0.55, 1.0)
    } else {
        WHITE
    };

    draw_card_layers_tinted(
        x, y, w, h,
        &minion.card.color,
        &minion.card.image_url,
        &minion.card.name,
        minion.cost,
        minion.card.description.as_deref().unwrap_or(""),
        cache,
        tint,
    );

    // Keyword border highlights
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

    draw_stat_badge(minion.attack, x + 6.0, y + h - 14.0, COL_ATK);
    draw_stat_badge(minion.defence, x + w - 18.0, y + h - 14.0, COL_DEF);
}

fn draw_card_layers(
    x: f32, y: f32, w: f32, h: f32,
    color: &CardColor, image_url: &str, name: &str, cost: i32, description: &str,
    cache: &TextureCache,
) {
    draw_card_layers_tinted(x, y, w, h, color, image_url, name, cost, description, cache, WHITE);
}

fn draw_card_layers_tinted(
    x: f32, y: f32, w: f32, h: f32,
    color: &CardColor, image_url: &str, name: &str, cost: i32, description: &str,
    cache: &TextureCache,
    tint: Color,
) {
    // 1. Background
    if let Some(bg) = cache.bg(color) {
        draw_texture_cover(bg, x, y, w, h, tint);
    } else {
        let bg_col = match color {
            CardColor::White => Color::new(0.75, 0.70, 0.55, 1.0),
            CardColor::Black => Color::new(0.18, 0.14, 0.22, 1.0),
        };
        draw_rectangle(x, y, w, h, bg_col);
    }

    // 2. Top bar: cost gem | name
    let gem_size = TOP_BAR - 4.0;
    draw_rectangle(x + 2.0, y + 2.0, gem_size, gem_size, COL_MANA);
    draw_text_centered(&cost.to_string(), x + 2.0 + gem_size / 2.0, y + 2.0 + gem_size * 0.72, 14.0, WHITE);

    let name_x = x + gem_size + 6.0;
    let name_max_w = w - gem_size - 8.0;
    let name_str = fit_text(name, name_max_w, 11.0);
    draw_text(&name_str, name_x, y + 2.0 + gem_size * 0.72, 11.0, BLACK);

    // 3. Art frame
    let art_y = y + TOP_BAR;
    let art_h = h * ART_FRAC;
    let art_pad = 2.0;
    if let Some(art) = cache.art(image_url) {
        draw_texture_cover(art, x + art_pad, art_y, w - art_pad * 2.0, art_h, tint);
    } else {
        draw_rectangle(x + art_pad, art_y, w - art_pad * 2.0, art_h, COL_CARD_BG);
    }

    // 4. Description box
    let desc_y = art_y + art_h;
    let desc_h = h - TOP_BAR - art_h - BOTTOM_BAR;
    draw_rectangle(x + 2.0, desc_y, w - 4.0, desc_h, COL_DESC_BG);
    let clean = strip_html(description);
    let desc_str = fit_text(&clean, w - 10.0, 9.0);
    draw_text(&desc_str, x + 5.0, desc_y + 12.0, 9.0, BLACK);

    // 5. Bottom bar background
    draw_rectangle(x + 2.0, y + h - BOTTOM_BAR, w - 4.0, BOTTOM_BAR, COL_DESC_BG);
}

// ── Sub-components ─────────────────────────────────────────────────────────────

fn draw_battlefield(minions: &[MinionEntity], w: f32, y: f32, cache: &TextureCache) {
    let count = minions.len() as f32;
    let total = count * (MINION_W + MINION_GAP) - MINION_GAP;
    let start_x = w / 2.0 - total / 2.0;

    for (i, minion) in minions.iter().enumerate() {
        let x = start_x + i as f32 * (MINION_W + MINION_GAP);
        draw_minion_card(minion, x, y, MINION_W, MINION_H, cache);
    }
}

fn draw_hand(hand: &[CardEntity], w: f32, y: f32, is_self: bool, cache: &TextureCache) {
    let count = hand.len() as f32;
    let total = count * (CARD_W + CARD_GAP) - CARD_GAP;
    let start_x = w / 2.0 - total / 2.0;

    for (i, card) in hand.iter().enumerate() {
        let x = start_x + i as f32 * (CARD_W + CARD_GAP);
        if is_self {
            draw_card(card, x, y, CARD_W, CARD_H, cache);
        } else {
            draw_card_back(x, y, CARD_W, CARD_H);
        }
    }
}

fn draw_card_back(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::new(0.08, 0.08, 0.14, 1.0));
    draw_rectangle_lines(x, y, w, h, 1.5, Color::new(0.35, 0.18, 0.45, 1.0));
    // Simple cross-hatch pattern
    let steps = 6;
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        draw_line(x, y + h * t, x + w, y + h * (1.0 - t), 0.5, Color::new(0.3, 0.15, 0.4, 0.4));
    }
}

fn draw_hero(hero: &crate::types::Hero, x: f32, y: f32, is_self: bool) {
    let color = if is_self { COL_SELF } else { COL_ENEMY };
    draw_rectangle(x, y, HERO_W, HERO_H, color);
    draw_rectangle_lines(x, y, HERO_W, HERO_H, 2.0, WHITE);

    let hp = hero.defence.to_string();
    let dims = measure_text(&hp, None, 24, 1.0);
    draw_text(&hp, x + HERO_W / 2.0 - dims.width / 2.0, y + HERO_H / 2.0 + 8.0, 24.0, WHITE);

    draw_text("HP", x + 4.0, y + HERO_H - 5.0, 11.0, LIGHTGRAY);
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
    let label = format!("{}/{}", current, max);
    draw_text(&label, x, y + r * 2.0 + 14.0, 13.0, LIGHTGRAY);
}

fn draw_deck_count(count: usize, x: f32, y: f32) {
    draw_rectangle(x, y, 38.0, 38.0, Color::new(0.18, 0.18, 0.28, 1.0));
    draw_rectangle_lines(x, y, 38.0, 38.0, 1.5, GRAY);
    draw_text_centered(&count.to_string(), x + 19.0, y + 25.0, 18.0, WHITE);
}

fn draw_stat_badge(val: i32, x: f32, y: f32, color: Color) {
    draw_rectangle(x - 2.0, y - 13.0, 20.0, 18.0, color);
    draw_rectangle_lines(x - 2.0, y - 13.0, 20.0, 18.0, 1.0, BLACK);
    let s = val.to_string();
    let d = measure_text(&s, None, 13, 1.0);
    draw_text(&s, x + 8.0 - d.width / 2.0, y, 13.0, WHITE);
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

// ── Helpers ────────────────────────────────────────────────────────────────────

fn draw_text_centered(text: &str, cx: f32, cy: f32, size: f32, color: Color) {
    let d = measure_text(text, None, size as u16, 1.0);
    draw_text(text, cx - d.width / 2.0, cy, size, color);
}

/// Strip HTML tags from description strings.
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

/// Truncate text to fit within `max_w` pixels at the given font size.
fn fit_text(s: &str, max_w: f32, size: f32) -> String {
    let d = measure_text(s, None, size as u16, 1.0);
    if d.width <= max_w {
        return s.to_string();
    }
    let mut result = String::new();
    for ch in s.chars() {
        let candidate = format!("{}{}…", result, ch);
        let cd = measure_text(&candidate, None, size as u16, 1.0);
        if cd.width > max_w {
            return format!("{}…", result);
        }
        result.push(ch);
    }
    result
}
