use macroquad::prelude::*;
use std::collections::HashSet;

use crate::deckbuilder::{card_cost, card_image_url, card_meta, DeckBuilderState};
use crate::layout::{self, CARD_GAP, CARD_H, CARD_W, HERO_H, HERO_W};
use crate::textures::{draw_texture_cover, TextureCache};
use crate::glow;
use crate::ui;
use shared::types::{Board, Card, CardEntity, CardHint, Color as CardColor, GameStateClient, MinionEntity, OmenCard, OmenEntity, ScenarioFrameInfo};


// mana gem as a fraction of card height, stat badge as a multiple of its base size
const GEM_HAND: f32 = 0.14;
const GEM_BOARD: f32 = 0.18;
const BADGE_HAND: f32 = 1.2;
const BADGE_BOARD: f32 = 1.5;

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
const COL_TARGET: Color = Color::new(0.0, 1.0, 0.4, 0.55);
const COL_DRAG_SHADOW: Color = Color::new(0.0, 0.0, 0.0, 0.35);
const COL_OMEN: Color = Color::new(0.62, 0.45, 0.85, 0.95);
const COL_PANEL: Color = Color::new(0.06, 0.05, 0.10, 0.92);


// ── Public entry points ────────────────────────────────────────────────────────

pub struct DragRender<'a> {
    pub card: Option<&'a CardEntity>,   // hand card being dragged
    pub minion_id: Option<u32>,         // battlefield minion being dragged for attack
    pub mx: f32,
    pub my: f32,
    pub targetable_ids: HashSet<u32>,
    pub trade_drop_active: bool,
    pub hovered_id: Option<u32>,
    pub anim_offsets: Vec<(u32, Vec2)>, // (entity_id, pixel offset) for bump anims
    pub hand_flips: Vec<(u32, f32)>,    // (entity_id, cos(rotation_y)) for draw-from-deck flip
}

pub fn draw_game(state: &GameStateClient, cache: &TextureCache, drag: &DragRender) {
    let w = ui::size().x;
    let h = ui::size().y;
    let mid = h / 2.0;

    draw_line(0.0, mid, w, mid, 1.5, COL_DIVIDER);
    draw_turn_indicator(state, w, mid);

    draw_board_half(&state.enemy_board, w, h, false, cache, false, &drag.anim_offsets, &drag.hand_flips, state.open_cards, &[]);
    draw_board_half(&state.self_board, w, h, true, cache, drag.trade_drop_active, &drag.anim_offsets, &drag.hand_flips, state.open_cards, &state.hand_hints);

    // Debug: hand zone boundary
    let hz = layout::hand_zone_rect(w, h);
    draw_rectangle_lines(hz.x, hz.y, hz.w, hz.h, 1.0, Color::new(1.0, 1.0, 0.0, 0.4));

    if state.your_turn {
        draw_end_turn_button(w, mid);
        draw_drop_targets(state, drag, w, h);
    }

    if let Some(hid) = drag.hovered_id {
        draw_entity_preview(hid, state, cache, w, h);
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
    let w = ui::size().x;
    let h = ui::size().y;

    let title = "Choose cards to replace";
    let dims = ui::measure(title, 28.0);
    ui::text(title, w / 2.0 - dims.width / 2.0, 60.0, 28.0, WHITE);

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
            let xd = ui::measure("X", 36.0);
            ui::text("X", x + full_w / 2.0 - xd.width / 2.0, card_y + full_h / 2.0 + 12.0, 36.0, RED);
        }

        let hint = format!("[{}]", i + 1);
        let hd = ui::measure(&hint, 14.0);
        ui::text(&hint, x + full_w / 2.0 - hd.width / 2.0, card_y + full_h + 16.0, 14.0, GRAY);
    }

    if !state.mulligan_submitted {
        draw_button("Confirm  [Enter]", w / 2.0 - 90.0, h - 80.0, 180.0, 44.0);
    } else {
        let msg = "Waiting for opponent...";
        let md = ui::measure(msg, 18.0);
        ui::text(msg, w / 2.0 - md.width / 2.0, h - 60.0, 18.0, GRAY);
    }
}

pub fn draw_lobby(username: &str) {
    let w = ui::size().x;
    let h = ui::size().y;

    let title = "pejsesten";
    let td = ui::measure(title, 52.0);
    ui::text(title, w / 2.0 - td.width / 2.0, h / 3.0, 52.0, WHITE);

    let user_label = format!("Playing as: {}", username);
    let ud = ui::measure(&user_label, 18.0);
    ui::text(&user_label, w / 2.0 - ud.width / 2.0, h / 3.0 + 44.0, 18.0, LIGHTGRAY);

    draw_button("Play vs Human  [H]", w / 2.0 - 115.0, h / 2.0, 230.0, 46.0);
    draw_button("Play vs Bot    [B]", w / 2.0 - 115.0, h / 2.0 + 62.0, 230.0, 46.0);
    draw_button_danger("Reset Server  [R]", w / 2.0 - 115.0, h / 2.0 + 130.0, 230.0, 36.0);
    draw_button("Deck Builder  [D]", w / 2.0 - 115.0, h / 2.0 + 180.0, 230.0, 46.0);
    draw_button("Card Flip Test  [T]", w / 2.0 - 115.0, h / 2.0 + 242.0, 230.0, 46.0);
    draw_button("Run Scenarios  [S]", w / 2.0 - 115.0, h / 2.0 + 304.0, 230.0, 46.0);
    draw_button("Glow Test  [G]", w / 2.0 - 115.0, h / 2.0 + 366.0, 230.0, 46.0);
}

/// Side-by-side glow states over the live config, so the config file can be tuned by eye.
pub fn draw_glow_test(cards: &[CardEntity], cache: &TextureCache, reloads: u32) {
    let w = ui::size().x;
    let h = ui::size().y;
    let cfg = glow::current();

    let title = "Glow Test";
    let td = ui::measure(title, 34.0);
    ui::text(title, w / 2.0 - td.width / 2.0, 62.0, 34.0, WHITE);

    let hint = format!("editing {} - {} reloads - [Esc] back", glow::FILE, reloads);
    let hd = ui::measure(&hint, 16.0);
    ui::text(&hint, w / 2.0 - hd.width / 2.0, 88.0, 16.0, GRAY);

    let cw = CARD_W * 1.6;
    let ch = CARD_H * 1.6;
    let gap = 70.0;
    let columns: [(&str, Option<glow::Style>); 3] = [
        ("no hint", None),
        ("playable", Some(cfg.playable)),
        ("condition met", Some(cfg.condition)),
    ];
    let total = columns.len() as f32 * cw + (columns.len() - 1) as f32 * gap;
    let start_x = w / 2.0 - total / 2.0;
    let y = h / 2.0 - ch / 2.0;

    for (i, (label, style)) in columns.iter().enumerate() {
        let x = start_x + i as f32 * (cw + gap);
        if let Some(style) = style {
            draw_glow(&cfg, style, x, y, cw, ch, i as f32 * cfg.phase_step, 1.0);
        }
        match cards.get(i) {
            Some(card) => draw_card(card, x, y, cw, ch, cache, 1.0),
            None => draw_card_back(x, y, cw, ch),
        }
        let ld = ui::measure(label, 18.0);
        ui::text(label, x + cw / 2.0 - ld.width / 2.0, y + ch + 30.0, 18.0, LIGHTGRAY);
    }

    let lines = [
        format!("rings        {}", cfg.rings),
        format!("spread       {:.1}", cfg.spread),
        format!("inset        {:.1}", cfg.inset),
        format!("thickness    {:.1}", cfg.thickness),
        format!("falloff      {:.2}", cfg.falloff),
        format!("pulse_floor  {:.2}", cfg.pulse_floor),
        format!("breathe      {:.2}", cfg.breathe),
        format!("phase_step   {:.2}", cfg.phase_step),
    ];
    for (i, line) in lines.iter().enumerate() {
        ui::text(line, 28.0, h - 28.0 - (lines.len() - 1 - i) as f32 * 20.0, 16.0, GRAY);
    }
}

/// Narration strip for a scenario playback frame, drawn over the board.
pub fn draw_scenario_banner(info: &ScenarioFrameInfo) {
    let w = ui::size().x;
    let bar_h = 68.0;
    draw_rectangle(0.0, 0.0, w, bar_h, Color::new(0.05, 0.05, 0.08, 0.88));

    let accent = if info.ok { Color::new(0.35, 0.80, 0.45, 1.0) } else { Color::new(0.90, 0.30, 0.30, 1.0) };
    draw_rectangle(0.0, bar_h - 3.0, w, 3.0, accent);

    let counter = format!("{}/{}", info.case_index, info.case_total);
    ui::text(&counter, 16.0, 26.0, 18.0, GRAY);

    let heading = format!("[{}] {}", info.group, info.case);
    ui::text(&heading, 76.0, 26.0, 20.0, WHITE);

    let tick = if info.kind == "check" { if info.ok { "PASS  " } else { "FAIL  " } } else { "" };
    ui::text(&format!("{}{}", tick, info.label), 76.0, 52.0, 18.0, if info.kind == "check" { accent } else { LIGHTGRAY });

    if info.failed > 0 {
        let fail = format!("{} failed", info.failed);
        let d = ui::measure(&fail, 18.0);
        ui::text(&fail, w - d.width - 16.0, 26.0, 18.0, Color::new(0.90, 0.30, 0.30, 1.0));
    }

    if info.done {
        let msg = "scenarios complete — Esc to return to the lobby";
        let d = ui::measure(msg, 18.0);
        ui::text(msg, w - d.width - 16.0, 52.0, 18.0, LIGHTGRAY);
    }
}

pub fn draw_connecting() {
    let w = ui::size().x;
    let h = ui::size().y;
    let text = "Connecting...";
    let dims = ui::measure(text, 28.0);
    ui::text(text, w / 2.0 - dims.width / 2.0, h / 2.0, 28.0, GRAY);
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

fn draw_board_half(board: &Board, w: f32, h: f32, is_self: bool, cache: &TextureCache, deck_glow: bool, anim_offsets: &[(u32, Vec2)], hand_flips: &[(u32, f32)], open_cards: bool, hand_hints: &[CardHint]) {
    let mid = h / 2.0;
    if is_self {
        let hand_y = h - CARD_H - 10.0;
        let hero_y = mid + 18.0;
        let mana_y = h - 22.0;
        let deck_r = layout::self_deck_rect(w, h);

        draw_hero(&board.hero, w - 100.0, hero_y, true);
        draw_battlefield(&board.battlefield, w, h, true, cache, anim_offsets);
        draw_omen_row(&board.omens, &layout::self_omen_rects(board.omens.len(), h), cache);
        draw_hand(&board.hand, w, hand_y, true, cache, hand_flips, hand_hints);
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
        draw_omen_row(&board.omens, &layout::enemy_omen_rects(board.omens.len()), cache);
        draw_hand(&board.hand, w, hand_y, open_cards, cache, hand_flips, hand_hints);
        draw_mana(board.mana, board.base_mana, 20.0, mana_y);
        draw_embers(board.embers, 20.0, mana_y + 32.0);
        draw_deck(board.deck.len(), deck_r.x, deck_r.y, false);
    }
}

// ── Card rendering ─────────────────────────────────────────────────────────────

/// Windows of the 136x192 card frame template, as fractions of the card rect.
struct CardFrame {
    title: Rect,
    art: Rect,
    type_bar: Rect,
    text: Rect,
}

impl CardFrame {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        let win = |fx: f32, fy: f32, fw: f32, fh: f32| {
            Rect::new(x + w * fx, y + h * fy, w * fw, h * fh)
        };
        Self {
            title: win(0.0515, 0.0260, 0.8971, 0.0885),
            art: win(0.0809, 0.1198, 0.8382, 0.4375),
            type_bar: win(0.0515, 0.5573, 0.8971, 0.0833),
            text: win(0.0662, 0.6406, 0.8676, 0.2865),
        }
    }
}

/// The template's art window is transparent, so the art is drawn under the frame.
fn draw_card_frame(
    x: f32, y: f32, w: f32, h: f32,
    color: &CardColor, image_url: &str, cache: &TextureCache, tint: Color,
) -> CardFrame {
    let frame = CardFrame::new(x, y, w, h);
    let tex = cache.frame(color);

    if tex.is_none() {
        let bg = match color {
            CardColor::White => Color::new(0.75, 0.70, 0.55, 1.0),
            CardColor::Black => Color::new(0.18, 0.14, 0.22, 1.0),
        };
        draw_rectangle(x, y, w, h, bg);
    }

    let art = frame.art;
    match cache.art(image_url) {
        Some(tex) => draw_texture_cover(tex, art.x, art.y, art.w, art.h, tint),
        None => draw_rectangle(art.x, art.y, art.w, art.h, COL_CARD_BG),
    }

    if let Some(tex) = tex {
        draw_texture_ex(tex, x, y, tint, DrawTextureParams {
            dest_size: Some(Vec2::new(w, h)),
            ..Default::default()
        });
    }

    frame
}

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
        CardEntity::Omen(o) => (
            &o.card.color,
            o.card.image_url.as_str(),
            o.card.name.as_str(),
            o.cost,
            o.card.description.as_deref().unwrap_or(""),
        ),
    };

    draw_card_layers(sx, y, scaled_w, h, color, image_url, name, cost, description, cache, WHITE, scale, GEM_HAND, true);

    if let CardEntity::Minion(m) = card {
        let badge = scale * BADGE_HAND;
        draw_stat_badge(m.attack, sx + 2.0 + 2.0 * badge, y + h - 2.0 - 5.0 * badge, COL_ATK, badge);
        draw_stat_badge(m.defence, sx + scaled_w - 2.0 - 18.0 * badge, y + h - 2.0 - 5.0 * badge, COL_DEF, badge);
    } else if scale > 0.6 {
        let sl = if matches!(card, CardEntity::Omen(_)) { "omen" } else { "incantation" };
        let font = 10.0 * scale;
        let sd = ui::measure(sl, font);
        ui::text(sl, sx + scaled_w / 2.0 - sd.width / 2.0, y + h - 6.0, font, DARKGRAY);
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
        GEM_BOARD,
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

    draw_stat_badge(minion.attack, x + 2.0 + 2.0 * BADGE_BOARD, y + h - 2.0 - 5.0 * BADGE_BOARD, COL_ATK, BADGE_BOARD);
    draw_stat_badge(minion.defence, x + w - 2.0 - 18.0 * BADGE_BOARD, y + h - 2.0 - 5.0 * BADGE_BOARD, COL_DEF, BADGE_BOARD);
}

fn draw_card_layers(
    x: f32, y: f32, w: f32, h: f32,
    color: &CardColor, image_url: &str, name: &str, cost: i32, description: &str,
    cache: &TextureCache, tint: Color,
    scale: f32,
    gem_frac: f32,
    show_text: bool,
) {
    let frame = draw_card_frame(x, y, w, h, color, image_url, cache, tint);

    // gem overhangs the title bar into the art window
    let gem = (h * gem_frac).min(w * 0.4);
    draw_rectangle(frame.title.x, frame.title.y, gem, gem, COL_MANA);
    if scale <= 0.6 {
        return;
    }

    let cost_font = (gem * 0.7).round();
    draw_text_centered(&cost.to_string(), frame.title.x + gem / 2.0, frame.title.y + gem * 0.72, cost_font, WHITE);
    if !show_text {
        return;
    }

    let name_font = (11.0 * scale).round();
    let name_str = fit_text(name, (frame.title.w - gem - 4.0).max(0.0), name_font);
    ui::text(&name_str, frame.title.x + gem + 3.0, frame.title.y + frame.title.h * 0.78, name_font, BLACK);

    let desc_font = (frame.text.h * 0.20 * scale).round().max(8.0);
    draw_wrapped_text(
        &strip_html(description),
        frame.text.x + 3.0,
        frame.text.y + desc_font,
        (frame.text.w - 6.0).max(0.0),
        desc_font,
        BLACK,
    );
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

fn draw_entity_preview(entity_id: u32, state: &GameStateClient, cache: &TextureCache, w: f32, h: f32) {
    let pw = PREVIEW_W;
    let ph = PREVIEW_H;

    // Hand cards: preview floats above the hovered card so it doesn't overlap the hand row.
    let hand_rects = layout::hand_rects(state.self_board.hand.len(), w, h);
    if let Some((card, rect)) = state.self_board.hand.iter().zip(hand_rects.iter())
        .find(|(c, _)| c.entity_id() == entity_id)
        .map(|(c, r)| (c, *r))
    {
        let px = (rect.x + rect.w / 2.0 - pw / 2.0).clamp(0.0, w - pw);
        // an omen hangs its trigger panel below the card, so the whole block has to clear the hand
        let panel = match card {
            CardEntity::Omen(o) => omen_panel_height(o) + 6.0,
            _ => 0.0,
        };
        let py = (rect.y - ph - panel - 10.0).max(0.0);
        draw_rectangle(px + 5.0, py + 8.0, pw, ph, Color::new(0.0, 0.0, 0.0, 0.55));
        draw_card_entity_preview(card, px, py, pw, ph, cache);
        return;
    }

    // Omens: the trigger panel hangs under the card, so the block is anchored to the side.
    let omen = {
        let rects = layout::self_omen_rects(state.self_board.omens.len(), h);
        state.self_board.omens.iter().zip(rects)
            .find(|(o, _)| o.entity_id == entity_id)
            .map(|(o, r)| (o, r))
    }.or_else(|| {
        let rects = layout::enemy_omen_rects(state.enemy_board.omens.len());
        state.enemy_board.omens.iter().zip(rects)
            .find(|(o, _)| o.entity_id == entity_id)
            .map(|(o, r)| (o, r))
    });

    if let Some((omen, rect)) = omen {
        let block_h = ph + 6.0 + omen_panel_height(omen);
        let px = rect.x + rect.w + 10.0;
        let py = (rect.y + rect.h / 2.0 - block_h / 2.0).clamp(0.0, h - block_h);
        draw_rectangle(px + 5.0, py + 8.0, pw, ph, Color::new(0.0, 0.0, 0.0, 0.55));
        draw_omen_preview(omen, px, py, pw, ph, cache);
        return;
    }

    // Battlefield minions: preview floats to the side of the minion.
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

    let px = if rect.x + rect.w + pw + 10.0 <= w {
        rect.x + rect.w + 10.0
    } else {
        rect.x - pw - 10.0
    };
    let py = (rect.y + rect.h / 2.0 - ph / 2.0).clamp(0.0, h - ph);

    draw_rectangle(px + 5.0, py + 8.0, pw, ph, Color::new(0.0, 0.0, 0.0, 0.55));
    draw_card_preview(minion, px, py, pw, ph, cache);
}

fn draw_card_entity_preview(card: &CardEntity, x: f32, y: f32, w: f32, h: f32, cache: &TextureCache) {
    match card {
        CardEntity::Minion(m) => draw_card_preview(m, x, y, w, h, cache),
        CardEntity::Incantation(i) => draw_incantation_preview(i, x, y, w, h, cache),
        CardEntity::Omen(o) => draw_omen_preview(o, x, y, w, h, cache),
    }
}

fn draw_incantation_preview(inc: &shared::types::IncantationEntity, x: f32, y: f32, w: f32, h: f32, cache: &TextureCache) {
    let card = &inc.card;
    let frame = draw_card_frame(x, y, w, h, &card.color, &card.image_url, cache, WHITE);

    let gem = frame.title.h;
    draw_rectangle(frame.title.x, frame.title.y, gem, gem, COL_MANA);
    draw_text_centered(&inc.cost.to_string(), frame.title.x + gem / 2.0, frame.title.y + gem * 0.78, 22.0, WHITE);

    let name_fit = fit_text(&card.name, frame.title.w - gem - 6.0, 18.0);
    ui::text(&name_fit, frame.title.x + gem + 4.0, frame.title.y + frame.title.h * 0.78, 18.0, BLACK);

    let sl = "incantation";
    let sd = ui::measure(sl, 14.0);
    ui::text(sl, frame.type_bar.x + frame.type_bar.w / 2.0 - sd.width / 2.0, frame.type_bar.y + frame.type_bar.h * 0.75, 14.0, DARKGRAY);

    let desc = card.description.as_deref().unwrap_or("");
    let desc_font = (frame.text.h * 0.20).round();
    draw_wrapped_text(&strip_html(desc), frame.text.x + 4.0, frame.text.y + desc_font, frame.text.w - 8.0, desc_font, BLACK);
}

fn draw_omen_row(omens: &[OmenEntity], rects: &[Rect], cache: &TextureCache) {
    for (omen, rect) in omens.iter().zip(rects.iter()) {
        let frame = draw_card_frame(rect.x, rect.y, rect.w, rect.h, &omen.card.color, &omen.card.image_url, cache, WHITE);
        let gem = (rect.h * GEM_BOARD).min(rect.w * 0.4);
        draw_rectangle(frame.title.x, frame.title.y, gem, gem, COL_MANA);
        draw_text_centered(&omen.cost.to_string(), frame.title.x + gem / 2.0, frame.title.y + gem * 0.72, (gem * 0.7).round(), WHITE);
        let name = fit_text(&omen.card.name, frame.text.w, 9.0);
        ui::text(&name, frame.text.x, frame.text.y + 9.0, 9.0, BLACK);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, COL_OMEN);
    }
}

const OMEN_PANEL_W: f32 = 230.0;
const OMEN_PANEL_FONT: f32 = 11.0;
const OMEN_PANEL_LINE: f32 = 15.0;
const OMEN_PANEL_PAD: f32 = 7.0;

/// Your own omen shows what it is armed on; an enemy omen only narrows it to three.
fn omen_panel_lines(omen: &OmenEntity) -> Vec<String> {
    match omen.armed_trigger {
        Some(t) => vec![format!("Armed: {}", t.label())],
        None => {
            let mut lines = vec!["Watching one of:".to_string()];
            lines.extend(omen.card.triggers.iter().map(|t| format!("- {}", t.label())));
            lines
        }
    }
}

fn omen_panel_height(omen: &OmenEntity) -> f32 {
    omen_panel_lines(omen).len() as f32 * OMEN_PANEL_LINE + OMEN_PANEL_PAD * 2.0
}

fn draw_omen_panel(omen: &OmenEntity, x: f32, y: f32) {
    let lines = omen_panel_lines(omen);
    let h = lines.len() as f32 * OMEN_PANEL_LINE + OMEN_PANEL_PAD * 2.0;
    draw_rectangle(x, y, OMEN_PANEL_W, h, COL_PANEL);
    draw_rectangle_lines(x, y, OMEN_PANEL_W, h, 1.5, COL_OMEN);
    for (i, line) in lines.iter().enumerate() {
        let fitted = fit_text(line, OMEN_PANEL_W - OMEN_PANEL_PAD * 2.0, OMEN_PANEL_FONT);
        let color = if i == 0 && omen.armed_trigger.is_none() { GRAY } else { WHITE };
        ui::text(&fitted, x + OMEN_PANEL_PAD, y + OMEN_PANEL_PAD + (i as f32 + 0.8) * OMEN_PANEL_LINE, OMEN_PANEL_FONT, color);
    }
}

fn draw_omen_preview(omen: &OmenEntity, x: f32, y: f32, w: f32, h: f32, cache: &TextureCache) {
    let card = &omen.card;
    let frame = draw_card_frame(x, y, w, h, &card.color, &card.image_url, cache, WHITE);

    let gem = frame.title.h;
    draw_rectangle(frame.title.x, frame.title.y, gem, gem, COL_MANA);
    draw_text_centered(&omen.cost.to_string(), frame.title.x + gem / 2.0, frame.title.y + gem * 0.78, 22.0, WHITE);

    let name_fit = fit_text(&card.name, frame.title.w - gem - 6.0, 18.0);
    ui::text(&name_fit, frame.title.x + gem + 4.0, frame.title.y + frame.title.h * 0.78, 18.0, BLACK);

    let sl = "omen";
    let sd = ui::measure(sl, 14.0);
    ui::text(sl, frame.type_bar.x + frame.type_bar.w / 2.0 - sd.width / 2.0, frame.type_bar.y + frame.type_bar.h * 0.75, 14.0, DARKGRAY);

    let desc = card.description.as_deref().unwrap_or("");
    let desc_font = (frame.text.h * 0.20).round();
    draw_wrapped_text(&strip_html(desc), frame.text.x + 4.0, frame.text.y + desc_font, frame.text.w - 8.0, desc_font, BLACK);

    draw_omen_panel(omen, x, y + h + 6.0);
}

/// Full-screen choice of which of the three printed triggers to arm.
pub fn draw_omen_picker(card: &OmenCard, mx: f32, my: f32, cache: &TextureCache) {
    let w = ui::size().x;
    let h = ui::size().y;
    draw_rectangle(0.0, 0.0, w, h, Color::new(0.0, 0.0, 0.0, 0.72));

    let title = format!("{} - {}", card.name, strip_html(card.description.as_deref().unwrap_or("")).replace('\n', " "));
    let td = ui::measure(&title, 26.0);
    ui::text(&title, w / 2.0 - td.width / 2.0, h / 2.0 - layout::OMEN_PICK_H / 2.0 - 54.0, 26.0, WHITE);

    let sub = "Choose the trigger. Your opponent sees the effect, never the trigger.";
    let sd = ui::measure(sub, 16.0);
    ui::text(sub, w / 2.0 - sd.width / 2.0, h / 2.0 - layout::OMEN_PICK_H / 2.0 - 28.0, 16.0, LIGHTGRAY);

    for (i, rect) in layout::omen_pick_rects(w, h).iter().enumerate() {
        let hovered = rect.contains(Vec2::new(mx, my));
        draw_rectangle(rect.x + 4.0, rect.y + 6.0, rect.w, rect.h, COL_DRAG_SHADOW);
        let frame = draw_card_frame(rect.x, rect.y, rect.w, rect.h, &card.color, &card.image_url, cache, WHITE);

        let gem = frame.title.h;
        draw_rectangle(frame.title.x, frame.title.y, gem, gem, COL_MANA);
        draw_text_centered(&card.base_cost.to_string(), frame.title.x + gem / 2.0, frame.title.y + gem * 0.78, 16.0, WHITE);
        let name_fit = fit_text(&card.name, frame.title.w - gem - 6.0, 13.0);
        ui::text(&name_fit, frame.title.x + gem + 4.0, frame.title.y + frame.title.h * 0.78, 13.0, BLACK);

        let sl = "omen";
        let sd = ui::measure(sl, 11.0);
        ui::text(sl, frame.type_bar.x + frame.type_bar.w / 2.0 - sd.width / 2.0, frame.type_bar.y + frame.type_bar.h * 0.75, 11.0, DARKGRAY);

        let label = card.triggers[i].label();
        draw_wrapped_text(label, frame.text.x + 3.0, frame.text.y + 12.0, frame.text.w - 6.0, 11.0, BLACK);

        if hovered {
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 3.0, GOLD);
        }
    }

    let footer = "[Esc] cancel";
    let fd = ui::measure(footer, 16.0);
    ui::text(footer, w / 2.0 - fd.width / 2.0, h / 2.0 + layout::OMEN_PICK_H / 2.0 + 34.0, 16.0, GRAY);
}

fn draw_card_preview(minion: &MinionEntity, x: f32, y: f32, w: f32, h: f32, cache: &TextureCache) {
    let card = &minion.card;
    let frame = draw_card_frame(x, y, w, h, &card.color, &card.image_url, cache, WHITE);

    let gem = frame.title.h;
    draw_rectangle(frame.title.x, frame.title.y, gem, gem, COL_MANA);
    draw_text_centered(&card.base_cost.to_string(), frame.title.x + gem / 2.0, frame.title.y + gem * 0.78, 22.0, WHITE);

    let name_fit = fit_text(&card.name, frame.title.w - gem - 6.0, 18.0);
    ui::text(&name_fit, frame.title.x + gem + 4.0, frame.title.y + frame.title.h * 0.78, 18.0, BLACK);

    let desc = card.description.as_deref().unwrap_or("");
    let desc_font = (frame.text.h * 0.20).round();
    draw_wrapped_text(&strip_html(desc), frame.text.x + 4.0, frame.text.y + desc_font, frame.text.w - 8.0, desc_font, BLACK);

    draw_stat_badge(minion.attack, x + 8.0, y + h - 5.0, COL_ATK, 1.0);
    draw_stat_badge(minion.defence, x + w - 22.0, y + h - 5.0, COL_DEF, 1.0);
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

fn draw_hand(hand: &[CardEntity], w: f32, y: f32, face_up: bool, cache: &TextureCache, hand_flips: &[(u32, f32)], hints: &[CardHint]) {
    let count = hand.len() as f32;
    let total = count * (CARD_W + CARD_GAP) - CARD_GAP;
    let start_x = w / 2.0 - total / 2.0;
    let cfg = glow::current();

    for (i, card) in hand.iter().enumerate() {
        let x = start_x + i as f32 * (CARD_W + CARD_GAP);
        if face_up {
            let flip_cos = hand_flips.iter()
                .find(|(id, _)| *id == card.entity_id())
                .map(|(_, v)| *v)
                .unwrap_or(1.0);
            if let Some(style) = hints.get(i).and_then(|h| glow_style(&cfg, h)) {
                draw_glow(&cfg, &style, x, y, CARD_W, CARD_H, i as f32 * cfg.phase_step, flip_cos.clamp(0.0, 1.0));
            }
            draw_card(card, x, y, CARD_W, CARD_H, cache, flip_cos);
        } else {
            draw_card_back(x, y, CARD_W, CARD_H);
        }
    }
}

/// A met condition outranks plain affordability: it is the rarer thing to notice.
fn glow_style(cfg: &glow::Config, hint: &CardHint) -> Option<glow::Style> {
    match (hint.condition_met, hint.playable) {
        (true, _) => Some(cfg.condition),
        (false, true) => Some(cfg.playable),
        _ => None,
    }
}

/// Phase spaces the pulse per hand slot; fade tracks the draw flip, since the card
/// is a narrow sliver mid-animation and a full-width aura would frame empty space.
pub fn draw_glow(cfg: &glow::Config, style: &glow::Style, x: f32, y: f32, w: f32, h: f32, phase: f32, fade: f32) {
    let wave = 0.5 + 0.5 * (get_time() as f32 * style.speed + phase).sin();
    let pulse = cfg.pulse_floor + (1.0 - cfg.pulse_floor) * wave;
    let breathe = 1.0 - cfg.breathe + cfg.breathe * pulse;
    let color = style.color();

    // rings overlap so the alpha layers composite into a falloff instead of banding
    for i in 1..=cfg.rings {
        let t = i as f32 / cfg.rings as f32;
        let pad = cfg.inset + t * cfg.spread * breathe;
        let a = style.intensity * pulse * fade * (1.0 - t).powf(cfg.falloff);
        draw_rectangle_lines(x - pad, y - pad, w + pad * 2.0, h + pad * 2.0, cfg.thickness, Color { a, ..color });
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
    let dims = ui::measure(&hp, 24.0);
    ui::text(&hp, x + HERO_W / 2.0 - dims.width / 2.0, y + HERO_H / 2.0 + 8.0, 24.0, WHITE);
    ui::text("HP", x + 4.0, y + HERO_H - 5.0, 11.0, LIGHTGRAY);
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
    ui::text(&format!("{}/{}", current, max), x, y + r * 2.0 + 14.0, 13.0, LIGHTGRAY);
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
        let d = ui::measure(&s, font);
        ui::text(&s, x + 8.0 * scale - d.width / 2.0, y, font, WHITE);
    }
}

fn draw_turn_indicator(state: &GameStateClient, w: f32, mid: f32) {
    let label = if state.your_turn { "YOUR TURN" } else { "OPPONENT'S TURN" };
    let color = if state.your_turn { GREEN } else { GRAY };
    let dims = ui::measure(label, 18.0);
    ui::text(label, w / 2.0 - dims.width / 2.0, mid - 4.0, 18.0, color);
    let turn = format!("Turn {}", state.turn_count);
    let td = ui::measure(&turn, 13.0);
    ui::text(&turn, w / 2.0 - td.width / 2.0, mid + 12.0, 13.0, GRAY);
}

fn draw_end_turn_button(w: f32, mid: f32) {
    let bx = w - 124.0;
    let by = mid + 14.0;
    let bw = 104.0;
    let bh = 36.0;
    draw_rectangle(bx, by, bw, bh, Color::new(0.1, 0.55, 0.18, 1.0));
    draw_rectangle_lines(bx, by, bw, bh, 1.5, GREEN);
    let label = "END TURN [E]";
    let d = ui::measure(label, 14.0);
    ui::text(label, bx + bw / 2.0 - d.width / 2.0, by + bh / 2.0 + 5.0, 14.0, WHITE);
}

pub fn draw_button(label: &str, x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::new(0.18, 0.28, 0.48, 1.0));
    draw_rectangle_lines(x, y, w, h, 1.5, Color::new(0.38, 0.56, 0.82, 1.0));
    let d = ui::measure(label, 17.0);
    ui::text(label, x + w / 2.0 - d.width / 2.0, y + h / 2.0 + 6.0, 17.0, WHITE);
}

pub fn draw_button_danger(label: &str, x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::new(0.35, 0.08, 0.08, 1.0));
    draw_rectangle_lines(x, y, w, h, 1.5, Color::new(0.70, 0.20, 0.20, 1.0));
    let d = ui::measure(label, 15.0);
    ui::text(label, x + w / 2.0 - d.width / 2.0, y + h / 2.0 + 5.0, 15.0, Color::new(0.9, 0.6, 0.6, 1.0));
}

// ── Deck Builder ──────────────────────────────────────────────────────────────

/// Paints the catalog slots that `deckbuilder_ui` laid out this frame.
pub fn draw_catalog_cards(state: &DeckBuilderState, slots: &[(usize, Rect, usize)], cache: &TextureCache) {
    for &(idx, rect, copies) in slots {
        let card = &state.all_cards[idx];
        draw_catalog_card(card, rect.x, rect.y, rect.w, rect.h, cache);
        if copies > 0 {
            let d = 18.0;
            draw_rectangle(rect.right() - d, rect.y, d, d, Color::new(0.10, 0.08, 0.16, 0.92));
            draw_text_centered(&copies.to_string(), rect.right() - d / 2.0, rect.y + d * 0.72, 12.0, WHITE);
        }
    }
}

fn draw_catalog_card(card: &Card, x: f32, y: f32, w: f32, h: f32, cache: &TextureCache) {
    let (name, color, _) = card_meta(card);
    let image_url = card_image_url(card);
    let cost = card_cost(card);

    let frame = draw_card_frame(x, y, w, h, color, image_url, cache, WHITE);

    let gem = frame.title.h;
    draw_rectangle(frame.title.x, frame.title.y, gem, gem, COL_MANA);
    draw_text_centered(&cost.to_string(), frame.title.x + gem / 2.0, frame.title.y + gem * 0.78, 10.0, WHITE);

    let name_fit = fit_text(name, frame.title.w - gem - 4.0, 9.0);
    ui::text(&name_fit, frame.title.x + gem + 3.0, frame.title.y + frame.title.h * 0.78, 9.0, BLACK);

    let label_y = frame.type_bar.y + frame.type_bar.h * 0.8;
    if let Card::Minion(m) = card {
        draw_text_centered(&m.base_attack.to_string(), frame.type_bar.x + 8.0, label_y, 10.0, Color::new(0.8, 0.6, 0.1, 1.0));
        draw_text_centered(&m.base_defence.to_string(), frame.type_bar.right() - 8.0, label_y, 10.0, Color::new(0.8, 0.25, 0.25, 1.0));
    } else {
        let sl = if matches!(card, Card::Omen(_)) { "omen" } else { "spell" };
        let sd = ui::measure(sl, 8.0);
        ui::text(sl, frame.type_bar.x + frame.type_bar.w / 2.0 - sd.width / 2.0, label_y, 8.0, DARKGRAY);
    }
}

// ── Helpers ────────────────────────────────────────────────────────────────────

fn draw_text_centered(text: &str, cx: f32, cy: f32, size: f32, color: Color) {
    let d = ui::measure(text, size);
    ui::text(text, cx - d.width / 2.0, cy, size, color);
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
    for paragraph in text.split('\n') {
        let mut line = String::new();
        for word in paragraph.split_whitespace() {
            let candidate = if line.is_empty() { word.to_string() } else { format!("{} {}", line, word) };
            if ui::measure(&candidate, size).width > max_w && !line.is_empty() {
                ui::text(&line, x, y, size, color);
                y += line_h;
                line = word.to_string();
            } else {
                line = candidate;
            }
        }
        ui::text(&line, x, y, size, color);
        y += line_h;
    }
}

fn strip_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut tag = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => {
                in_tag = true;
                tag.clear();
            }
            '>' if in_tag => {
                in_tag = false;
                let t = tag.trim_start_matches('/').trim().to_ascii_lowercase();
                if t == "br" || t == "br/" || t.starts_with("br ") {
                    out.push('\n');
                }
            }
            _ if in_tag => tag.push(c),
            _ => out.push(c),
        }
    }
    out.replace("&nbsp;", " ")
}

fn fit_text(s: &str, max_w: f32, size: f32) -> String {
    if ui::measure(s, size).width <= max_w {
        return s.to_string();
    }
    let mut result = String::new();
    for ch in s.chars() {
        let candidate = format!("{}{}…", result, ch);
        if ui::measure(&candidate, size).width > max_w {
            return format!("{}…", result);
        }
        result.push(ch);
    }
    result
}
