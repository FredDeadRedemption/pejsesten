use macroquad::prelude::*;

mod network;
mod render;
mod textures;
mod types;

use network::{NetworkClient, ServerEvent};
use textures::TextureCache;
use types::{GamePhase, GameStateClient, PlayerMetaData};

const SERVER_URL: &str = match option_env!("SERVER_URL") {
    Some(url) => url,
    None => "ws://localhost:3000",
};

const USERNAME: &str = match option_env!("USERNAME") {
    Some(u) => u,
    None => "player",
};

fn default_deck() -> Vec<u32> {
    // 2x of each collectible card (IDs 1–30)
    (1u32..=30).flat_map(|id| [id, id]).collect()
}

enum Screen {
    Connecting,
    Lobby,
    Mulligan { state: GameStateClient, selected: Vec<usize> },
    Playing(GameStateClient),
}

fn window_conf() -> macroquad::prelude::Conf {
    macroquad::prelude::Conf {
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

    loop {
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
                ServerEvent::Redirect(_game_id) => {
                    // State arrives via the following newGameState event
                }
            }
        }

        if matches!(screen, Screen::Connecting) && !connected {
            connected = true;
            screen = Screen::Lobby;
        }

        // --- Input ---
        handle_input(&mut screen, &mut net);

        // --- Render ---
        clear_background(Color::from_rgba(12, 12, 20, 255));

        match &screen {
            Screen::Connecting => render::draw_connecting(),
            Screen::Lobby => render::draw_lobby(USERNAME),
            Screen::Mulligan { state, selected } => render::draw_mulligan(state, selected, &cache),
            Screen::Playing(state) => render::draw_game(state, &cache),
        }

        next_frame().await;
    }
}

fn handle_input(screen: &mut Screen, net: &mut NetworkClient) {
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
                (KeyCode::Key1, 0),
                (KeyCode::Key2, 1),
                (KeyCode::Key3, 2),
                (KeyCode::Key4, 3),
                (KeyCode::Key5, 4),
                (KeyCode::Key6, 5),
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
            if is_key_pressed(KeyCode::E) && state.your_turn {
                net.end_turn();
            }
        }
        _ => {}
    }
}
