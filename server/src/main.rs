mod cards;
mod engine;
mod types;

use axum::routing::get;
use axum::Json;
use engine::Game;
use serde::Deserialize;
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef, State},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use types::*;

#[derive(Clone, Default)]
struct ServerState {
    inner: Arc<Mutex<InnerState>>,
}

#[derive(Default)]
struct InnerState {
    game: Option<Game>,
    queue: Vec<(String, PlayerMetaData)>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlayCardData {
    index: usize,
    target: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TradeCardData {
    index: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AttackInput {
    origin_id: String,
    target_id: String,
}

fn validate_turn(socket_id: &str, game: &Game) -> bool {
    let s = &game.state;
    (socket_id == s.white_player_id && s.white_turn) || (socket_id == s.black_player_id && !s.white_turn)
}

async fn broadcast(io: &SocketIo, game: &Game) {
    let white = game.parse_client_state(true);
    let black = game.parse_client_state(false);
    io.to(game.state.white_player_id.clone()).emit("newGameState", &white).await.ok();
    io.to(game.state.black_player_id.clone()).emit("newGameState", &black).await.ok();
}

async fn on_connect(socket: SocketRef, State(state): State<ServerState>, io: SocketIo) {
    println!("Connected: {}", socket.id);
    socket.join(socket.id.to_string());

    socket.on("queueUp", {
        let io = io.clone();
        let state = state.clone();
        move |socket: SocketRef, Data::<PlayerMetaData>(meta)| async move {
            let mut inner = state.inner.lock().await;

            if inner.queue.iter().any(|(id, _)| id == &socket.id.to_string()) {
                println!("already in queue");
                return;
            }

            inner.queue.push((socket.id.to_string(), meta));
            println!("queue length: {}", inner.queue.len());

            if inner.queue.len() >= 2 && inner.game.is_none() {
                println!("starting game!");
                let (id1, meta1) = inner.queue.pop().unwrap();
                let (id2, meta2) = inner.queue.pop().unwrap();

                let deck1 = cards::deck_to_cards(&meta1.choosen_deck);
                let deck2 = cards::deck_to_cards(&meta2.choosen_deck);

                let is_p1_white = rand::random::<bool>();
                let game = Game::new(id1.clone(), id2.clone(), is_p1_white, deck1, deck2);
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

    socket.on("endTurn", {
        let io = io.clone();
        let state = state.clone();
        move |socket: SocketRef| async move {
            let mut inner = state.inner.lock().await;
            let game = match inner.game.as_mut() {
                Some(g) => g,
                None => return,
            };
            if !validate_turn(&socket.id.to_string(), game) {
                return;
            }
            game.end_turn();
            broadcast(&io, game).await;
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
            if !validate_turn(&socket.id.to_string(), game) {
                return;
            }
            game.play_card(data.index, data.target);
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
            if !validate_turn(&socket.id.to_string(), game) {
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
            if !validate_turn(&socket.id.to_string(), game) {
                return;
            }
            game.trade_card(data.index);
            broadcast(&io, game).await;
        }
    });

    socket.on("leaveQueue", {
        let state = state.clone();
        move |socket: SocketRef| async move {
            let mut inner = state.inner.lock().await;
            inner.queue.retain(|(id, _)| id != &socket.id.to_string());
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
        move |socket: SocketRef| async move {
            let mut inner = state.inner.lock().await;
            inner.queue.retain(|(id, _)| id != &socket.id.to_string());
            println!("Disconnected: {}", socket.id);
        }
    });
}

async fn get_cards_handler() -> Json<Vec<Card>> {
    Json(cards::get_cards())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = ServerState::default();
    let (layer, io) = SocketIo::builder().with_state(state).build_layer();

    let io_clone = io.clone();
    io.ns("/", move |socket: SocketRef, State(state): State<ServerState>| {
        on_connect(socket, State(state), io_clone.clone())
    });

    let app = axum::Router::new()
        .route("/", get(async || "Hello World"))
        .route("/cards", get(get_cards_handler))
        .layer(layer)
        .layer(CorsLayer::new().allow_origin(Any).allow_headers(Any).allow_methods(Any));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on port 3000");
    axum::serve(listener, app).await?;
    Ok(())
}
