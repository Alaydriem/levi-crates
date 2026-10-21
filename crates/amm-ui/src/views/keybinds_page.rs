use egui::{Align2, ScrollArea, Sense, Ui, pos2, vec2};
use studio_ui::theme::{Icon, Metrics, StudioPalette, Typeface};
use studio_ui::widgets::Draw;

use crate::model::{PanelController, PanelIntent, PanelModel};
use crate::views::{KeybindRowView, PageHeader, TableHeader};

/// The Keybinds page of the Manager group: every keybind of every mod in one table, grouped by mod,
/// with the panel toggle at the top. A click on a row waits for the next key press as its new chord.
pub struct KeybindsPage;

impl KeybindsPage {
    pub fn show(ui: &mut Ui, model: &PanelModel, controller: &PanelController, out: &mut Vec<PanelIntent>) {
        PageHeader::show(
            ui,
            Icon::KEYBOARD,
            StudioPalette::ACCENT_HOVER,
            "Keybinds",
            "Every keybind of every installed mod, and the panel toggle",
        );
        ui.add_space(Metrics::GAP_L);
        let rows = model.keybind_rows();
        let preview = controller.rebind_preview();
        ScrollArea::vertical().id_salt("amm.keybinds").auto_shrink([false, false]).show(ui, |ui| {
            let width = ui.available_width() - 8.0;
            TableHeader::show(ui, width, &KeybindRowView::TITLES, &KeybindRowView::COLS);
            let mut group = "";
            let mut previous_clash = true;
            for row in &rows {
                if row.mod_name != group {
                    group = row.mod_name;
                    Self::group(ui, group, width);
                    previous_clash = true;
                }
                let waiting = controller.rebinding() == Some(row.target);
                let prompt = waiting.then(|| preview.as_deref().unwrap_or(KeybindRowView::PRESS_A_KEY));
                if KeybindRowView::show(ui, model, row, width, prompt, !previous_clash).clicked() && !waiting {
                    out.push(PanelIntent::BeginRebind(row.target));
                }
                previous_clash = row.clash_with.is_some();
                ui.add_space(2.0);
            }
            ui.add_space(Metrics::GAP_M);
            let hint = "Click a keybind, then press its new chord. Esc cancels. Changes save on their own.";
            ui.label(egui::RichText::new(hint).font(Typeface::help()).color(StudioPalette::TEXT_FAINT));
        });
    }

    fn group(ui: &mut Ui, name: &str, width: f32) {
        let height = Draw::line_height(ui.painter(), &Typeface::section()) + Metrics::GAP_L;
        let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        Draw::caps(
            ui.painter(),
            pos2(rect.left() + TableHeader::INSET, rect.bottom() - 6.0),
            Align2::LEFT_BOTTOM,
            name,
            StudioPalette::TEXT_FAINT,
        );
    }
}
