//! egui view for the deck builder. Owns every widget except the card art itself:
//! the catalog grid is laid out and hit-tested here, then painted by macroquad
//! between the egui build and draw passes, so the two share one coordinate space.

use egui_macroquad::egui;
use macroquad::prelude::Rect;

use crate::deckbuilder::{
    card_cost, card_id, card_meta, ColorFilter, DeckBuilderState, FilterType, Panel,
    MAX_DECK_CARDS,
};
use crate::ui;

const CARD_W: f32 = 96.0;
const CARD_H: f32 = 140.0;
const CARD_GAP: f32 = 8.0;
const PANEL_W: f32 = 320.0;
/// Costs above this fold into the last bar of the curve.
const CURVE_MAX_COST: i32 = 7;

pub enum Action {
    None,
    GoLobby,
}

pub struct Frame {
    pub action: Action,
    /// Catalog slots to paint this frame, as (index into `all_cards`, rect, copies in deck).
    pub cards: Vec<(usize, Rect, usize)>,
}

/// Runs the whole deck builder for one frame. Must be followed by painting `Frame::cards`
/// and then `egui_macroquad::draw()`.
pub fn run(db: &mut DeckBuilderState) -> Frame {
    db.poll_import();

    let mut frame = Frame { action: Action::None, cards: vec![] };

    egui_macroquad::ui(|ctx| {
        // one egui point per virtual canvas unit, so egui rects are macroquad rects
        ctx.set_pixels_per_point(macroquad::prelude::screen_height() / ui::REF_H);
        style(ctx);

        top_bar(ctx, db, &mut frame.action);
        side_panel(ctx, db);
        catalog(ctx, db, &mut frame.cards);

        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            match db.panel {
                Panel::DeckList => frame.action = Action::GoLobby,
                Panel::Editor { .. } => db.panel = Panel::DeckList,
            }
        }
    });

    frame
}

fn style(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = egui::Color32::from_rgb(26, 26, 41);
    visuals.window_fill = egui::Color32::from_rgb(33, 33, 51);
    visuals.extreme_bg_color = egui::Color32::from_rgb(20, 20, 33);
    visuals.selection.bg_fill = egui::Color32::from_rgb(90, 68, 150);
    ctx.set_visuals(visuals);
}

fn top_bar(ctx: &egui::Context, db: &mut DeckBuilderState, action: &mut Action) {
    egui::TopBottomPanel::top("db_top").show(ctx, |ui| {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            if ui.button("< Lobby").clicked() {
                *action = Action::GoLobby;
            }
            ui.separator();
            ui.heading("Deck Builder");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!("{} of {} cards", db.filtered.len(), db.all_cards.len()));
            });
        });

        // second row keeps the filters from clipping when the window is narrow
        ui.horizontal_wrapped(|ui| {
            let search = ui.add(
                egui::TextEdit::singleline(&mut db.search)
                    .desired_width(150.0)
                    .hint_text("search"),
            );
            let mut dirty = search.changed();

            ui.separator();
            for (label, value) in [
                ("All", FilterType::All),
                ("Minions", FilterType::Minions),
                ("Incantations", FilterType::Incantations),
            ] {
                dirty |= ui.selectable_value(&mut db.filter_type, value, label).changed();
            }

            ui.separator();
            for (label, value) in [
                ("Any", ColorFilter::All),
                ("White", ColorFilter::White),
                ("Black", ColorFilter::Black),
            ] {
                dirty |= ui.selectable_value(&mut db.filter_color, value, label).changed();
            }

            if dirty {
                db.apply_filter();
            }
        });
        ui.add_space(4.0);
    });
}

fn side_panel(ctx: &egui::Context, db: &mut DeckBuilderState) {
    egui::SidePanel::right("db_side")
        .exact_width(PANEL_W)
        .show(ctx, |ui| match &db.panel {
            Panel::DeckList => deck_list(ui, db),
            Panel::Editor { .. } => editor(ui, db),
        });
}

fn deck_list(ui: &mut egui::Ui, db: &mut DeckBuilderState) {
    ui.add_space(6.0);
    ui.heading("Decks");
    ui.separator();

    let mut open: Option<u64> = None;
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .max_height(ui.available_height() - 92.0)
        .show(ui, |ui| {
            if db.decks.is_empty() {
                ui.weak("No decks yet.");
            }
            for deck in &db.decks {
                let row = ui.add_sized(
                    [ui.available_width(), 34.0],
                    egui::Button::new(format!("{}\n{} cards", deck.name, deck.cards.len())),
                );
                if row.clicked() {
                    open = Some(deck.id);
                }
                ui.add_space(2.0);
            }
        });

    if let Some(id) = open {
        db.open_deck(id);
    }

    ui.separator();
    if ui.add_sized([ui.available_width(), 26.0], egui::Button::new("New deck  [N]")).clicked() {
        db.open_new_deck();
    }
    if ui.add_sized([ui.available_width(), 26.0], egui::Button::new("Import from clipboard  [I]")).clicked() {
        db.request_import();
    }
    if ui.add_sized([ui.available_width(), 26.0], egui::Button::new("Export all  [X]")).clicked() {
        db.export_all();
    }
    status(ui, db);
}

fn editor(ui: &mut egui::Ui, db: &mut DeckBuilderState) {
    ui.add_space(6.0);
    if let Panel::Editor { name, .. } = &mut db.panel {
        ui.add(
            egui::TextEdit::singleline(name)
                .desired_width(f32::INFINITY)
                .char_limit(36)
                .font(egui::TextStyle::Heading),
        );
    }

    let total = db.editing_cards.len();
    let color = if total == MAX_DECK_CARDS {
        egui::Color32::from_rgb(120, 200, 130)
    } else {
        egui::Color32::GRAY
    };
    ui.colored_label(color, format!("{total} / {MAX_DECK_CARDS} cards"));

    curve(ui, db);
    ui.separator();

    let mut add: Option<u32> = None;
    let mut remove: Option<u32> = None;
    let uniques = db.unique_editing();

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .max_height(ui.available_height() - 92.0)
        .show(ui, |ui| {
            if uniques.is_empty() {
                ui.weak("Click cards in the catalog to add them.");
            }
            for cid in uniques {
                let Some(card) = db.card_by_id(cid) else { continue };
                let (name, _, _) = card_meta(card);
                let cost = card_cost(card);
                let count = db.count_of(cid);

                ui.horizontal(|ui| {
                    ui.add_sized([22.0, 20.0], egui::Label::new(format!("{cost}")));
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width() - 76.0, 20.0),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| { ui.add(egui::Label::new(name).truncate()); },
                    );
                    ui.label(format!("x{count}"));
                    if ui.small_button("-").clicked() {
                        remove = Some(cid);
                    }
                    if ui.small_button("+").clicked() {
                        add = Some(cid);
                    }
                });
            }
        });

    if let Some(cid) = remove {
        db.remove_one(cid);
    }
    if let Some(cid) = add {
        db.add_card(cid);
    }

    ui.separator();
    if ui.add_sized([ui.available_width(), 26.0], egui::Button::new("Save  [Enter]")).clicked() {
        db.save_deck();
    }
    if ui.add_sized([ui.available_width(), 26.0], egui::Button::new("Export  [X]")).clicked() {
        db.export_current();
    }
    if ui.add_sized([ui.available_width(), 26.0], egui::Button::new("Delete  [Del]")).clicked() {
        db.delete_deck();
    }
    status(ui, db);
}

/// Cards per mana cost, with everything at or above the cap folded into the last bar.
fn curve(ui: &mut egui::Ui, db: &DeckBuilderState) {
    let mut buckets = [0usize; CURVE_MAX_COST as usize + 1];
    for &cid in &db.editing_cards {
        if let Some(card) = db.card_by_id(cid) {
            let slot = card_cost(card).clamp(0, CURVE_MAX_COST) as usize;
            buckets[slot] += 1;
        }
    }
    let peak = buckets.iter().copied().max().unwrap_or(0).max(1) as f32;

    let height = 46.0;
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    let painter = ui.painter();
    let slot_w = rect.width() / buckets.len() as f32;

    for (i, &count) in buckets.iter().enumerate() {
        let x = rect.left() + i as f32 * slot_w;
        let bar_h = (count as f32 / peak) * (height - 14.0);
        let bar = egui::Rect::from_min_size(
            egui::pos2(x + 2.0, rect.bottom() - 12.0 - bar_h),
            egui::vec2(slot_w - 4.0, bar_h),
        );
        painter.rect_filled(bar, 2.0, egui::Color32::from_rgb(120, 96, 190));

        let label = if i as i32 == CURVE_MAX_COST { format!("{i}+") } else { i.to_string() };
        painter.text(
            egui::pos2(x + slot_w / 2.0, rect.bottom() - 10.0),
            egui::Align2::CENTER_TOP,
            label,
            egui::FontId::proportional(9.0),
            egui::Color32::GRAY,
        );
    }
}

fn status(ui: &mut egui::Ui, db: &DeckBuilderState) {
    if let Some((msg, _)) = &db.clipboard_msg {
        ui.colored_label(egui::Color32::from_rgb(120, 200, 130), msg);
    }
}

/// Allocates one interactive slot per filtered card and records where macroquad
/// should paint it. Only rows touching the viewport are built.
fn catalog(ctx: &egui::Context, db: &mut DeckBuilderState, out: &mut Vec<(usize, Rect, usize)>) {
    let mut clicked: Option<u32> = None;
    let editing = matches!(db.panel, Panel::Editor { .. });

    egui::CentralPanel::default()
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            let step_x = CARD_W + CARD_GAP;
            let cols = ((ui.available_width() + CARD_GAP) / step_x).floor().max(1.0) as usize;
            let step_y = CARD_H + CARD_GAP;
            let rows = db.filtered.len().div_ceil(cols);

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show_viewport(ui, |ui, viewport| {
                    ui.set_height(rows as f32 * step_y);
                    let origin = ui.min_rect().min + egui::vec2(CARD_GAP, CARD_GAP);

                    let first = (viewport.min.y / step_y).floor().max(0.0) as usize;
                    let last = ((viewport.max.y / step_y).ceil() as usize + 1).min(rows);

                    for row in first..last {
                        for col in 0..cols {
                            let slot = row * cols + col;
                            let Some(&card_idx) = db.filtered.get(slot) else { break };

                            let pos = origin + egui::vec2(col as f32 * step_x, row as f32 * step_y);
                            let rect = egui::Rect::from_min_size(pos, egui::vec2(CARD_W, CARD_H));
                            let response = ui.allocate_rect(rect, egui::Sense::click());

                            let card = &db.all_cards[card_idx];
                            let id = card_id(card);
                            if editing && response.clicked() {
                                clicked = Some(id);
                            }
                            let (name, _, _) = card_meta(card);
                            let response = response.on_hover_text(name);

                            if response.hovered() {
                                ui.painter().rect_stroke(
                                    rect.expand(2.0),
                                    3.0,
                                    egui::Stroke::new(2.0, egui::Color32::from_rgb(150, 120, 220)),
                                    egui::StrokeKind::Outside,
                                );
                            }
                            out.push((card_idx, to_macroquad(rect), db.count_of(id)));
                        }
                    }
                });
        });

    if let Some(id) = clicked {
        db.add_card(id);
    }
}

fn to_macroquad(rect: egui::Rect) -> Rect {
    Rect::new(rect.min.x, rect.min.y, rect.width(), rect.height())
}
