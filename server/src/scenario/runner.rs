//! Plays the scenario catalogue out over a socket so it can be watched in the real client.
//!
//! Runs the same [`cases`] the test suite runs; nothing here re-implements a check.

use super::cases;
use crate::engine::{Game, IdGenerator};
use crate::settings;
use shared::types::ScenarioFrameInfo;
use socketioxide::SocketIo;
use std::time::Duration;

pub async fn run(io: SocketIo, socket_id: String) {
    let all = cases::all();
    let total = all.len();
    let mut failed = 0usize;

    for (i, case) in all.iter().enumerate() {
        let scenario = (case.run)();
        failed += scenario.failures().len();

        for frame in scenario.frames() {
            let view = Game::from_state(frame.state.clone(), IdGenerator::new()).parse_client_state(true);
            let info = ScenarioFrameInfo {
                case: case.name.clone(),
                group: case.group.to_string(),
                label: frame.label.clone(),
                kind: format!("{:?}", frame.kind).to_lowercase(),
                ok: frame.ok,
                case_index: i + 1,
                case_total: total,
                failed,
                done: false,
            };
            io.to(socket_id.clone()).emit("newGameState", &view).await.ok();
            io.to(socket_id.clone()).emit("scenarioFrame", &info).await.ok();
            tokio::time::sleep(Duration::from_millis(settings::SCENARIO_FRAME_MS)).await;
        }
    }

    let summary = ScenarioFrameInfo {
        case: format!("{} scenarios, {} failed", total, failed),
        group: "done".to_string(),
        label: if failed == 0 { "all green".to_string() } else { format!("{failed} expectations failed") },
        kind: "check".to_string(),
        ok: failed == 0,
        case_index: total,
        case_total: total,
        failed,
        done: true,
    };
    io.to(socket_id).emit("scenarioFrame", &summary).await.ok();
}
