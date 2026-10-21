use egui::{Align, Align2, Layout, Painter, Rect, Ui, UiBuilder, pos2, vec2};
use studio_ui::theme::{Icon, Metrics, StudioPalette, Typeface};
use studio_ui::widgets::{Chip, ChipStyle, Draw, IconButton};

use amm_core::structs::write::SaveStatus;

use crate::model::{PanelAction, PanelIntent, PanelModel};

/// The panel's top row: title, mod counts, LeviLamina version, the save status and close. Edits save
/// on their own, so the status says "Saving..." while a save waits, "Saved" just after it goes out and
/// "Save failed" when the writer refused it.
pub struct HeaderView;

impl HeaderView {
    const BADGE: f32 = 36.0;

    /// The height of the caps line over the title line, with a small gap.
    pub fn height(painter: &Painter) -> f32 {
        Draw::line_height(painter, &Typeface::section())
            + Draw::line_height(painter, &Typeface::title())
            + Metrics::GAP_S
    }

    pub fn show(ui: &mut Ui, rect: Rect, model: &PanelModel, out: &mut Vec<PanelIntent>) {
        let painter = ui.painter().clone();
        let badge = Rect::from_center_size(
            pos2(rect.left() + Self::BADGE * 0.5, rect.center().y),
            vec2(Self::BADGE, Self::BADGE),
        );
        Draw::fill(&painter, badge, Metrics::RADIUS_CONTROL + 2, StudioPalette::ACCENT_SOFT);
        Draw::icon(&painter, badge.center(), Icon::APPS, Typeface::ICON_BADGE, StudioPalette::ACCENT_HOVER);
        let x = badge.right() + Metrics::GAP_M;
        let caps_height = Draw::line_height(&painter, &Typeface::section());
        let title_height = Draw::line_height(&painter, &Typeface::title());
        // the caps line sits on the title line and the pair is centred in the header
        let y = rect.center().y - (caps_height + title_height) * 0.5 + caps_height;
        Draw::caps(&painter, pos2(x, y), Align2::LEFT_BOTTOM, "Alaydriem", StudioPalette::TEXT_FAINT);
        let title =
            Draw::text(&painter, pos2(x, y), Align2::LEFT_TOP, "Mod Manager", Typeface::title(), StudioPalette::TEXT);
        let summary = format!(
            "{} mods installed  \u{b7}  {} enabled  \u{b7}  LeviLamina {}",
            model.mod_count(),
            model.enabled_count(),
            model.levilamina_version()
        );
        Draw::text(
            &painter,
            pos2(title.right() + 14.0, title.center().y + 1.0),
            Align2::LEFT_CENTER,
            &summary,
            Typeface::help(),
            StudioPalette::TEXT_DIM,
        );
        let right = Rect::from_min_max(pos2(rect.center().x, rect.top()), rect.max);
        ui.scope_builder(UiBuilder::new().max_rect(right).layout(Layout::right_to_left(Align::Center)), |ui| {
            ui.spacing_mut().item_spacing.x = Metrics::GAP_S;
            if IconButton::show(ui, Icon::CLOSE, Typeface::ICON_SIZE).clicked() {
                out.push(PanelIntent::Action(PanelAction::Close));
            }
            ui.add_space(Metrics::GAP_S);
            if let Some((label, style)) = Self::status(model.save_status()) {
                Self::status_chip(ui, label, style);
            }
        });
    }

    fn status(status: SaveStatus) -> Option<(&'static str, ChipStyle)> {
        match status {
            SaveStatus::Idle => None,
            SaveStatus::Saving => Some(("Saving\u{2026}", ChipStyle::NEUTRAL)),
            SaveStatus::Saved => Some(("Saved", ChipStyle::SUCCESS.with_icon(Icon::CHECK))),
            SaveStatus::Failed => Some(("Save failed", ChipStyle::WARNING.with_icon(Icon::WARNING))),
        }
    }

    fn status_chip(ui: &mut Ui, label: &str, style: ChipStyle) {
        let width = Chip::width(ui.painter(), label, style);
        let (rect, _) = ui.allocate_exact_size(vec2(width, Chip::HEIGHT), egui::Sense::hover());
        Chip::paint(ui.painter(), rect.left(), rect.center().y, label, style);
    }
}
