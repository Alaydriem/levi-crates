use egui::{Align2, Sense, Ui, pos2, vec2};
use studio_ui::theme::{Metrics, StudioPalette, Typeface};
use studio_ui::widgets::Draw;

/// The caps titles above a table, each at a fraction of the table width.
pub struct TableHeader;

impl TableHeader {
    pub const INSET: f32 = 14.0;

    pub fn show(ui: &mut Ui, width: f32, titles: &[&str], cols: &[f32]) {
        let height = Draw::line_height(ui.painter(), &Typeface::section()) + Metrics::GAP_M;
        let (head, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        for (title, col) in titles.iter().zip(cols) {
            let x = Self::column(head.left(), width, *col);
            Draw::caps(ui.painter(), pos2(x, head.center().y), Align2::LEFT_CENTER, title, StudioPalette::TEXT_FAINT);
        }
        Draw::hline(ui.painter(), head.left(), head.right(), head.bottom(), StudioPalette::LINE_STRONG);
        ui.add_space(4.0);
    }

    /// The left edge of a column at fraction `col` of `width`; the first column is inset.
    pub fn column(left: f32, width: f32, col: f32) -> f32 {
        if col == 0.0 { left + Self::INSET } else { left + width * col }
    }
}
