use egui::{Align2, Color32, Response, Sense, Ui, Vec2};

use crate::theme::{StudioPalette, Typeface};
use crate::widgets::{Draw, Motion, RailIcon, RailTone};

// one square tile of the capture rail: an icon over a short label
pub struct RailTile;

impl RailTile {
    pub const SIDE: f32 = 58.0;
    pub const RADIUS: u8 = 8;
    pub const ICON: f32 = 16.0;
    const DOT_RADIUS: f32 = 8.0;
    const ICON_Y: f32 = 22.0;
    const LABEL_Y: f32 = 15.0;
    const DISABLED_ALPHA: f32 = 0.45;

    pub fn show(ui: &mut Ui, icon: RailIcon, label: &str, tone: RailTone, enabled: bool) -> Response {
        let sense = if enabled { Sense::click() } else { Sense::hover() };
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(Self::SIDE), sense);
        let (hover, press) = if enabled { Motion::states(ui.ctx(), &response) } else { (0.0, 0.0) };
        let (fill, line, ink) = Self::colours(tone, hover, press);
        let alpha = if enabled { 1.0 } else { Self::DISABLED_ALPHA };
        let painter = ui.painter();
        Draw::fill(painter, rect, Self::RADIUS, StudioPalette::alpha(fill, alpha));
        Draw::outline(painter, rect, Self::RADIUS, 1.0, StudioPalette::alpha(line, alpha));
        if enabled && response.has_focus() {
            Draw::outline(painter, rect, Self::RADIUS, 2.0, StudioPalette::ACCENT);
        }
        let icon_centre = egui::pos2(rect.center().x, rect.top() + Self::ICON_Y);
        match icon {
            RailIcon::Glyph(glyph) => {
                Draw::icon(painter, icon_centre, glyph, Self::ICON, StudioPalette::alpha(ink, alpha));
            }
            RailIcon::RecordDot => {
                let dot = StudioPalette::mix(StudioPalette::RECORD, StudioPalette::RECORD_HOVER, hover);
                painter.circle_filled(icon_centre, Self::DOT_RADIUS, StudioPalette::alpha(dot, alpha));
            }
        }
        let label_pos = egui::pos2(rect.center().x, rect.bottom() - Self::LABEL_Y);
        let label_ink = if tone == RailTone::Active { StudioPalette::TEXT } else { ink };
        let font = Typeface::semibold(Typeface::CAPTION_SIZE);
        Draw::text(painter, label_pos, Align2::CENTER_CENTER, label, font, StudioPalette::alpha(label_ink, alpha));
        response
    }

    // (fill, outline, icon and label) for a tone at the given hover and press amounts
    fn colours(tone: RailTone, hover: f32, press: f32) -> (Color32, Color32, Color32) {
        let (rest, hot, line, hot_line, ink) = match tone {
            RailTone::Normal => (
                StudioPalette::CARD,
                StudioPalette::BG_HOVER,
                StudioPalette::LINE_OVERLAY,
                StudioPalette::LINE_STRONG,
                StudioPalette::TEXT,
            ),
            RailTone::Active => (
                StudioPalette::ACCENT_SOFT,
                StudioPalette::alpha(StudioPalette::ACCENT, 0.3),
                StudioPalette::alpha(StudioPalette::ACCENT, 0.55),
                StudioPalette::ACCENT,
                StudioPalette::ACCENT_HOVER,
            ),
            RailTone::Danger => (
                StudioPalette::CARD,
                StudioPalette::DANGER_SOFT,
                StudioPalette::LINE_OVERLAY,
                StudioPalette::alpha(StudioPalette::DANGER, 0.6),
                StudioPalette::DANGER,
            ),
        };
        let fill = StudioPalette::mix(StudioPalette::mix(rest, hot, hover), StudioPalette::BG_ACTIVE, press * 0.6);
        (fill, StudioPalette::mix(line, hot_line, hover), ink)
    }
}
