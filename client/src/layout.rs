use macroquad::prelude::*;

pub const CARD_W: f32 = 110.0;
pub const CARD_H: f32 = 160.0;

pub const MINION_W: f32 = 100.0;
pub const MINION_H: f32 = 145.0;
pub const MINION_GAP: f32 = 8.0;

// a stack of three has to fit between the divider and the mana readout
pub const OMEN_W: f32 = 56.0;
pub const OMEN_H: f32 = 80.0;
pub const OMEN_GAP: f32 = 8.0;
/// Gap between the left column and the omen stacks.
const OMEN_MARGIN: f32 = 12.0;

pub const HERO_W: f32 = 92.0;
pub const HERO_H: f32 = 134.0;

/// The left column: one small art icon per card played, newest at the top.
pub const SIDEBAR_W: f32 = 52.0;
const SIDEBAR_PAD: f32 = 8.0;
pub const ICON: f32 = 36.0;
const ICON_GAP: f32 = 5.0;
const ICON_TOP: f32 = 22.0;

/// One rect per icon the column has room for, newest at the top.
pub fn log_icon_rects(h: f32) -> Vec<Rect> {
    let pitch = ICON + ICON_GAP;
    let count = (((h - 10.0 - ICON_TOP) / pitch).max(0.0)) as usize;
    (0..count)
        .map(|i| Rect::new(SIDEBAR_PAD, ICON_TOP + i as f32 * pitch, ICON, ICON))
        .collect()
}

/// The played card, blown up beside the strip for as long as the play lasts.
pub const REVEAL_W: f32 = 196.0;
pub const REVEAL_H: f32 = 284.0;

pub fn reveal_rect(h: f32) -> Rect {
    Rect::new(SIDEBAR_W + 10.0, (h - REVEAL_H) / 2.0, REVEAL_W, REVEAL_H)
}

const TARGET_GAP: f32 = 6.0;
const TARGET_MARGIN: f32 = 10.0;
/// Biggest a chip gets, so a lone target does not read as a second played card.
const TARGET_H_MAX: f32 = REVEAL_H * 0.5;

/// Chips beside the reveal, shrunk to keep the whole column inside its height.
pub fn reveal_target_rects(count: usize, h: f32) -> Vec<Rect> {
    if count == 0 {
        return vec![];
    }
    let reveal = reveal_rect(h);
    let gaps = (count - 1) as f32 * TARGET_GAP;
    let ch = ((reveal.h - gaps) / count as f32).min(TARGET_H_MAX);
    let cw = ch * REVEAL_W / REVEAL_H;
    let x = reveal.x + reveal.w + TARGET_MARGIN;
    let top = reveal.y + (reveal.h - (count as f32 * ch + gaps)) / 2.0;
    (0..count)
        .map(|i| Rect::new(x, top + i as f32 * (ch + TARGET_GAP), cw, ch))
        .collect()
}

/// Everything centred sits in what the left column leaves over.
pub fn board_center_x(w: f32) -> f32 {
    (SIDEBAR_W + w) / 2.0
}

/// Mana and ember readouts, clear of the left column.
pub fn readout_x() -> f32 {
    SIDEBAR_W + OMEN_MARGIN
}

/// Dead zone — dropping a card here cancels the play. Extends well above the actual cards.
pub const HAND_ZONE_PADDING: f32 = 60.0;

pub fn hand_zone_rect(w: f32, h: f32) -> Rect {
    let zone_h = CARD_H + HAND_ZONE_PADDING;
    Rect::new(0.0, h - zone_h, w, zone_h)
}

/// Widest the fan may spread, and the tightest two neighbours may sit.
const HAND_SPAN: f32 = 620.0;
const HAND_PITCH_MAX: f32 = CARD_W * 0.72;
/// Circle the fan hangs from; with the pitch it decides how far each card tilts.
const HAND_ARC_R: f32 = 1400.0;
/// How far a fan hangs past its own edge of the screen, bottom for you and top for the enemy.
const HAND_SINK: f32 = 60.0;

/// One card of the fanned hand. The rotation is around the card centre.
pub struct HandSlot {
    pub rect: Rect,
    pub rotation: f32,
}

impl HandSlot {
    pub fn center(&self) -> Vec2 {
        Vec2::new(self.rect.x + self.rect.w / 2.0, self.rect.y + self.rect.h / 2.0)
    }

    /// Half the height the tilted card covers, which is more than half the rect.
    fn half_height(&self) -> f32 {
        (self.rect.w * self.rotation.sin().abs() + self.rect.h * self.rotation.cos().abs()) / 2.0
    }

    pub fn contains(&self, p: Vec2) -> bool {
        let d = p - self.center();
        let (sin, cos) = (-self.rotation).sin_cos();
        let local = Vec2::new(d.x * cos - d.y * sin, d.x * sin + d.y * cos);
        local.x.abs() <= self.rect.w / 2.0 && local.y.abs() <= self.rect.h / 2.0
    }
}

/// Own hand: fan bulging up, pinned by its lowest corner against the bottom edge.
pub fn hand_slots(count: usize, w: f32, h: f32) -> Vec<HandSlot> {
    let mut slots = fan(count, w, 1.0);
    let lowest = slots.iter().map(|s| s.center().y + s.half_height()).fold(f32::MIN, f32::max);
    shift(&mut slots, h + HAND_SINK - lowest);
    slots
}

/// Enemy hand: the same fan mirrored, hanging past the top edge by the same amount.
pub fn enemy_hand_slots(count: usize, w: f32) -> Vec<HandSlot> {
    let mut slots = fan(count, w, -1.0);
    let highest = slots.iter().map(|s| s.center().y - s.half_height()).fold(f32::MAX, f32::min);
    shift(&mut slots, -HAND_SINK - highest);
    slots
}

fn fan(count: usize, w: f32, sign: f32) -> Vec<HandSlot> {
    if count == 0 {
        return vec![];
    }
    let pitch = (HAND_SPAN / count as f32).min(HAND_PITCH_MAX);
    let step = (pitch / HAND_ARC_R).asin();
    let mid = (count - 1) as f32 / 2.0;

    (0..count)
        .map(|i| {
            let angle = (i as f32 - mid) * step;
            let cx = board_center_x(w) + HAND_ARC_R * angle.sin();
            let cy = sign * HAND_ARC_R * (1.0 - angle.cos());
            HandSlot {
                rect: Rect::new(cx - CARD_W / 2.0, cy - CARD_H / 2.0, CARD_W, CARD_H),
                rotation: sign * angle,
            }
        })
        .collect()
}

fn shift(slots: &mut [HandSlot], dy: f32) {
    for slot in slots {
        slot.rect.y += dy;
    }
}

fn self_row_y(h: f32) -> f32 {
    h / 2.0 + 18.0
}

pub fn self_minion_rects(count: usize, w: f32, h: f32) -> Vec<Rect> {
    centered_rects(count, MINION_W, MINION_H, MINION_GAP, board_center_x(w), self_row_y(h))
}

/// Where a dragged minion would slot in: how many of your minions sit left of the mouse.
pub fn drop_slot(count: usize, w: f32, h: f32, mouse: Vec2) -> usize {
    self_minion_rects(count, w, h).iter().filter(|r| r.center().x < mouse.x).count()
}

/// Your row as it is drawn while a card is held over it: the bodies lay themselves out as if the
/// newcomer were already there, so the gap under the cursor is the slot it will take.
pub fn self_minion_rects_parted(count: usize, slot: Option<usize>, w: f32, h: f32) -> Vec<Rect> {
    let Some(slot) = slot.filter(|s| *s <= count) else {
        return self_minion_rects(count, w, h);
    };
    let spread = self_minion_rects(count + 1, w, h);
    (0..count).map(|i| spread[if i < slot { i } else { i + 1 }]).collect()
}

pub fn enemy_minion_rects(count: usize, w: f32, h: f32) -> Vec<Rect> {
    let y = h / 2.0 - MINION_H - 18.0;
    centered_rects(count, MINION_W, MINION_H, MINION_GAP, board_center_x(w), y)
}

/// Omens stack away from the divider at the board's left edge, where no minion row reaches.
pub fn self_omen_rects(count: usize, h: f32) -> Vec<Rect> {
    omen_rects(count, |i| h / 2.0 + 12.0 + i * (OMEN_H + OMEN_GAP))
}

pub fn enemy_omen_rects(count: usize, h: f32) -> Vec<Rect> {
    omen_rects(count, |i| h / 2.0 - 12.0 - OMEN_H - i * (OMEN_H + OMEN_GAP))
}

fn omen_rects(count: usize, y: impl Fn(f32) -> f32) -> Vec<Rect> {
    (0..count)
        .map(|i| Rect::new(SIDEBAR_W + OMEN_MARGIN, y(i as f32), OMEN_W, OMEN_H))
        .collect()
}

pub const OMEN_PICK_W: f32 = 150.0;
pub const OMEN_PICK_H: f32 = 220.0;
pub const OMEN_PICK_GAP: f32 = 24.0;

pub fn omen_pick_rects(w: f32, h: f32) -> Vec<Rect> {
    centered_rects(3, OMEN_PICK_W, OMEN_PICK_H, OMEN_PICK_GAP, w / 2.0, h / 2.0 - OMEN_PICK_H / 2.0)
}

pub fn self_hero_rect(w: f32, h: f32) -> Rect {
    Rect::new(w - HERO_W - 14.0, h / 2.0 + 18.0, HERO_W, HERO_H)
}

pub fn enemy_hero_rect(w: f32, h: f32) -> Rect {
    Rect::new(w - HERO_W - 14.0, h / 2.0 - HERO_H - 18.0, HERO_W, HERO_H)
}

pub fn self_deck_rect(w: f32, h: f32) -> Rect {
    Rect::new(w - CARD_W - 8.0, h / 2.0 + 18.0 + HERO_H + 8.0, CARD_W, CARD_H)
}

pub fn enemy_deck_rect(w: f32, h: f32) -> Rect {
    Rect::new(w - CARD_W - 8.0, h / 2.0 - HERO_H - 18.0 - CARD_H - 8.0, CARD_W, CARD_H)
}

pub const MULLIGAN_CARD_W: f32 = 130.0;
pub const MULLIGAN_CARD_H: f32 = 190.0;
pub const MULLIGAN_SPACING: f32 = 18.0;

pub fn mulligan_card_rects(count: usize, w: f32, h: f32) -> Vec<Rect> {
    let total_w = count as f32 * (MULLIGAN_CARD_W + MULLIGAN_SPACING) - MULLIGAN_SPACING;
    let start_x = w / 2.0 - total_w / 2.0;
    let card_y = h / 2.0 - MULLIGAN_CARD_H / 2.0;
    (0..count)
        .map(|i| Rect::new(start_x + i as f32 * (MULLIGAN_CARD_W + MULLIGAN_SPACING), card_y, MULLIGAN_CARD_W, MULLIGAN_CARD_H))
        .collect()
}

pub fn mulligan_confirm_rect(w: f32, h: f32) -> Rect {
    Rect::new(w / 2.0 - 90.0, h - 80.0, 180.0, 44.0)
}

pub fn game_over_button_rect(w: f32, h: f32) -> Rect {
    Rect::new(board_center_x(w) - 95.0, h / 2.0 + 28.0, 190.0, 46.0)
}

pub fn end_turn_rect(w: f32, h: f32) -> Rect {
    Rect::new(w - HERO_W - 132.0, h / 2.0 + 14.0, 104.0, 36.0)
}

pub fn lobby_deck_rect(w: f32, h: f32) -> Rect {
    Rect::new(w / 2.0 - 115.0, h / 2.0 - 74.0, 230.0, 46.0)
}

pub fn lobby_queue_human_rect(w: f32, h: f32) -> Rect {
    Rect::new(w / 2.0 - 115.0, h / 2.0, 230.0, 46.0)
}

pub fn lobby_queue_bot_rect(w: f32, h: f32) -> Rect {
    Rect::new(w / 2.0 - 115.0, h / 2.0 + 62.0, 230.0, 46.0)
}

pub fn lobby_reset_rect(w: f32, h: f32) -> Rect {
    Rect::new(w / 2.0 - 115.0, h / 2.0 + 130.0, 230.0, 36.0)
}

pub fn lobby_deck_builder_rect(w: f32, h: f32) -> Rect {
    Rect::new(w / 2.0 - 115.0, h / 2.0 + 180.0, 230.0, 46.0)
}

pub fn lobby_card_flip_test_rect(w: f32, h: f32) -> Rect {
    Rect::new(w / 2.0 - 115.0, h / 2.0 + 242.0, 230.0, 46.0)
}

pub fn lobby_scenarios_rect(w: f32, h: f32) -> Rect {
    Rect::new(w / 2.0 - 115.0, h / 2.0 + 304.0, 230.0, 46.0)
}

pub fn lobby_glow_test_rect(w: f32, h: f32) -> Rect {
    Rect::new(w / 2.0 - 115.0, h / 2.0 + 366.0, 230.0, 46.0)
}

fn centered_rects(count: usize, rw: f32, rh: f32, gap: f32, cx: f32, y: f32) -> Vec<Rect> {
    if count == 0 {
        return vec![];
    }
    let total = count as f32 * (rw + gap) - gap;
    let start_x = cx - total / 2.0;
    (0..count)
        .map(|i| Rect::new(start_x + i as f32 * (rw + gap), y, rw, rh))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const W: f32 = 1280.0;
    const H: f32 = 720.0;

    /// The gap a slot opens must be exactly one body wide, and must sit where the slot says.
    fn gap_before(count: usize, slot: usize) -> f32 {
        let parted = self_minion_rects_parted(count, Some(slot), W, H);
        let spread = self_minion_rects(count + 1, W, H);
        assert_eq!(parted.len(), count);
        for (i, r) in parted.iter().enumerate() {
            let expected = spread[if i < slot { i } else { i + 1 }];
            assert!((r.x - expected.x).abs() < 1e-3, "body {i} at {} wanted {}", r.x, expected.x);
        }
        spread[slot].x
    }

    #[test]
    fn a_slot_opens_a_body_wide_gap_where_it_was_asked_for() {
        for slot in 0..=3 {
            let gap_x = gap_before(3, slot);
            let parted = self_minion_rects_parted(3, Some(slot), W, H);
            let left = parted.iter().filter(|r| r.x < gap_x).count();
            assert_eq!(left, slot, "slot {slot} should have {slot} bodies to its left");
        }
    }

    #[test]
    fn no_slot_leaves_the_row_where_it_was() {
        let plain = self_minion_rects(4, W, H);
        let parted = self_minion_rects_parted(4, None, W, H);
        assert_eq!(plain.len(), parted.len());
        assert!(plain.iter().zip(&parted).all(|(a, b)| (a.x - b.x).abs() < 1e-6));
    }

    #[test]
    fn a_slot_past_the_end_is_ignored_rather_than_panicking() {
        let parted = self_minion_rects_parted(2, Some(9), W, H);
        assert_eq!(parted.len(), 2);
    }

    #[test]
    fn the_drop_slot_counts_the_bodies_left_of_the_mouse() {
        let rects = self_minion_rects(3, W, H);
        assert_eq!(drop_slot(3, W, H, Vec2::new(rects[0].x - 50.0, 0.0)), 0);
        assert_eq!(drop_slot(3, W, H, Vec2::new(rects[1].center().x - 1.0, 0.0)), 1);
        assert_eq!(drop_slot(3, W, H, Vec2::new(rects[2].x + rects[2].w + 50.0, 0.0)), 3);
    }
}
