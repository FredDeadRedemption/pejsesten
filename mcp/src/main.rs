mod socket;
mod view;

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{Implementation, ServerCapabilities, ServerConfig};
use rmcp::transport::stdio;
use rmcp::{ErrorData, ServerHandler, ServiceExt, tool, tool_handler, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};
use shared::types::{GamePhase, GameStateClient, PlayerMetaData};
use tokio::sync::Mutex;

const INSTRUCTIONS: &str = "Play a game of Pejsesten against the person at the browser client. \
Call join_game first, then submit_mulligan, then act on your turn and end_turn. Hand entries are \
numbered by position and that number is what play_card and trade_card take; everything on a board \
carries an entity id in brackets and that id is what attack and play_card targets take. The \
server silently ignores an illegal action, so a tool that reports no change means the engine \
refused the move.";

/// How long an action waits for the state the server broadcasts in reply.
const ACTION_GRACE: Duration = Duration::from_secs(2);

#[derive(Deserialize)]
struct Deck {
    name: String,
    hero: u32,
    cards: Vec<u32>,
}

#[derive(Deserialize, JsonSchema)]
struct JoinArgs {
    /// Deck name from decks.json. The first deck in the file when left out.
    deck: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
struct MulliganArgs {
    /// Hand positions to put back. An empty list keeps the whole hand.
    indices: Vec<usize>,
}

#[derive(Deserialize, JsonSchema)]
struct PlayArgs {
    /// Hand position of the card to play.
    index: usize,
    /// Entity id to aim a targeted ability at.
    target: Option<u32>,
    /// Which of an omen's three triggers to arm, 0 to 2. Required to play an omen; ignored by every other card.
    omen_trigger: Option<usize>,
    /// Where a minion slots into your board, 0 being leftmost. Defaults to the right end.
    position: Option<usize>,
}

#[derive(Deserialize, JsonSchema)]
struct AttackArgs {
    /// Entity id of your attacking minion.
    origin_id: u32,
    /// Entity id of the enemy minion or hero being attacked.
    target_id: u32,
}

#[derive(Deserialize, JsonSchema)]
struct TradeArgs {
    /// Hand position of a tradeable card.
    index: usize,
}

#[derive(Deserialize, JsonSchema)]
struct WaitArgs {
    /// How long to hold the call open. 25 seconds when left out, 120 at most.
    timeout_seconds: Option<u64>,
}

#[derive(Clone)]
struct Pejsesten {
    host: String,
    token: String,
    connection: Arc<Mutex<Option<Arc<socket::Connection>>>>,
    tool_router: ToolRouter<Self>,
}

impl Pejsesten {
    fn new(host: String, token: String) -> Self {
        Self { host, token, connection: Arc::new(Mutex::new(None)), tool_router: Self::tool_router() }
    }

    /// Connects on first use and after the server has gone away, so the order of startup is free.
    async fn connection(&self) -> Result<Arc<socket::Connection>, ErrorData> {
        let mut slot = self.connection.lock().await;
        if let Some(live) = slot.as_ref().filter(|c| c.alive()) {
            return Ok(live.clone());
        }
        let fresh = socket::Connection::connect(&self.host, &self.token)
            .await
            .map(Arc::new)
            .map_err(|e| ErrorData::internal_error(format!("cannot reach the game server at {}: {e}", self.host), None))?;
        *slot = Some(fresh.clone());
        Ok(fresh)
    }

    /// Sends an action and reports the state the server pushes back. An action off turn or in the
    /// wrong phase draws no answer at all; an illegal one is answered with the state unchanged.
    async fn act(&self, event: &str, data: Value) -> Result<String, ErrorData> {
        let connection = self.connection().await?;
        let mut states = connection.watch();
        let before = states.borrow_and_update().as_ref().map(as_json);

        connection.emit(event, data).map_err(|e| ErrorData::internal_error(e, None))?;
        let answered = tokio::time::timeout(ACTION_GRACE, states.changed()).await.is_ok();

        let after = states.borrow().clone();
        let after = match after {
            Some(after) => after,
            None => return Ok("no game in progress, call join_game".to_string()),
        };
        let refused = if !answered {
            "the server ignored that, so it is not your turn or the phase is wrong\n\n"
        } else if before.as_ref() == Some(&as_json(&after)) {
            "nothing changed, so the engine refused that move\n\n"
        } else {
            ""
        };
        Ok(format!("{refused}{}", view::render(&after)))
    }
}

fn as_json(state: &GameStateClient) -> Value {
    serde_json::to_value(state).unwrap_or(Value::Null)
}

/// Decks live in the repo root next to the client that writes them, so edits need no rebuild.
fn load_decks() -> Result<Vec<Deck>, ErrorData> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the mcp crate always sits one level below the workspace root")
        .join("decks.json");
    let raw = std::fs::read_to_string(&path)
        .map_err(|e| ErrorData::internal_error(format!("cannot read {}: {e}", path.display()), None))?;
    serde_json::from_str(&raw).map_err(|e| ErrorData::internal_error(format!("cannot parse decks.json: {e}"), None))
}

/// Whether there is anything to do: a mulligan to submit, or a turn to play.
fn ready_to_act(state: &GameStateClient) -> bool {
    match state.phase {
        GamePhase::Mulligan => !state.mulligan_submitted,
        GamePhase::Playing => state.your_turn,
        GamePhase::Over { .. } => false,
    }
}

#[tool_router(router = tool_router)]
impl Pejsesten {
    #[tool(description = "Join the queue with one of the decks from decks.json and wait for the game to start.")]
    async fn join_game(&self, Parameters(args): Parameters<JoinArgs>) -> Result<String, ErrorData> {
        let decks = load_decks()?;
        let deck = match &args.deck {
            Some(wanted) => decks.iter().find(|d| d.name.eq_ignore_ascii_case(wanted)).ok_or_else(|| {
                let names: Vec<&str> = decks.iter().map(|d| d.name.as_str()).collect();
                ErrorData::invalid_params(format!("no deck named {wanted}, decks.json has: {}", names.join(", ")), None)
            })?,
            None => decks.first().ok_or_else(|| ErrorData::internal_error("decks.json is empty", None))?,
        };

        let connection = self.connection().await?;
        let meta = PlayerMetaData {
            username: "Claude".to_string(),
            chosen_deck: deck.cards.clone(),
            hero: deck.hero,
            avatar: String::new(),
        };
        connection
            .emit("queueUp", serde_json::to_value(&meta).unwrap())
            .map_err(|e| ErrorData::internal_error(e, None))?;

        match connection.state(Duration::from_secs(20)).await {
            Some(state) => Ok(format!("queued with {} and the game is on\n\n{}", deck.name, view::render(&state))),
            None => Ok(format!("queued with {}, waiting for an opponent to queue up too", deck.name)),
        }
    }

    #[tool(description = "The current board, hands and resources.")]
    async fn game_state(&self) -> Result<String, ErrorData> {
        let connection = self.connection().await?;
        match connection.state(Duration::from_secs(2)).await {
            Some(state) => Ok(view::render(&state)),
            None => Ok("no game in progress, call join_game".to_string()),
        }
    }

    #[tool(description = "Put back the listed hand positions and draw replacements. An empty list keeps the hand.")]
    async fn submit_mulligan(&self, Parameters(args): Parameters<MulliganArgs>) -> Result<String, ErrorData> {
        self.act("submitMulligan", json!({ "indices": args.indices })).await
    }

    #[tool(description = "Play a card from hand, with an entity id as target, a trigger index for an omen, and a board position for a minion.")]
    async fn play_card(&self, Parameters(args): Parameters<PlayArgs>) -> Result<String, ErrorData> {
        let data = json!({ "index": args.index, "target": args.target, "omen_trigger": args.omen_trigger, "position": args.position });
        self.act("playCard", data).await
    }

    #[tool(description = "Attack an enemy minion or hero with one of your minions, both by entity id.")]
    async fn attack(&self, Parameters(args): Parameters<AttackArgs>) -> Result<String, ErrorData> {
        self.act("attack", json!({ "origin_id": args.origin_id, "target_id": args.target_id })).await
    }

    #[tool(description = "Trade a tradeable card: one mana, the card to the bottom of the deck, draw one.")]
    async fn trade_card(&self, Parameters(args): Parameters<TradeArgs>) -> Result<String, ErrorData> {
        self.act("tradeCard", json!({ "index": args.index })).await
    }

    #[tool(description = "End your turn and hand the board to the opponent.")]
    async fn end_turn(&self) -> Result<String, ErrorData> {
        self.act("endTurn", Value::Null).await
    }

    #[tool(description = "Hold until there is something to do, which is your turn or a mulligan to submit.")]
    async fn wait_for_turn(&self, Parameters(args): Parameters<WaitArgs>) -> Result<String, ErrorData> {
        let connection = self.connection().await?;
        let deadline = tokio::time::Instant::now() + Duration::from_secs(args.timeout_seconds.unwrap_or(25).clamp(1, 120));
        let mut states = connection.watch();
        loop {
            let state = states.borrow_and_update().clone();
            if let Some(state) = state {
                if ready_to_act(&state) {
                    return Ok(view::render(&state));
                }
            }
            if tokio::time::timeout_at(deadline, states.changed()).await.is_err() {
                return Ok(match connection.state(Duration::ZERO).await {
                    Some(state) => format!("nothing to act on yet\n\n{}", view::render(&state)),
                    None => "no game in progress, call join_game".to_string(),
                });
            }
        }
    }

    #[tool(description = "Throw away the server's current game and queue. The server holds one game at a time.")]
    async fn reset_game(&self) -> Result<String, ErrorData> {
        let connection = self.connection().await?;
        connection.emit("resetServer", Value::Null).map_err(|e| ErrorData::internal_error(e, None))?;
        Ok("the server's game slot is clear".to_string())
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for Pejsesten {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("pejsesten", env!("CARGO_PKG_VERSION")))
            .with_instructions(INSTRUCTIONS)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = std::env::var("PEJSESTEN_HOST").unwrap_or_else(|_| "ws://127.0.0.1:3000".to_string());
    let token = std::env::var("PEJSESTEN_TOKEN").unwrap_or_else(|_| "claude".to_string());
    Pejsesten::new(host, token).serve(stdio()).await?.waiting().await?;
    Ok(())
}
