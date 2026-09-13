//! Hand-card glow tuning. Values live in a config file so the look can be tweaked
//! without a rebuild; the baked defaults are what wasm and a missing file both get.

use macroquad::prelude::Color;
use serde::Deserialize;
use std::cell::RefCell;

pub const FILE: &str = "glow.toml";

/// Written out when the file is missing. Kept in step with [`Config::default`] by a test,
/// since a stale value here would silently win over the Rust default.
#[cfg(not(target_arch = "wasm32"))]
const TEMPLATE: &str = r#"# Hand-card glow. Saved edits apply straight away, no rebuild.

rings = 18
# how far past the card edge the outermost ring sits
spread = 18.0
# gap between the card edge and the innermost ring; nonzero reads as a hard outline
inset = 0.0
thickness = 3.0
# alpha exponent across the rings, higher fades to nothing sooner
falloff = 2.2
# dimmest point of the pulse, as a fraction of full brightness
pulse_floor = 0.45
# fraction of the spread that breathes with the pulse
breathe = 0.18
# pulse offset per hand slot, so a full hand waves instead of strobing in unison
phase_step = 0.55

# a met condition is the rarer thing to notice, so it outshines plain affordability
[condition]
color = [1.0, 0.84, 0.25]
intensity = 0.42
speed = 3.0

[playable]
color = [0.35, 0.65, 1.0]
intensity = 0.26
speed = 1.6
"#;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct Config {
    pub rings: usize,
    pub spread: f32,
    pub inset: f32,
    pub thickness: f32,
    pub falloff: f32,
    pub pulse_floor: f32,
    pub breathe: f32,
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
    use super::{set, Watcher, FILE, TEMPLATE};

    const POLL_SECONDS: f32 = 0.4;

    impl Watcher {
        pub fn new() -> Self {
            if std::fs::metadata(FILE).is_err() {
                let _ = std::fs::write(FILE, TEMPLATE);
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
            match toml::from_str(&text) {
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

#[cfg(test)]
mod tests {
    use super::{Config, TEMPLATE};

    #[test]
    fn template_matches_defaults() {
        let parsed: Config = toml::from_str(TEMPLATE).expect("template parses");
        assert_eq!(parsed, Config::default());
    }
}
