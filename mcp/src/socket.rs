use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use shared::types::GameStateClient;
use tokio::sync::{mpsc, watch};
use tokio_tungstenite::tungstenite::Message;

/// One socket.io connection to the game server, framed by hand the same way the client does it.
/// A background task owns the socket; tools read the last state the server pushed.
pub struct Connection {
    out: mpsc::UnboundedSender<String>,
    state: watch::Receiver<Option<GameStateClient>>,
}

impl Connection {
    pub async fn connect(host: &str, token: &str) -> Result<Self, String> {
        let url = format!("{host}/socket.io/?EIO=4&transport=websocket");
        let (ws, _) = tokio_tungstenite::connect_async(&url).await.map_err(|e| e.to_string())?;
        let (mut sink, mut stream) = ws.split();

        let (out, mut outbox) = mpsc::unbounded_channel::<String>();
        let (tx, state) = watch::channel(None);
        let auth = serde_json::json!({ "token": token }).to_string();

        tokio::spawn(async move {
            // an event sent before the namespace is up is dropped by the server, so hold the outbox
            let mut joined = false;
            loop {
                let outgoing = tokio::select! {
                    queued = outbox.recv(), if joined => match queued {
                        Some(text) => text,
                        None => break,
                    },
                    inbound = stream.next() => match inbound {
                        Some(Ok(Message::Text(text))) => match handle(text.as_str(), &tx, &auth) {
                            Some(Reply::Send(frame)) => frame,
                            Some(Reply::Joined) => {
                                joined = true;
                                continue;
                            }
                            None => continue,
                        },
                        Some(Ok(_)) => continue,
                        _ => break,
                    },
                };
                if sink.send(Message::Text(outgoing.into())).await.is_err() {
                    break;
                }
            }
        });

        Ok(Self { out, state })
    }

    pub fn alive(&self) -> bool {
        !self.out.is_closed()
    }

    pub fn emit(&self, event: &str, data: Value) -> Result<(), String> {
        let payload = serde_json::to_string(&[Value::String(event.to_string()), data]).map_err(|e| e.to_string())?;
        self.out.send(format!("42{payload}")).map_err(|_| "the connection to the game server is gone".to_string())
    }

    pub fn watch(&self) -> watch::Receiver<Option<GameStateClient>> {
        self.state.clone()
    }

    /// The last state pushed, waiting the given grace for a first one on a fresh connection.
    pub async fn state(&self, grace: Duration) -> Option<GameStateClient> {
        let mut rx = self.watch();
        let current = rx.borrow_and_update().clone();
        if current.is_some() {
            return current;
        }
        tokio::time::timeout(grace, rx.changed()).await.ok()?.ok()?;
        rx.borrow().clone()
    }
}

enum Reply {
    Send(String),
    Joined,
}

/// What the reader owes the server for one message, having folded a pushed state away itself.
fn handle(text: &str, state: &watch::Sender<Option<GameStateClient>>, auth: &str) -> Option<Reply> {
    let mut chars = text.chars();
    match chars.next()? {
        // engine.io open; the token rides along as socket.io auth so the server knows us up front
        '0' => Some(Reply::Send(format!("40{auth}"))),
        '2' => Some(Reply::Send("3".to_string())),
        '4' => match chars.next()? {
            '0' => Some(Reply::Joined),
            '2' => {
                let frame: Vec<Value> = serde_json::from_str(&text[2..]).ok()?;
                if frame.first()?.as_str()? == "newGameState" {
                    if let Ok(pushed) = serde_json::from_value(frame.get(1)?.clone()) {
                        state.send_replace(Some(pushed));
                    }
                }
                None
            }
            _ => None,
        },
        _ => None,
    }
}
