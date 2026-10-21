use amm_core::services::settings::ChordText;
use amm_core::structs::clash::{BindRef, ClashKind};
use amm_core::structs::registry::RegisteredMod;
use egui::{Align2, Sense, Ui, pos2, vec2};
use studio_ui::theme::{Icon, Metrics, StudioPalette, Typeface};
use studio_ui::widgets::Draw;

use crate::model::{PanelController, PanelIntent, PanelModel, RebindTarget};
use crate::views::{KeybindRowView, TableHeader};

/// The Keybinds tab: action, chord, capability, scope and a clash marker for each keybind. A click
/// on a row waits for the next key press as the new chord.
pub struct KeybindsTab;

impl KeybindsTab {
    pub fn show(
        ui: &mut Ui,
        model: &PanelModel,
        controller: &PanelController,
        registered: &RegisteredMod,
        out: &mut Vec<PanelIntent>,
    ) {
        let index = registered.index;
        let width = ui.available_width() - 8.0;
        let clashes: Vec<(BindRef, BindRef)> = model
            .clash_report()
            .conflicts
            .iter()
            .filter_map(|clash| match clash.kind {
                ClashKind::Key { a, b, .. } if a.mod_index == index => Some((a, b)),
                ClashKind::Key { a, b, .. } if b.mod_index == index => Some((b, a)),
                _ => None,
            })
            .collect();
        if let Some((own, other)) = clashes.first() {
            Self::banner(ui, model, registered, clashes.len(), *own, *other, width);
        }
        TableHeader::show(ui, width, &KeybindRowView::TITLES, &KeybindRowView::COLS);
        let rows = model.keybind_rows();
        let preview = controller.rebind_preview();
        let own =
            rows.iter().filter(|row| matches!(row.target, RebindTarget::Mod { mod_index, .. } if mod_index == index));
        let mut previous_clash = true;
        for row in own {
            let waiting = controller.rebinding() == Some(row.target);
            let prompt = waiting.then(|| preview.as_deref().unwrap_or(KeybindRowView::PRESS_A_KEY));
            if KeybindRowView::show(ui, model, row, width, prompt, !previous_clash).clicked() && !waiting {
                out.push(PanelIntent::BeginRebind(row.target));
            }
            previous_clash = row.clash_with.is_some();
            ui.add_space(2.0);
        }
        ui.add_space(Metrics::GAP_M);
        let hint = "Click a keybind, then press its new chord. Esc cancels.";
        ui.label(egui::RichText::new(hint).font(Typeface::help()).color(StudioPalette::TEXT_FAINT));
    }

    fn banner(
        ui: &mut Ui,
        model: &PanelModel,
        registered: &RegisteredMod,
        count: usize,
        own: BindRef,
        other: BindRef,
        width: f32,
    ) {
        let lead = format!("{count} key conflict{}.  ", if count == 1 { "" } else { "s" });
        let chord = registered
            .descriptor
            .keybinds
            .get(usize::from(own.action))
            .map(|bind| ChordText::format(model.chord(own.mod_index, bind)))
            .unwrap_or_default();
        let tail = format!(" is also bound by {}.", model.display_name(other.mod_index));
        let text = Draw::runs(
            ui.painter(),
            &[
                (&lead, Typeface::label_strong(), StudioPalette::WARNING),
                (&chord, Typeface::mono_text(), StudioPalette::TEXT),
                (&tail, Typeface::label(), StudioPalette::TEXT),
            ],
        );
        let height = text.size().y + 2.0 * Metrics::GAP_M;
        let (banner, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        let painter = ui.painter();
        Draw::fill(painter, banner, Metrics::RADIUS_CONTROL + 2, StudioPalette::WARNING_SOFT);
        Draw::outline(
            painter,
            banner,
            Metrics::RADIUS_CONTROL + 2,
            1.0,
            StudioPalette::alpha(StudioPalette::WARNING, 0.35),
        );
        let icon_centre = pos2(banner.left() + 22.0, banner.center().y);
        Draw::icon(painter, icon_centre, Icon::WARNING, Typeface::ICON_SIZE, StudioPalette::WARNING);
        Draw::place(painter, text, pos2(banner.left() + 42.0, banner.center().y), Align2::LEFT_CENTER);
        ui.add_space(Metrics::GAP_M);
    }
}
