use macroquad::prelude::*;

mod deckbuilder;
mod decks;
mod layout;
mod network;
mod render;
mod textures;
mod types;

use deckbuilder::{card_id, DeckBuilderState, Panel};
use network::{NetworkClient, ServerEvent};
use render::DragRender;
use textures::TextureCache;
use types::{AttackData, GamePhase, GameStateClient, PlayerMetaData};

const SERVER_URL: &str = match option_env!("SERVER_URL") {
    Some(url) => url,
    None => "ws://localhost:3000",
};

const USERNAME: &str = match option_env!("USERNAME") {
    Some(u) => u,
    None => "player",
};

fn default_deck() -> Vec<u32> {
    (1u32..=30).flat_map(|id| [id, id]).collect()
}

enum Screen {
    Connecting,
    Lobby,
    Mulligan { state: GameStateClient, selected: Vec<usize> },
    Playing(GameStateClient),
    DeckBuilder(DeckBuilderState),
}

enum DragState {
    Card { index: usize },
    Minion { entity_id: u32 },
}

fn window_conf() -> Conf {
    Conf {
        window_title: "pejsesten".to_string(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut net = NetworkClient::new(SERVER_URL);
    let mut cache = TextureCache::new();
    let mut screen = Screen::Connecting;
    let mut connected = false;
    let mut drag: Option<DragState> = None;

    loop {
        let (mx, my) = mouse_position();
        let w = screen_width();
        let h = screen_height();

        // --- Network ---
        while let Some(event) = net.poll() {
            match event {
                ServerEvent::GameState(state) => {
                    cache.preload_for_state(&state).await;
                    screen = match state.phase {
                        GamePhase::Mulligan => {
                            let selected = match &screen {
                                Screen::Mulligan { selected, .. } => selected.clone(),
                                _ => vec![],
                            };
                            Screen::Mulligan { state, selected }
                        }
                        GamePhase::Playing => Screen::Playing(state),
                    };
                }
                ServerEvent::Redirect(_) => {}
            }
        }

        if matches!(screen, Screen::Connecting) && !connected {
            connected = true;
            screen = Screen::Lobby;
        }

        // Preload catalog textures once on entering deck builder
        if let Screen::DeckBuilder(db) = &mut screen {
            if !db.textures_preloaded && !db.all_cards.is_empty() {
                let cards = db.all_cards.clone();
                cache.preload_cards(&cards).await;
                db.textures_preloaded = true;
            }
        }

        // --- Input ---
        handle_input(&mut screen, &mut net, &mut drag, mx, my, w, h);

        // --- Render ---
        clear_background(Color::from_rgba(12, 12, 20, 255));

        match &screen {
            Screen::Connecting => render::draw_connecting(),
            Screen::Lobby => render::draw_lobby(USERNAME),
            Screen::Mulligan { state, selected } => render::draw_mulligan(state, selected, &cache),
            Screen::Playing(state) => {
                let drag_render = make_drag_render(&drag, state, mx, my);
                render::draw_game(state, &cache, &drag_render);
            }
            Screen::DeckBuilder(db_state) => render::draw_deck_builder(db_state, &cache),
        }

        next_frame().await;
    }
}

fn make_drag_render<'a>(drag: &'a Option<DragState>, state: &'a GameStateClient, mx: f32, my: f32) -> DragRender<'a> {
    match drag {
        Some(DragState::Card { index }) => DragRender {
            card: state.self_board.hand.get(*index),
            minion_id: None,
            mx,
            my,
        },
        Some(DragState::Minion { entity_id }) => DragRender {
            card: None,
            minion_id: Some(*entity_id),
            mx,
            my,
        },
        None => DragRender { card: None, minion_id: None, mx, my },
    }
}

fn handle_input(
    screen: &mut Screen,
    net: &mut NetworkClient,
    drag: &mut Option<DragState>,
    mx: f32, my: f32,
    w: f32, h: f32,
) {
    match screen {
        Screen::Lobby => {
            let clicked = is_mouse_button_released(MouseButton::Left);
            let mouse = Vec2::new(mx, my);

            if is_key_pressed(KeyCode::H) || (clicked && layout::lobby_queue_human_rect(w, h).contains(mouse)) {
                net.queue_up(&PlayerMetaData {
                    username: USERNAME.to_string(),
                    chosen_deck: default_deck(),
                    avatar: String::new(),
                });
            }
            if is_key_pressed(KeyCode::B) || (clicked && layout::lobby_queue_bot_rect(w, h).contains(mouse)) {
                net.queue_up_bot(&PlayerMetaData {
                    username: USERNAME.to_string(),
                    chosen_deck: default_deck(),
                    avatar: String::new(),
                });
            }
            if is_key_pressed(KeyCode::R) || (clicked && layout::lobby_reset_rect(w, h).contains(mouse)) {
                net.reset_server();
            }
            if is_key_pressed(KeyCode::D) || (clicked && layout::lobby_deck_builder_rect(w, h).contains(mouse)) {
                *screen = Screen::DeckBuilder(DeckBuilderState::new(SERVER_URL));
            }
        }

        Screen::Mulligan { state, selected } => {
            let hand_len = state.self_board.hand.len();
            let mouse = Vec2::new(mx, my);
            let clicked = is_mouse_button_released(MouseButton::Left);

            // Click a card to toggle selection
            if clicked {
                let rects = layout::mulligan_card_rects(hand_len, w, h);
                if let Some(idx) = rects.iter().position(|r| r.contains(mouse)) {
                    if let Some(pos) = selected.iter().position(|&s| s == idx) {
                        selected.remove(pos);
                    } else {
                        selected.push(idx);
                    }
                }
            }

            // Keyboard shortcuts still work
            for (key, idx) in [
                (KeyCode::Key1, 0), (KeyCode::Key2, 1), (KeyCode::Key3, 2),
                (KeyCode::Key4, 3), (KeyCode::Key5, 4), (KeyCode::Key6, 5),
                (KeyCode::Key7, 6),
            ] {
                if is_key_pressed(key) && idx < hand_len {
                    if let Some(pos) = selected.iter().position(|&s| s == idx) {
                        selected.remove(pos);
                    } else {
                        selected.push(idx);
                    }
                }
            }

            let confirm = is_key_pressed(KeyCode::Enter)
                || (clicked && layout::mulligan_confirm_rect(w, h).contains(mouse));
            if confirm && !state.mulligan_submitted {
                let indices: Vec<usize> = selected.drain(..).collect();
                net.submit_mulligan(&indices);
            }
        }

        Screen::Playing(state) => {
            if !state.your_turn {
                return;
            }

            let clicked = is_mouse_button_released(MouseButton::Left) && drag.is_none();
            if is_key_pressed(KeyCode::E) || (clicked && layout::end_turn_rect(w, h).contains(Vec2::new(mx, my))) {
                net.end_turn();
                return;
            }

            let mouse = Vec2::new(mx, my);

            // Start drag
            if is_mouse_button_pressed(MouseButton::Left) && drag.is_none() {
                // Check hand cards first
                let hand_rects = layout::hand_rects(state.self_board.hand.len(), w, h);
                if let Some(idx) = hand_rects.iter().position(|r| r.contains(mouse)) {
                    *drag = Some(DragState::Card { index: idx });
                } else {
                    // Check own battlefield minions
                    let minion_rects = layout::self_minion_rects(state.self_board.battlefield.len(), w, h);
                    if let Some(idx) = minion_rects.iter().position(|r| r.contains(mouse)) {
                        let minion = &state.self_board.battlefield[idx];
                        if !minion.exhausted {
                            *drag = Some(DragState::Minion { entity_id: minion.entity_id });
                        }
                    }
                }
            }

            // Release drag → resolve action
            if is_mouse_button_released(MouseButton::Left) {
                if let Some(d) = drag.take() {
                    match d {
                        DragState::Card { index } => {
                            // Only play if dropped clearly outside the hand zone
                            if !layout::hand_zone_rect(w, h).contains(mouse) {
                                let target = find_target(mouse, state, w, h);
                                net.play_card(index, target);
                            }
                        }
                        DragState::Minion { entity_id } => {
                            if let Some(target_id) = find_enemy_target(mouse, state, w, h) {
                                net.attack(&AttackData { origin_id: entity_id, target_id });
                            }
                        }
                    }
                }
            }

            // Cancel drag with right click or Escape
            if is_mouse_button_pressed(MouseButton::Right) || is_key_pressed(KeyCode::Escape) {
                *drag = None;
            }
        }

        Screen::DeckBuilder(_) => {
            if let Screen::DeckBuilder(db) = screen {
                let action = handle_deck_builder_input(db, mx, my, w, h);
                match action {
                    DeckBuilderAction::GoLobby => { *screen = Screen::Lobby; }
                    DeckBuilderAction::None => {}
                }
            }
        }

        _ => {}
    }
}

enum DeckBuilderAction { None, GoLobby }

fn handle_deck_builder_input(
    db: &mut DeckBuilderState,
    mx: f32, my: f32,
    w: f32, h: f32,
) -> DeckBuilderAction {
    let clicked = is_mouse_button_released(MouseButton::Left);
    let mouse = Vec2::new(mx, my);
    db.tick_clipboard(get_frame_time());

    // Scroll
    let scroll_delta = mouse_wheel().1;
    if mouse.x < render::DB_CATALOG_W {
        let row_h = render::DB_CARD_H + render::DB_CARD_GAP;
        let area_h = h - render::DB_FILTER_H - 8.0;
        let total_rows = (db.filtered.len() + render::DB_COLS - 1) / render::DB_COLS;
        let max_scroll = (total_rows as f32 * row_h - area_h).max(0.0);
        db.catalog_scroll = (db.catalog_scroll - scroll_delta * 30.0).clamp(0.0, max_scroll);
    } else {
        let list_h = match &db.panel {
            Panel::DeckList => h - 40.0 - 4.0 * (render::DB_ROW_H + 4.0) - 8.0,
            Panel::Editor { .. } => h - 44.0 - 3.0 * (render::DB_ROW_H + 4.0) - 8.0,
        };
        let count = match &db.panel {
            Panel::DeckList => db.decks.len(),
            Panel::Editor { .. } => db.unique_editing().len(),
        };
        let max_scroll = (count as f32 * (render::DB_ROW_H + 4.0) - list_h).max(0.0);
        db.deck_scroll = (db.deck_scroll - scroll_delta * 30.0).clamp(0.0, max_scroll);
    }

    // Text input
    let editing = matches!(db.panel, Panel::Editor { .. });
    while let Some(ch) = get_char_pressed() {
        if editing {
            if ch.is_alphanumeric() || ch == ' ' || ch == '\'' || ch == '-' {
                if let Panel::Editor { name, .. } = &mut db.panel {
                    if name.len() < 36 { name.push(ch); }
                }
            }
        } else if ch.is_alphanumeric() || ch == ' ' {
            db.search.push(ch);
            db.apply_filter();
        }
    }
    if is_key_pressed(KeyCode::Backspace) {
        if editing {
            if let Panel::Editor { name, .. } = &mut db.panel { name.pop(); }
        } else {
            db.search.pop();
            db.apply_filter();
        }
    }

    // Escape
    if is_key_pressed(KeyCode::Escape) {
        return match &db.panel {
            Panel::DeckList => DeckBuilderAction::GoLobby,
            Panel::Editor { .. } => { db.panel = Panel::DeckList; DeckBuilderAction::None }
        };
    }

    // Filter bar
    let bar_y = h - render::DB_FILTER_H - 4.0;
    if clicked {
        let tx = 240.0;
        if Rect::new(tx, bar_y + 5.0, 46.0, 28.0).contains(mouse) {
            db.filter_type = deckbuilder::FilterType::All; db.apply_filter();
        }
        if Rect::new(tx + 50.0, bar_y + 5.0, 68.0, 28.0).contains(mouse) {
            db.filter_type = deckbuilder::FilterType::Minions; db.apply_filter();
        }
        if Rect::new(tx + 122.0, bar_y + 5.0, 60.0, 28.0).contains(mouse) {
            db.filter_type = deckbuilder::FilterType::Incantations; db.apply_filter();
        }
        let cx2 = 410.0;
        if Rect::new(cx2, bar_y + 5.0, 36.0, 28.0).contains(mouse) {
            db.filter_color = deckbuilder::ColorFilter::All; db.apply_filter();
        }
        if Rect::new(cx2 + 40.0, bar_y + 5.0, 28.0, 28.0).contains(mouse) {
            db.filter_color = deckbuilder::ColorFilter::White; db.apply_filter();
        }
        if Rect::new(cx2 + 72.0, bar_y + 5.0, 28.0, 28.0).contains(mouse) {
            db.filter_color = deckbuilder::ColorFilter::Black; db.apply_filter();
        }
        if Rect::new(render::DB_CATALOG_W - 124.0, bar_y + 5.0, 120.0, 28.0).contains(mouse) {
            return DeckBuilderAction::GoLobby;
        }
    }

    // Catalog click → add card (only when editing a deck)
    if clicked && matches!(db.panel, Panel::Editor { .. })
        && mouse.x < render::DB_CATALOG_W && mouse.y < bar_y
    {
        let card_step = render::DB_CARD_W + render::DB_CARD_GAP;
        let start_x = (render::DB_CATALOG_W - render::DB_COLS as f32 * card_step + render::DB_CARD_GAP) / 2.0;
        let row_h = render::DB_CARD_H + render::DB_CARD_GAP;
        let filtered_copy: Vec<usize> = db.filtered.clone();
        for (grid_i, card_idx) in filtered_copy.iter().enumerate() {
            let col = grid_i % render::DB_COLS;
            let row = grid_i / render::DB_COLS;
            let cx = start_x + col as f32 * card_step;
            let cy = row as f32 * row_h + 4.0 - db.catalog_scroll;
            if Rect::new(cx, cy, render::DB_CARD_W, render::DB_CARD_H).contains(mouse) {
                let id = card_id(&db.all_cards[*card_idx]);
                db.add_card(id);
                break;
            }
        }
    }

    // Right panel
    let px = render::DB_CATALOG_W + 6.0;
    let pw = w - px - 6.0;

    match &db.panel {
        Panel::DeckList => {
            let list_y_start = 40.0;
            let scroll = db.deck_scroll;
            let btn_y_base = h - 4.0 * (render::DB_ROW_H + 4.0) - 4.0;

            if clicked {
                let deck_ids: Vec<u64> = db.decks.iter().map(|d| d.id).collect();
                for (i, &did) in deck_ids.iter().enumerate() {
                    let ry = list_y_start + i as f32 * (render::DB_ROW_H + 3.0) - scroll;
                    if Rect::new(px, ry, pw, render::DB_ROW_H).contains(mouse) {
                        db.open_deck(did);
                        return DeckBuilderAction::None;
                    }
                }
                if Rect::new(px, btn_y_base, pw, render::DB_ROW_H).contains(mouse) {
                    db.open_new_deck(); return DeckBuilderAction::None;
                }
                if Rect::new(px, btn_y_base + render::DB_ROW_H + 4.0, pw, render::DB_ROW_H).contains(mouse) {
                    handle_import(db); return DeckBuilderAction::None;
                }
                if Rect::new(px, btn_y_base + (render::DB_ROW_H + 4.0) * 2.0, pw, render::DB_ROW_H).contains(mouse) {
                    let b64 = decks::export_all(&db.decks);
                    miniquad::window::clipboard_set(&b64);
                    db.set_clipboard_msg("All decks copied!".to_string());
                    return DeckBuilderAction::None;
                }
            }
            if is_key_pressed(KeyCode::N) { db.open_new_deck(); }
            if is_key_pressed(KeyCode::I) { handle_import(db); }
            if is_key_pressed(KeyCode::X) {
                let b64 = decks::export_all(&db.decks);
                miniquad::window::clipboard_set(&b64);
                db.set_clipboard_msg("All decks copied!".to_string());
            }
        }
        Panel::Editor { .. } => {
            let list_y_start = 44.0;
            let scroll = db.deck_scroll;
            let btn_y_base = h - 3.0 * (render::DB_ROW_H + 4.0) - 4.0;

            if clicked {
                let uniques = db.unique_editing();
                for (i, &cid) in uniques.iter().enumerate() {
                    let ry = list_y_start + i as f32 * (render::DB_ROW_H + 2.0) - scroll;
                    if Rect::new(px, ry, pw, render::DB_ROW_H).contains(mouse) {
                        db.remove_one(cid);
                        return DeckBuilderAction::None;
                    }
                }
                if Rect::new(px, btn_y_base, pw, render::DB_ROW_H).contains(mouse) {
                    export_current_deck(db);
                    return DeckBuilderAction::None;
                }
                if Rect::new(px, btn_y_base + render::DB_ROW_H + 4.0, pw, render::DB_ROW_H).contains(mouse) {
                    db.save_deck(); return DeckBuilderAction::None;
                }
                if Rect::new(px, btn_y_base + (render::DB_ROW_H + 4.0) * 2.0, pw, render::DB_ROW_H).contains(mouse) {
                    db.delete_deck(); return DeckBuilderAction::None;
                }
            }
            if is_key_pressed(KeyCode::X) { export_current_deck(db); }
            if is_key_pressed(KeyCode::Enter) { db.save_deck(); }
            if is_key_pressed(KeyCode::Delete) { db.delete_deck(); }
        }
    }

    DeckBuilderAction::None
}

fn export_current_deck(db: &mut DeckBuilderState) {
    let (id, name) = match &db.panel {
        Panel::Editor { id, name } => (*id, name.clone()),
        _ => return,
    };
    let deck = decks::Deck { id: id.unwrap_or_else(decks::now_id), name, cards: db.editing_cards.clone() };
    let b64 = decks::export_one(&deck);
    miniquad::window::clipboard_set(&b64);
    db.set_clipboard_msg("Deck copied!".to_string());
}

fn handle_import(db: &mut DeckBuilderState) {
    if let Some(clip) = miniquad::window::clipboard_get() {
        match decks::import_from_base64(&clip) {
            Ok(imported) => {
                for deck in imported {
                    let pos = db.decks.iter().position(|d| d.id == deck.id);
                    match pos {
                        Some(i) => db.decks[i] = deck,
                        None => db.decks.push(deck),
                    }
                }
                decks::save(&db.decks);
                db.set_clipboard_msg("Decks imported!".to_string());
            }
            Err(e) => {
                eprintln!("[import] failed: {}", e);
                db.set_clipboard_msg("Import failed!".to_string());
            }
        }
    }
}

/// Returns the entity_id of whatever is under the mouse (enemy or friendly).
fn find_target(mouse: Vec2, state: &GameStateClient, w: f32, h: f32) -> Option<u32> {
    let enemy_rects = layout::enemy_minion_rects(state.enemy_board.battlefield.len(), w, h);
    if let Some(i) = enemy_rects.iter().position(|r| r.contains(mouse)) {
        return Some(state.enemy_board.battlefield[i].entity_id);
    }
    if layout::enemy_hero_rect(w, h).contains(mouse) {
        return Some(state.enemy_board.hero.entity_id);
    }
    let self_rects = layout::self_minion_rects(state.self_board.battlefield.len(), w, h);
    if let Some(i) = self_rects.iter().position(|r| r.contains(mouse)) {
        return Some(state.self_board.battlefield[i].entity_id);
    }
    if layout::self_hero_rect(w, h).contains(mouse) {
        return Some(state.self_board.hero.entity_id);
    }
    None
}

/// Returns an enemy entity_id only (for attacks).
fn find_enemy_target(mouse: Vec2, state: &GameStateClient, w: f32, h: f32) -> Option<u32> {
    let enemy_rects = layout::enemy_minion_rects(state.enemy_board.battlefield.len(), w, h);
    if let Some(i) = enemy_rects.iter().position(|r| r.contains(mouse)) {
        return Some(state.enemy_board.battlefield[i].entity_id);
    }
    if layout::enemy_hero_rect(w, h).contains(mouse) {
        return Some(state.enemy_board.hero.entity_id);
    }
    None
}
