use serde_json::Value;

use shared::types::{AttackData, GameStateClient, PlayerMetaData, ScenarioFrameInfo};

pub enum ServerEvent {
    GameState(GameStateClient),
    ScenarioFrame(ScenarioFrameInfo),
    Redirect,
}

enum SioState {
    Connecting,
    Connected,
}

#[cfg(not(target_arch = "wasm32"))]
mod ws {
    use ewebsock::{WsEvent, WsMessage, WsReceiver, WsSender};

    pub struct WebSocket {
        sender: WsSender,
        receiver: WsReceiver,
        is_open: bool,
    }

    impl WebSocket {
        pub fn connect(url: &str) -> Self {
            let (sender, receiver) = ewebsock::connect(url, ewebsock::Options::default())
                .expect("ws connect failed");
            Self { sender, receiver, is_open: false }
        }
        pub fn send_text(&mut self, s: &str) {
            self.sender.send(WsMessage::Text(s.to_string()));
        }
        pub fn try_recv(&mut self) -> Option<String> {
            loop {
                match self.receiver.try_recv()? {
                    WsEvent::Opened => self.is_open = true,
                    WsEvent::Message(WsMessage::Text(t)) => return Some(t),
                    WsEvent::Message(_) => {}
                    WsEvent::Closed => self.is_open = false,
                    WsEvent::Error(e) => {
                        eprintln!("[ws] error: {}", e);
                        self.is_open = false;
                    }
                }
            }
        }
        pub fn connected(&self) -> bool {
            self.is_open
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod ws {
    unsafe extern "C" {
        fn app_ws_connect(url_ptr: *const u8, url_len: u32) -> u32;
        fn app_ws_send_text(handle: u32, ptr: *const u8, len: u32);
        fn app_ws_recv_peek_len(handle: u32) -> u32;
        fn app_ws_recv_take(handle: u32, buf_ptr: *mut u8) -> u32;
        fn app_ws_state(handle: u32) -> u32;
    }

    pub struct WebSocket {
        handle: u32,
    }

    impl WebSocket {
        pub fn connect(url: &str) -> Self {
            let bytes = url.as_bytes();
            let handle = unsafe { app_ws_connect(bytes.as_ptr(), bytes.len() as u32) };
            Self { handle }
        }
        pub fn send_text(&mut self, s: &str) {
            let bytes = s.as_bytes();
            unsafe { app_ws_send_text(self.handle, bytes.as_ptr(), bytes.len() as u32); }
        }
        pub fn try_recv(&mut self) -> Option<String> {
            let len = unsafe { app_ws_recv_peek_len(self.handle) };
            if len == 0 {
                return None;
            }
            let mut buf = vec![0u8; len as usize];
            let actual = unsafe { app_ws_recv_take(self.handle, buf.as_mut_ptr()) };
            buf.truncate(actual as usize);
            String::from_utf8(buf).ok()
        }
        pub fn connected(&self) -> bool {
            unsafe { app_ws_state(self.handle) == 1 }
        }
    }
}

pub struct NetworkClient {
    ws: ws::WebSocket,
    state: SioState,
    was_connected: bool,
}

impl NetworkClient {
    pub fn new(host: &str) -> Self {
        let url = format!("{}/socket.io/?EIO=4&transport=websocket", host);
        let ws = ws::WebSocket::connect(&url);
        Self { ws, state: SioState::Connecting, was_connected: false }
    }

    pub fn emit(&mut self, event: &str, data: Value) {
        let payload = serde_json::to_string(&[Value::String(event.to_string()), data]).unwrap();
        self.ws.send_text(&format!("42{}", payload));
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

    pub fn run_scenarios(&mut self) {
        self.emit("runScenarios", Value::Null);
    }

    pub fn reset_server(&mut self) {
        self.emit("resetServer", serde_json::Value::Null);
    }

    /// Call each frame; returns one parsed server event per invocation (call in a loop).
    pub fn poll(&mut self) -> Option<ServerEvent> {
        loop {
            // Detect connection-open transition and send the namespace-connect frame.
            let now_connected = self.ws.connected();
            if now_connected && !self.was_connected {
                self.was_connected = true;
                self.ws.send_text("40");
            }

            let text = self.ws.try_recv()?;
            if let Some(ev) = self.handle_message(&text) {
                return Some(ev);
            }
        }
    }

    fn handle_message(&mut self, text: &str) -> Option<ServerEvent> {
        let mut chars = text.chars();
        match chars.next()? {
            // Engine.io ping — respond with pong
            '2' => {
                self.ws.send_text("3");
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
                        "scenarioFrame" => {
                            let info: ScenarioFrameInfo = serde_json::from_value(data.clone()).ok()?;
                            Some(ServerEvent::ScenarioFrame(info))
                        }
                        "redirect" => Some(ServerEvent::Redirect),
                        _ => None,
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }
}
