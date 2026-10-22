use egui::{Align2, Color32, Response, Sense, Ui, Vec2, vec2};

use crate::theme::{Metrics, StudioPalette, Typeface};
use crate::widgets::Draw;

// the accent buttons in the bottom-right corner of the title and pause screens
pub struct CornerButton;

impl CornerButton {
    const ICON: f32 = 16.0;
    const MARK: f32 = 21.0;
    const MARK_RADIUS: u8 = 4;
    const MARK_TEXT: f32 = 9.5;
    const PAD_LEFT: f32 = 8.0;
    const PAD_RIGHT: f32 = 14.0;
    const GAP: f32 = 8.0;

    // a square icon-only button (the Mods button)
    pub fn icon(ui: &mut Ui, icon: char) -> Response {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(Metrics::BUTTON), Sense::click());
        let painter = ui.painter();
        Draw::fill(painter, rect, Metrics::RADIUS_CONTROL, Self::fill(&response));
        Draw::icon(painter, rect.center(), icon, Self::ICON, StudioPalette::WHITE);
        response
    }

    // the width `labelled` allocates for this label (the mark tile has a fixed size)
    pub fn labelled_width(ui: &Ui, label: &str) -> f32 {
        let galley = Draw::galley(ui.painter(), label, Typeface::label_strong(), StudioPalette::WHITE);
        Self::PAD_LEFT + Self::MARK + Self::GAP + galley.size().x + Self::PAD_RIGHT
    }

    // a mark tile (white with the mark in accent) and a label, for a mod's own entry
    pub fn labelled(ui: &mut Ui, mark: &str, label: &str) -> Response {
        let width = Self::labelled_width(ui, label);
        let (rect, response) = ui.allocate_exact_size(vec2(width, Metrics::BUTTON), Sense::click());
        let fill = Self::fill(&response);
        let painter = ui.painter();
        Draw::fill(painter, rect, Metrics::RADIUS_CONTROL, fill);
        let tile = egui::Rect::from_min_size(
            egui::pos2(rect.left() + Self::PAD_LEFT, rect.center().y - Self::MARK * 0.5),
            Vec2::splat(Self::MARK),
        );
        Draw::fill(painter, tile, Self::MARK_RADIUS, StudioPalette::WHITE);
        Draw::text(painter, tile.center(), Align2::CENTER_CENTER, mark, Typeface::semibold(Self::MARK_TEXT), fill);
        let label_pos = egui::pos2(tile.right() + Self::GAP, rect.center().y);
        Draw::text(painter, label_pos, Align2::LEFT_CENTER, label, Typeface::label_strong(), StudioPalette::WHITE);
        response
    }

    fn fill(response: &Response) -> Color32 {
        match (response.is_pointer_button_down_on(), response.hovered() || response.has_focus()) {
            (true, _) => StudioPalette::ACCENT_PRESS,
            (false, true) => StudioPalette::ACCENT_HOVER,
            (false, false) => StudioPalette::ACCENT,
        }
    }
}
