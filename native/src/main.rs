use macroquad::prelude::*;

mod layout;
mod network;
mod render;
mod textures;
mod types;

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
            if is_key_pressed(KeyCode::H) {
                net.queue_up(&PlayerMetaData {
                    username: USERNAME.to_string(),
                    chosen_deck: default_deck(),
                    avatar: String::new(),
                });
            }
            if is_key_pressed(KeyCode::B) {
                net.queue_up_bot(&PlayerMetaData {
                    username: USERNAME.to_string(),
                    chosen_deck: default_deck(),
                    avatar: String::new(),
                });
            }
            if is_key_pressed(KeyCode::R) {
                net.reset_server();
            }
        }

        Screen::Mulligan { state, selected } => {
            let hand_len = state.self_board.hand.len();
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
            if is_key_pressed(KeyCode::Enter) && !state.mulligan_submitted {
                let indices: Vec<usize> = selected.drain(..).collect();
                net.submit_mulligan(&indices);
            }
        }

        Screen::Playing(state) => {
            if !state.your_turn {
                return;
            }

            if is_key_pressed(KeyCode::E) {
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
                            let target = find_target(mouse, state, w, h);
                            net.play_card(index, target);
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

        _ => {}
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
