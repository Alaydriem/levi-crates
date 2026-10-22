use egui::{Align2, Response, Sense, Ui, pos2, vec2};

use crate::theme::{Icon, Metrics, StudioPalette, Typeface};
use crate::widgets::Draw;

// a short success notice: a green check disc, a title and one detail line
pub struct NoticeToast;

impl NoticeToast {
    pub const HEIGHT: f32 = 58.0;
    const DISC: f32 = 22.0;
    const PAD: f32 = 14.0;
    const LINE_GAP: f32 = 2.0;

    pub fn show(ui: &mut Ui, title: &str, detail: &str) -> Response {
        let painter = ui.painter();
        let title_galley = Draw::galley(painter, title, Typeface::label_strong(), StudioPalette::TEXT);
        let detail_galley = Draw::galley(painter, detail, Typeface::note(), StudioPalette::TEXT_DIM);
        let text_width = title_galley.size().x.max(detail_galley.size().x);
        let width = Self::PAD + Self::DISC + Metrics::GAP_M + text_width + Self::PAD * 1.5;
        let (rect, response) = ui.allocate_exact_size(vec2(width, Self::HEIGHT), Sense::hover());

        let painter = ui.painter();
        Draw::card(painter, rect);
        let disc = pos2(rect.left() + Self::PAD + Self::DISC * 0.5, rect.center().y);
        painter.circle_filled(disc, Self::DISC * 0.5, StudioPalette::alpha(StudioPalette::SUCCESS, 0.16));
        Draw::icon(painter, disc, Icon::CHECK, Typeface::ICON_SMALL, StudioPalette::SUCCESS);
        let text_left = disc.x + Self::DISC * 0.5 + Metrics::GAP_M;
        let block = title_galley.size().y + Self::LINE_GAP + detail_galley.size().y;
        let top = rect.center().y - block * 0.5;
        let title_rect = Draw::place(painter, title_galley, pos2(text_left, top), Align2::LEFT_TOP);
        Draw::place(painter, detail_galley, pos2(text_left, title_rect.bottom() + Self::LINE_GAP), Align2::LEFT_TOP);
        response
    }
}
