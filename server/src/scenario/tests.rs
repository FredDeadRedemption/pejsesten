use super::build::SCENARIO_ID_BASE;
use super::cases;

#[test]
fn every_scenario_passes() {
    let mut failures: Vec<String> = vec![];
    let mut ran = 0;

    for c in cases::all() {
        let scn = (c.run)();
        ran += 1;
        for f in scn.failures() {
            failures.push(format!("[{}] {} — {}", c.group, c.name, f));
        }
    }

    assert!(ran > 0, "no scenarios registered");
    assert!(failures.is_empty(), "{} of {} scenarios failed:\n{}", failures.len(), ran, failures.join("\n"));
}

#[test]
fn scenario_cards_cannot_collide_with_the_real_pool() {
    let highest = shared::cards::get_all_cards()
        .iter()
        .map(|c| match c {
            shared::types::Card::Minion(m) => m.id,
            shared::types::Card::Incantation(i) => i.id,
        })
        .max()
        .unwrap_or(0);
    assert!(highest < SCENARIO_ID_BASE, "cards.rs has reached the scenario id range");
}
