use amm_core::services::settings::ChordText;
use egui::{Align2, Painter, Rect, ScrollArea, Sense, Ui, UiBuilder, Vec2, pos2, vec2};
use studio_ui::theme::{Icon, Metrics, StudioPalette, Typeface};
use studio_ui::widgets::{Chip, Draw, Toggle};

use crate::model::{PanelAction, PanelIntent, PanelModel};
use crate::views::{PageHeader, SettingsRowParts};

/// The Manager settings page: the panel chord, the enabled flag of each mod, and the mod limit the
/// manager read from its own config at start.
pub struct ManagerSettingsView;

impl ManagerSettingsView {
    const PAD: f32 = 10.0;
    const TEXT_GAP: f32 = 4.0;
    const INSET: f32 = 18.0;

    pub fn show(ui: &mut Ui, model: &PanelModel, out: &mut Vec<PanelIntent>) {
        PageHeader::show(
            ui,
            Icon::SETTINGS,
            StudioPalette::ACCENT_HOVER,
            "Manager settings",
            "Stored in the manager's config.hcl",
        );
        ui.add_space(Metrics::GAP_L);
        ScrollArea::vertical().id_salt("amm.manager_settings").auto_shrink([false, false]).show(ui, |ui| {
            Self::panel(ui, model);
            ui.add_space(Metrics::GAP_L + 4.0);
            Self::mods(ui, model, out);
            ui.add_space(Metrics::GAP_L + 4.0);
            Self::limits(ui, model);
            ui.add_space(Metrics::GAP_M);
            let note = "Limits are read when the game starts. Change them in the manager's config.hcl.";
            ui.label(egui::RichText::new(note).font(Typeface::help()).color(StudioPalette::TEXT_FAINT));
        });
    }

    /// The height of one row: a body label line over a help line, with padding above and below.
    pub fn row_height(painter: &Painter) -> f32 {
        let label = Draw::line_height(painter, &Typeface::body());
        let help = Draw::line_height(painter, &Typeface::help());
        Self::PAD + label + Self::TEXT_GAP + help + Self::PAD
    }

    /// The parts of row `row`: the label at the top left, the help under it, and a control of size
    /// `control` centred on the right.
    pub fn parts(painter: &Painter, row: Rect, label: &str, help: &str, control: Vec2) -> SettingsRowParts {
        let x = row.left() + Self::INSET;
        let label_size = Draw::galley(painter, label, Typeface::body(), StudioPalette::TEXT).size();
        let help_size = Draw::galley(painter, help, Typeface::help(), StudioPalette::TEXT_DIM).size();
        let label = Rect::from_min_size(pos2(x, row.top() + Self::PAD), label_size);
        let help = Rect::from_min_size(pos2(x, label.bottom() + Self::TEXT_GAP), help_size);
        let control =
            Rect::from_center_size(pos2(row.right() - Self::INSET - control.x * 0.5, row.center().y), control);
        SettingsRowParts { label, help, control }
    }

    fn panel(ui: &mut Ui, model: &PanelModel) {
        let text = ChordText::format(model.config().toggle_chord);
        let keys: Vec<&str> = text.split('+').collect();
        let size = Chip::chord_size(ui.painter(), &keys);
        let frame = Self::frame(ui, "Panel", 1);
        let parts =
            Self::row(ui, frame, 0, "Panel toggle", "Opens and closes this panel from anywhere in the game", size);
        Chip::chord(ui.painter(), parts.control.left(), parts.control.center().y, &keys, false);
    }

    fn mods(ui: &mut Ui, model: &PanelModel, out: &mut Vec<PanelIntent>) {
        let count = model.mod_count();
        let frame = Self::frame(ui, "Mods", count);
        let size = vec2(Toggle::SIZE.0, Toggle::SIZE.1);
        for (position, registered) in model.mods().enumerate() {
            let help = format!("{}  \u{b7}  v{}", registered.descriptor.id, registered.descriptor.version);
            let parts = Self::row(ui, frame, position, &registered.descriptor.display_name, &help, size);
            let mut enabled = registered.enabled;
            if Toggle::show(&mut ui.new_child(UiBuilder::new().max_rect(parts.control)), &mut enabled).changed() {
                out.push(PanelIntent::Action(PanelAction::SetEnabled(registered.index, enabled)));
            }
        }
    }

    fn limits(ui: &mut Ui, model: &PanelModel) {
        let value = model.config().budgets.max_mods.to_string();
        let size = Draw::galley(ui.painter(), &value, Typeface::mono_text(), StudioPalette::TEXT).size();
        let frame = Self::frame(ui, "Limits", 1);
        let parts = Self::row(ui, frame, 0, "Mods", "Most mods that can register", size);
        Draw::text(
            ui.painter(),
            parts.control.right_center(),
            Align2::RIGHT_CENTER,
            &value,
            Typeface::mono_text(),
            StudioPalette::TEXT,
        );
    }

    // The caps title and the card of `rows` rows; returns the card.
    fn frame(ui: &mut Ui, title: &str, rows: usize) -> Rect {
        let left = ui.cursor().left();
        let caps = Draw::caps(
            ui.painter(),
            pos2(left + 2.0, ui.cursor().top()),
            Align2::LEFT_TOP,
            title,
            StudioPalette::TEXT_FAINT,
        );
        ui.add_space(caps.height() + Metrics::GAP_S);
        let width = ui.available_width() - 8.0;
        let height = Self::row_height(ui.painter()) * rows.max(1) as f32;
        let (frame, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        Draw::fill(ui.painter(), frame, Metrics::RADIUS_PANEL - 2, StudioPalette::BG_PANEL);
        Draw::outline(ui.painter(), frame, Metrics::RADIUS_PANEL - 2, 1.0, StudioPalette::LINE);
        frame
    }

    // Paints the label and help of row `position` of a card; returns its parts.
    fn row(ui: &Ui, frame: Rect, position: usize, label: &str, help: &str, control: Vec2) -> SettingsRowParts {
        let painter = ui.painter();
        let height = Self::row_height(painter);
        let row = Rect::from_min_size(
            pos2(frame.left(), frame.top() + position as f32 * height),
            vec2(frame.width(), height),
        );
        if position > 0 {
            Draw::hline(painter, row.left() + 16.0, row.right() - 16.0, row.top(), StudioPalette::LINE);
        }
        let parts = Self::parts(painter, row, label, help, control);
        Draw::text(painter, parts.label.min, Align2::LEFT_TOP, label, Typeface::body(), StudioPalette::TEXT);
        Draw::text(painter, parts.help.min, Align2::LEFT_TOP, help, Typeface::help(), StudioPalette::TEXT_DIM);
        parts
    }
}
