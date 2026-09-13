//! Hand-card glow tuning. Values live in a config file so the look can be tweaked
//! without a rebuild; the baked defaults are what wasm and a missing file both get.

use macroquad::prelude::Color;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

pub const FILE: &str = "glow.json";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Style {
    pub color: [f32; 3],
    pub intensity: f32,
    pub speed: f32,
}

impl Style {
    pub fn color(&self) -> Color {
        Color::new(self.color[0], self.color[1], self.color[2], 1.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub rings: usize,
    /// How far past the card edge the outermost ring sits.
    pub spread: f32,
    /// Gap between the card edge and the innermost ring. Nonzero reads as a hard outline.
    pub inset: f32,
    pub thickness: f32,
    /// Alpha exponent across the rings. Higher fades to nothing sooner.
    pub falloff: f32,
    /// Dimmest point of the pulse, as a fraction of full brightness.
    pub pulse_floor: f32,
    /// Fraction of the spread that breathes with the pulse.
    pub breathe: f32,
    /// Pulse offset per hand slot, so a full hand waves instead of strobing in unison.
    pub phase_step: f32,
    pub condition: Style,
    pub playable: Style,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rings: 18,
            spread: 18.0,
            inset: 0.0,
            thickness: 3.0,
            falloff: 2.2,
            pulse_floor: 0.45,
            breathe: 0.18,
            phase_step: 0.55,
            condition: Style { color: [1.00, 0.84, 0.25], intensity: 0.42, speed: 3.0 },
            playable: Style { color: [0.35, 0.65, 1.00], intensity: 0.26, speed: 1.6 },
        }
    }
}

thread_local! {
    static ACTIVE: RefCell<Config> = RefCell::new(Config::default());
}

pub fn current() -> Config {
    ACTIVE.with(|c| c.borrow().clone())
}

#[cfg(not(target_arch = "wasm32"))]
fn set(config: Config) {
    ACTIVE.with(|c| *c.borrow_mut() = config);
}

/// Polls the config file and republishes it on change. Native only; wasm keeps the defaults.
pub struct Watcher {
    reloads: u32,
    #[cfg(not(target_arch = "wasm32"))]
    poll_timer: f32,
    #[cfg(not(target_arch = "wasm32"))]
    stamp: Option<std::time::SystemTime>,
}

impl Watcher {
    pub fn reloads(&self) -> u32 {
        self.reloads
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod backend {
    use super::{set, Config, Watcher, FILE};

    const POLL_SECONDS: f32 = 0.4;

    impl Watcher {
        pub fn new() -> Self {
            if std::fs::metadata(FILE).is_err() {
                if let Ok(text) = serde_json::to_string_pretty(&Config::default()) {
                    let _ = std::fs::write(FILE, text);
                }
            }
            let mut watcher = Watcher { reloads: 0, poll_timer: 0.0, stamp: None };
            watcher.poll();
            watcher
        }

        /// The stamp advances even on a parse error, so a half-typed save is reported
        /// once rather than every poll, and the last good config stays on screen.
        fn poll(&mut self) {
            let Ok(stamp) = std::fs::metadata(FILE).and_then(|m| m.modified()) else {
                return;
            };
            if self.stamp == Some(stamp) {
                return;
            }
            self.stamp = Some(stamp);

            let Ok(text) = std::fs::read_to_string(FILE) else {
                return;
            };
            match serde_json::from_str(&text) {
                Ok(config) => {
                    set(config);
                    self.reloads += 1;
                }
                Err(e) => eprintln!("{FILE}: {e}"),
            }
        }

        pub fn tick(&mut self, dt: f32) {
            self.poll_timer += dt;
            if self.poll_timer < POLL_SECONDS {
                return;
            }
            self.poll_timer = 0.0;
            self.poll();
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod backend {
    use super::Watcher;

    impl Watcher {
        pub fn new() -> Self {
            Watcher { reloads: 0 }
        }

        pub fn tick(&mut self, _dt: f32) {}
    }
}
