use amm_core::services::input::CornerRow;
use amm_core::structs::menu::{CornerSlot, CornerTarget};
use egui::{Rect, Ui, UiBuilder, pos2};
use studio_ui::theme::Icon;
use studio_ui::widgets::CornerButton;

use crate::model::{PanelAction, PanelIntent, PanelModel};

/// The corner row in the bottom-right while the game shows its own cursor: the square Mods button, then each
/// mod's menu entry for this screen to its left. It sits above the game's version text, so it never covers it.
/// A click on Mods asks to open the panel; a click on an entry asks to tell its mod.
pub struct CornerRowView;

impl CornerRowView {
    /// Draws the row and returns the rect around it, in points.
    pub fn show(ui: &mut Ui, screen: Rect, model: &PanelModel, out: &mut Vec<PanelIntent>) -> Option<Rect> {
        let entries: Vec<(CornerTarget, f32)> = model
            .menu_entries()
            .map(|entry| {
                let target = CornerTarget::Entry { owner: entry.owner, id: entry.id };
                (target, CornerButton::labelled_width(ui, &entry.label))
            })
            .collect();
        let mut slots: Vec<CornerSlot> = Vec::with_capacity(entries.len() + 1);
        CornerRow::layout(screen.width(), screen.height(), &entries, &mut slots);
        for slot in &slots {
            let [min_x, min_y, max_x, max_y] = slot.rect;
            let rect = Rect::from_min_max(pos2(min_x, min_y), pos2(max_x, max_y));
            let clicked = ui
                .scope_builder(UiBuilder::new().max_rect(rect), |ui| match slot.target {
                    CornerTarget::Mods => CornerButton::icon(ui, Icon::PUZZLE),
                    CornerTarget::Entry { owner, id } => {
                        let entry = model.menu_entries().find(|entry| entry.owner == owner && entry.id == id);
                        let (mark, label) = entry.map_or(("", ""), |entry| (entry.mark.as_str(), entry.label.as_str()));
                        CornerButton::labelled(ui, mark, label)
                    }
                })
                .inner
                .clicked();
            if clicked {
                let action = match slot.target {
                    CornerTarget::Mods => PanelAction::Open,
                    CornerTarget::Entry { owner, id } => PanelAction::MenuEntry { owner, id },
                };
                out.push(PanelIntent::Action(action));
            }
        }
        CornerRow::bounds(&slots)
            .map(|[min_x, min_y, max_x, max_y]| Rect::from_min_max(pos2(min_x, min_y), pos2(max_x, max_y)))
    }
}
