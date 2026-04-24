use ewebsock::{WsEvent, WsMessage, WsReceiver, WsSender};
use serde_json::Value;

use crate::types::{AttackData, GameStateClient, PlayerMetaData};

pub enum ServerEvent {
    GameState(GameStateClient),
    Redirect(String),
}

enum SioState {
    Connecting,
    Connected,
}

pub struct NetworkClient {
    sender: WsSender,
    receiver: WsReceiver,
    state: SioState,
}

impl NetworkClient {
    pub fn new(host: &str) -> Self {
        // socket.io v4 over WebSocket
        let url = format!("{}/socket.io/?EIO=4&transport=websocket", host);
        let (sender, receiver) =
            ewebsock::connect(url, ewebsock::Options::default()).expect("ws connect failed");
        Self { sender, receiver, state: SioState::Connecting }
    }

    pub fn emit(&mut self, event: &str, data: Value) {
        let payload = serde_json::to_string(&[Value::String(event.to_string()), data]).unwrap();
        self.sender.send(WsMessage::Text(format!("42{}", payload)));
    }

    pub fn queue_up(&mut self, meta: &PlayerMetaData) {
        self.emit("queueUp", serde_json::to_value(meta).unwrap());
    }

    pub fn queue_up_bot(&mut self, meta: &PlayerMetaData) {
        self.emit("queueUpBot", serde_json::to_value(meta).unwrap());
    }

    pub fn end_turn(&mut self) {
        self.emit("endTurn", Value::Null);
    }

    pub fn play_card(&mut self, index: usize, target: Option<u32>) {
        let data = serde_json::json!({ "index": index, "target": target });
        self.emit("playCard", data);
    }

    pub fn attack(&mut self, data: &AttackData) {
        self.emit("attack", serde_json::to_value(data).unwrap());
    }

    pub fn trade_card(&mut self, index: usize) {
        self.emit("tradeCard", serde_json::json!({ "index": index }));
    }

    pub fn submit_mulligan(&mut self, indices: &[usize]) {
        self.emit("submitMulligan", serde_json::json!({ "indices": indices }));
    }

    pub fn reset_server(&mut self) {
        self.emit("resetServer", serde_json::Value::Null);
    }

    /// Call each frame; returns one parsed server event per invocation (call in a loop).
    pub fn poll(&mut self) -> Option<ServerEvent> {
        while let Some(ws_event) = self.receiver.try_recv() {
            match ws_event {
                WsEvent::Opened => {
                    // Connect to the default socket.io namespace
                    self.sender.send(WsMessage::Text("40".to_string()));
                }
                WsEvent::Message(WsMessage::Text(text)) => {
                    if let Some(ev) = self.handle_message(&text) {
                        return Some(ev);
                    }
                }
                WsEvent::Error(e) => eprintln!("[ws] error: {}", e),
                WsEvent::Closed => eprintln!("[ws] closed"),
                _ => {}
            }
        }
        None
    }

    fn handle_message(&mut self, text: &str) -> Option<ServerEvent> {
        let mut chars = text.chars();
        match chars.next()? {
            // Engine.io ping — respond with pong
            '2' => {
                self.sender.send(WsMessage::Text("3".to_string()));
                None
            }
            // Socket.io packet
            '4' => match chars.next()? {
                '0' => {
                    self.state = SioState::Connected;
                    None
                }
                '2' => {
                    let json = &text[2..];
                    let arr: Vec<Value> = serde_json::from_str(json).ok()?;
                    let event = arr.first()?.as_str()?;
                    let data = arr.get(1)?;
                    match event {
                        "newGameState" => {
                            let state: GameStateClient =
                                serde_json::from_value(data.clone()).ok()?;
                            Some(ServerEvent::GameState(state))
                        }
                        "redirect" => {
                            Some(ServerEvent::Redirect(data.as_str()?.to_string()))
                        }
                        _ => None,
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }
}
