mod bot;
mod engine;
mod scenario;
mod settings;

use engine::Game;
use serde::Deserialize;
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef, State, TryData},
    socket::DisconnectReason,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

use crate::engine::{IdGenerator, PlayerSetup};
use shared::types::*;

#[derive(Clone, Default)]
struct ServerState {
    inner: Arc<Mutex<InnerState>>,
}

#[derive(Default)]
struct InnerState {
    game: Option<Game>,
    queue: Vec<(String, PlayerMetaData)>,
    // open sockets per token; a second tab must not make closing the first look like leaving
    sockets: HashMap<String, u32>,
}

/// Sent as socket.io auth on connect. The token is stored client side and outlives a reload.
#[derive(Deserialize)]
struct Auth {
    token: String,
}

#[derive(Clone)]
struct PlayerToken(String);

#[derive(Deserialize)]
struct PlayCardData {
    index: usize,
    target: Option<u32>,
    omen_trigger: Option<usize>,
}

#[derive(Deserialize)]
struct TradeCardData {
    index: usize,
}

#[derive(Deserialize)]
struct AttackInput {
    origin_id: u32,
    target_id: u32,
}

#[derive(Deserialize)]
struct MulliganData {
    indices: Vec<usize>,
}

/// Player identity, which is the client's token and not the socket id, so it survives a reload.
fn player_id(socket: &SocketRef) -> String {
    socket.extensions.get::<PlayerToken>().map(|t| t.0).unwrap_or_else(|| socket.id.to_string())
}

fn is_player(player_id: &str, game: &Game) -> bool {
    player_id == game.state.white_player_id || player_id == game.state.black_player_id
}

fn validate_turn(player_id: &str, game: &Game) -> bool {
    if game.state.phase != GamePhase::Playing {
        return false;
    }
    let s = &game.state;
    (player_id == s.white_player_id && s.white_turn) || (player_id == s.black_player_id && !s.white_turn)
}

async fn broadcast(io: &SocketIo, game: &Game) {
    let white = game.parse_client_state(true);
    let black = game.parse_client_state(false);
    io.to(game.state.white_player_id.clone()).emit("newGameState", &white).await.ok();
    io.to(game.state.black_player_id.clone()).emit("newGameState", &black).await.ok();
}

async fn on_connect(socket: SocketRef, auth: Option<Auth>, State(state): State<ServerState>, io: SocketIo) {
    let me = auth.map(|a| a.token).filter(|t| !t.is_empty()).unwrap_or_else(|| socket.id.to_string());
    socket.extensions.insert(PlayerToken(me.clone()));
    // the room is keyed by token, so a reconnected socket inherits the player's messages
    socket.join(me.clone());
    println!("Connected: {} (player {me})", socket.id);

    {
        let mut inner = state.inner.lock().await;
        *inner.sockets.entry(me.clone()).or_insert(0) += 1;
        if let Some(game) = inner.game.as_ref() {
            if is_player(&me, game) {
                println!("resuming game for {me}");
                broadcast(&io, game).await;
            }
        }
    }

    socket.on("queueUp", {
        let io = io.clone();
        let state = state.clone();
        move |socket: SocketRef, Data::<PlayerMetaData>(meta)| async move {
            let mut inner = state.inner.lock().await;

            let me = player_id(&socket);
            if inner.queue.iter().any(|(id, _)| id == &me) {
                println!("already in queue");
                return;
            }

            inner.queue.push((me, meta));
            println!("queue length: {}", inner.queue.len());

            if inner.queue.len() >= 2 && inner.game.is_none() {
                println!("starting game!");
                let (id1, meta1) = inner.queue.pop().unwrap();
                let (id2, meta2) = inner.queue.pop().unwrap();

                let mut ids = IdGenerator::new();
                let player1 = PlayerSetup {
                    deck: crate::engine::deck_to_cards(&meta1.chosen_deck, &mut ids),
                    hero: shared::cards::hero_or_default(meta1.hero).clone(),
                    id: id1.clone(),
                };
                let player2 = PlayerSetup {
                    deck: crate::engine::deck_to_cards(&meta2.chosen_deck, &mut ids),
                    hero: shared::cards::hero_or_default(meta2.hero).clone(),
                    id: id2.clone(),
                };

                let is_p1_white = rand::random::<bool>();
                let game = Game::new(player1, player2, is_p1_white, ids);
                broadcast(&io, &game).await;

                let url = format!(
                    "{}{}",
                    rand::random::<u16>(),
                    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()
                );
                io.to(id1).emit("redirect", &url).await.ok();
                io.to(id2).emit("redirect", &url).await.ok();

                inner.game = Some(game);
            }
        }
    });

    socket.on("runScenarios", {
        let io = io.clone();
        move |socket: SocketRef| async move {
            println!("running scenario catalogue for {}", socket.id);
            scenario::runner::run(io, player_id(&socket)).await;
        }
    });

    socket.on("queueUpBot", {
        let io = io.clone();
        let state = state.clone();
        move |socket: SocketRef, Data::<PlayerMetaData>(meta)| async move {
            let mut inner = state.inner.lock().await;

            if inner.game.is_some() {
                println!("game already in progress, ignoring queueUpBot");
                return;
            }

            println!("starting bot game for {}", socket.id);
            let me = player_id(&socket);
            let mut ids = IdGenerator::new();
            let player = PlayerSetup {
                deck: crate::engine::deck_to_cards(&meta.chosen_deck, &mut ids),
                hero: shared::cards::hero_or_default(meta.hero).clone(),
                id: me.clone(),
            };
            let robot = PlayerSetup {
                deck: crate::engine::deck_to_cards(&bot::default_deck(), &mut ids),
                hero: shared::cards::hero_or_default(bot::HERO_ID).clone(),
                id: bot::BOT_ID.to_string(),
            };

            // randomize who goes first (white always moves first in Game)
            let is_player_white = rand::random::<bool>();
            let mut game = Game::new(player, robot, is_player_white, ids);

            // Bot instantly keeps its full hand
            game.submit_mulligan(bot::BOT_ID, vec![]);
            broadcast(&io, &game).await;

            let url = format!(
                "{}{}",
                rand::random::<u16>(),
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()
            );
            io.to(me).emit("redirect", &url).await.ok();

            inner.game = Some(game);
        }
    });

    socket.on("endTurn", {
        let io = io.clone();
        let state = state.clone();
        move |socket: SocketRef| async move {
            // Phase 1: end the player's turn, broadcast immediately so client sees "not your turn"
            let is_bot = {
                let mut inner = state.inner.lock().await;
                let game = match inner.game.as_mut() {
                    Some(g) => g,
                    None => return,
                };
                if !validate_turn(&player_id(&socket), game) {
                    return;
                }
                game.end_turn();
                let is_bot = bot::is_bot_turn(game);
                broadcast(&io, game).await;
                is_bot
            }; // lock released here — spammed endTurn events will now fail validate_turn

            // Phase 2: delay then run bot move
            if is_bot {
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                let mut inner = state.inner.lock().await;
                if let Some(game) = inner.game.as_mut() {
                    if bot::is_bot_turn(game) {
                        let bot_white = bot::bot_is_white(game);
                        bot::make_bot_move(game, bot_white, &io).await;
                        broadcast(&io, game).await;
                    }
                }
            }
        }
    });

    socket.on("playCard", {
        let io = io.clone();
        let state = state.clone();
        move |socket: SocketRef, Data::<PlayCardData>(data)| async move {
            let mut inner = state.inner.lock().await;
            let game = match inner.game.as_mut() {
                Some(g) => g,
                None => return,
            };
            if !validate_turn(&player_id(&socket), game) {
                return;
            }
            game.play_card(data.index, data.target, data.omen_trigger);
            broadcast(&io, game).await;
        }
    });

    socket.on("attack", {
        let io = io.clone();
        let state = state.clone();
        move |socket: SocketRef, Data::<AttackInput>(data)| async move {
            let mut inner = state.inner.lock().await;
            let game = match inner.game.as_mut() {
                Some(g) => g,
                None => return,
            };
            if !validate_turn(&player_id(&socket), game) {
                return;
            }
            game.attack(AttackData {
                origin_id: data.origin_id,
                target_id: data.target_id,
            });
            broadcast(&io, game).await;
        }
    });

    socket.on("tradeCard", {
        let io = io.clone();
        let state = state.clone();
        move |socket: SocketRef, Data::<TradeCardData>(data)| async move {
            let mut inner = state.inner.lock().await;
            let game = match inner.game.as_mut() {
                Some(g) => g,
                None => return,
            };
            if !validate_turn(&player_id(&socket), game) {
                return;
            }
            game.trade_card(data.index);
            broadcast(&io, game).await;
        }
    });

    socket.on("submitMulligan", {
        let io = io.clone();
        let state = state.clone();
        move |socket: SocketRef, Data::<MulliganData>(data)| async move {
            let is_bot_ready = {
                let mut inner = state.inner.lock().await;
                let game = match inner.game.as_mut() {
                    Some(g) => g,
                    None => return,
                };
                let was_mulligan = game.state.phase == GamePhase::Mulligan;
                game.submit_mulligan(&player_id(&socket), data.indices);
                broadcast(&io, game).await;
                was_mulligan && game.state.phase == GamePhase::Playing && bot::is_bot_turn(game)
            };

            if is_bot_ready {
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                let mut inner = state.inner.lock().await;
                if let Some(game) = inner.game.as_mut() {
                    if bot::is_bot_turn(game) {
                        let bot_white = bot::bot_is_white(game);
                        bot::make_bot_move(game, bot_white, &io).await;
                        broadcast(&io, game).await;
                    }
                }
            }
        }
    });

    socket.on("leaveQueue", {
        let state = state.clone();
        move |socket: SocketRef| async move {
            let mut inner = state.inner.lock().await;
            inner.queue.retain(|(id, _)| id != &player_id(&socket));
            println!("Left queue: {}", socket.id);
        }
    });

    socket.on("resetServer", {
        let state = state.clone();
        move |_: SocketRef| async move {
            let mut inner = state.inner.lock().await;
            inner.game = None;
            inner.queue.clear();
            println!("RESET GAME");
        }
    });

    socket.on_disconnect({
        let state = state.clone();
        move |socket: SocketRef, reason: DisconnectReason| async move {
            let me = player_id(&socket);
            println!("Disconnected: {} (player {me}) ({reason:?})", socket.id);

            let was_playing = {
                let mut inner = state.inner.lock().await;
                inner.queue.retain(|(id, _)| id != &me);
                match inner.sockets.get_mut(&me) {
                    Some(n) if *n > 1 => {
                        *n -= 1;
                        return;
                    }
                    _ => inner.sockets.remove(&me),
                };
                inner.game.as_ref().is_some_and(|g| is_player(&me, g))
            };
            if !was_playing {
                return;
            }

            tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(settings::DISCONNECT_GRACE_MS)).await;
                let mut inner = state.inner.lock().await;
                if inner.sockets.contains_key(&me) {
                    return;
                }
                // another game may hold the slot by now, and it is not this player's to destroy
                if inner.game.as_ref().is_some_and(|g| is_player(&me, g)) {
                    inner.game = None;
                    println!("game destroyed: {me} did not return");
                }
            });
        }
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = ServerState::default();
    let (layer, io) = SocketIo::builder().with_state(state).build_layer();

    let io_clone = io.clone();
    io.ns("/", move |socket: SocketRef, TryData(auth): TryData<Auth>, State(state): State<ServerState>| {
        on_connect(socket, auth.ok(), State(state), io_clone.clone())
    });

    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "dist".to_string());
    let app = axum::Router::new()
        .fallback_service(ServeDir::new(&static_dir))
        .layer(layer)
        .layer(CorsLayer::new().allow_origin(Any).allow_headers(Any).allow_methods(Any));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    println!("Server running on port {port} ({} settings)", settings::PROFILE);
    axum::serve(listener, app).await?;
    Ok(())
}
