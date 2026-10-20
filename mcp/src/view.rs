use std::fmt::Write;

use shared::types::*;

/// Hand entries are numbered by position, because that is what play and trade take. Everything on
/// the board carries its entity id in brackets, because that is what attack and targeting take.
pub fn render(s: &GameStateClient) -> String {
    let mut o = String::new();

    match s.phase {
        GamePhase::Mulligan => {
            let waiting = if s.mulligan_submitted { "submitted, waiting for the opponent" } else { "choose which cards to replace" };
            let _ = writeln!(o, "mulligan - {waiting}");
        }
        GamePhase::Playing => {
            let whose = if s.your_turn { "your turn" } else { "opponent's turn" };
            let _ = writeln!(o, "turn {} - {whose}, {} cards played this turn", s.turn_count, s.cards_played_this_turn);
        }
    }

    let _ = writeln!(o, "you:   {}", resources(&s.self_board));
    let _ = writeln!(o, "enemy: {}", resources(&s.enemy_board));

    battlefield(&mut o, "your board", &s.self_board.battlefield);
    battlefield(&mut o, "enemy board", &s.enemy_board.battlefield);
    omens(&mut o, "your omens", &s.self_board.omens);
    omens(&mut o, "enemy omens", &s.enemy_board.omens);
    hand(&mut o, "your hand", &s.self_board.hand, &s.hand_hints);
    if s.open_cards {
        hand(&mut o, "enemy hand (open cards is on in this profile)", &s.enemy_board.hand, &[]);
    }

    o
}

fn resources(b: &Board) -> String {
    format!(
        "{} [{}] {}/{} hp, mana {}/{}, embers {}, hand {}, deck {}",
        b.hero.card.name,
        b.hero.entity_id,
        b.hero.stats.defence,
        b.hero.stats.max_defence,
        b.mana,
        b.base_mana,
        b.embers,
        b.hand.len(),
        b.deck.len(),
    )
}

fn battlefield(o: &mut String, title: &str, minions: &[MinionEntity]) {
    if minions.is_empty() {
        let _ = writeln!(o, "{title}: empty");
        return;
    }
    let _ = writeln!(o, "{title}:");
    for m in minions {
        let _ = writeln!(o, "  [{}] {} {}{}{}", m.entity_id, m.card.name, stats(m), flags(m), text(&m.card.description));
    }
}

fn stats(m: &MinionEntity) -> String {
    if m.stats.defence < m.stats.max_defence {
        format!("{}/{} of {}", m.stats.attack, m.stats.defence, m.stats.max_defence)
    } else {
        format!("{}/{}", m.stats.attack, m.stats.defence)
    }
}

fn flags(m: &MinionEntity) -> String {
    let mut f: Vec<&str> = Vec::new();
    f.push(if m.exhausted { "exhausted" } else { "ready" });
    if m.card.attributes.contains(&MinionAttribute::Guard) {
        f.push("guard");
    }
    if m.ward_active {
        f.push("ward up");
    }
    if m.stealth_active {
        f.push("stealth");
    }
    if m.doomed {
        f.push("doomed");
    }
    for race in &m.card.races {
        f.push(match race {
            Race::Human => "human",
            Race::Elf => "elf",
            Race::Beast => "beast",
        });
    }
    format!(" ({})", f.join(", "))
}

fn omens(o: &mut String, title: &str, armed: &[OmenEntity]) {
    if armed.is_empty() {
        return;
    }
    let _ = writeln!(o, "{title}:");
    for omen in armed {
        let trigger = match omen.armed_trigger {
            Some(t) => t.label().to_string(),
            None => "trigger hidden".to_string(),
        };
        let _ = writeln!(o, "  [{}] {} ({trigger}){}", omen.entity_id, omen.card.name, text(&omen.card.description));
    }
}

fn hand(o: &mut String, title: &str, cards: &[CardEntity], hints: &[CardHint]) {
    if cards.is_empty() {
        let _ = writeln!(o, "{title}: empty");
        return;
    }
    let _ = writeln!(o, "{title}:");
    for (i, card) in cards.iter().enumerate() {
        let kind = match card {
            CardEntity::Minion(m) => format!("minion {}", stats(m)),
            CardEntity::Incantation(_) => "incantation".to_string(),
            CardEntity::Omen(_) => "omen".to_string(),
        };
        let hint = match hints.get(i) {
            Some(h) if !h.playable => " - cannot play now".to_string(),
            Some(h) if h.condition_met => " - playable, condition met".to_string(),
            Some(_) => " - playable".to_string(),
            None => String::new(),
        };
        let description = match card {
            CardEntity::Minion(m) => text(&m.card.description),
            CardEntity::Incantation(c) => text(&c.card.description),
            CardEntity::Omen(c) => text(&c.card.description),
        };
        let _ = writeln!(o, "  {i}. {} - {kind}, {} mana{hint}{description}", card.name(), card.cost());
        if let CardEntity::Omen(omen) = card {
            for (t, trigger) in omen.card.triggers.iter().enumerate() {
                let _ = writeln!(o, "       omen_trigger {t}: {}", trigger.label());
            }
        }
    }
}

/// Card text is written with html for the renderer, so tags come out as spacing here.
fn text(description: &Option<String>) -> String {
    let raw = match description {
        Some(d) => d,
        None => return String::new(),
    };
    let mut plain = String::with_capacity(raw.len());
    let mut in_tag = false;
    for c in raw.chars() {
        match c {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                plain.push(' ');
            }
            c if !in_tag => plain.push(c),
            _ => {}
        }
    }
    let plain = plain.split_whitespace().collect::<Vec<_>>().join(" ");
    if plain.is_empty() { String::new() } else { format!(" | {plain}") }
}
