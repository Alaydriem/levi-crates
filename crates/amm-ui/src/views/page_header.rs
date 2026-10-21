use egui::{Align2, Color32, Painter, Rect, Sense, Ui, pos2, vec2};
use studio_ui::theme::{Metrics, StudioPalette, Typeface};
use studio_ui::widgets::Draw;

/// The icon badge, title and subtitle at the top of the Conflicts and Manager settings pages.
pub struct PageHeader;

impl PageHeader {
    /// The height of a heading line over a help line, with a small gap: the badge is this square.
    pub fn height(painter: &Painter) -> f32 {
        Draw::line_height(painter, &Typeface::heading())
            + Draw::line_height(painter, &Typeface::help())
            + Metrics::GAP_S
    }

    pub fn show(ui: &mut Ui, icon: char, tint: Color32, title: &str, subtitle: &str) -> Rect {
        let height = Self::height(ui.painter());
        let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
        let painter = ui.painter();
        let badge = Rect::from_min_size(rect.min, vec2(height, height));
        Draw::fill(painter, badge, Metrics::RADIUS_PANEL, StudioPalette::alpha(tint, 0.16));
        Draw::icon(painter, badge.center(), icon, Typeface::ICON_HERO, tint);
        let x = badge.right() + Metrics::GAP_M + 2.0;
        Draw::text(
            painter,
            pos2(x, rect.top() + 1.0),
            Align2::LEFT_TOP,
            title,
            Typeface::heading(),
            StudioPalette::TEXT,
        );
        Draw::text(
            painter,
            pos2(x, rect.bottom() - 2.0),
            Align2::LEFT_BOTTOM,
            subtitle,
            Typeface::help(),
            StudioPalette::TEXT_DIM,
        );
        rect
    }
}
