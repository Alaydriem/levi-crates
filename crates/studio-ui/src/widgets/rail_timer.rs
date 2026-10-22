use egui::text::{LayoutJob, TextFormat};
use egui::{Align2, Response, Sense, Ui, pos2, vec2};

use crate::theme::{StudioPalette, Typeface};
use crate::widgets::{Draw, RailTile, RailTimerState};

// the capture rail's top block: a REC or PAUSED tag over the elapsed time
pub struct RailTimer;

impl RailTimer {
    pub const HEIGHT: f32 = 48.0;
    const TAG_SIZE: f32 = 9.5;
    const TAG_SPACING: f32 = 0.76;
    const MARK: f32 = 7.0;
    const MARK_GAP: f32 = 4.0;
    const TAG_Y: f32 = 15.0;
    const TIME_Y: f32 = 33.0;

    // "mm:ss" under an hour, "h:mm:ss" from an hour
    pub fn label(elapsed_ms: u64) -> String {
        let seconds = elapsed_ms / 1000;
        let (hours, minutes, secs) = (seconds / 3600, seconds / 60 % 60, seconds % 60);
        if hours > 0 { format!("{hours}:{minutes:02}:{secs:02}") } else { format!("{minutes:02}:{secs:02}") }
    }

    pub fn show(ui: &mut Ui, state: RailTimerState, elapsed_ms: u64) -> Response {
        let (rect, response) = ui.allocate_exact_size(vec2(RailTile::SIDE, Self::HEIGHT), Sense::hover());
        let (tag, colour) = match state {
            RailTimerState::Live => ("REC", StudioPalette::RECORD),
            RailTimerState::Paused => ("PAUSED", StudioPalette::WARNING),
        };
        let painter = ui.painter();
        Draw::fill(painter, rect, RailTile::RADIUS, StudioPalette::PILL);
        Draw::outline(painter, rect, RailTile::RADIUS, 1.0, StudioPalette::alpha(colour, 0.45));

        let format = TextFormat {
            font_id: Typeface::semibold(Self::TAG_SIZE),
            color: colour,
            extra_letter_spacing: Self::TAG_SPACING,
            ..Default::default()
        };
        let galley = painter.layout_job(LayoutJob::single_section(tag.to_owned(), format));
        let row = galley.size().x + Self::MARK + Self::MARK_GAP;
        let left = rect.center().x - row * 0.5;
        let tag_y = rect.top() + Self::TAG_Y;
        let mark = egui::Rect::from_center_size(pos2(left + Self::MARK * 0.5, tag_y), vec2(Self::MARK, Self::MARK));
        match state {
            RailTimerState::Live => {
                painter.circle_filled(mark.center(), Self::MARK * 0.5, colour);
            }
            RailTimerState::Paused => Draw::fill(painter, mark, 1, colour),
        }
        Draw::place(painter, galley, pos2(mark.right() + Self::MARK_GAP, tag_y), Align2::LEFT_CENTER);

        let time_colour = if state == RailTimerState::Live { StudioPalette::TEXT } else { StudioPalette::TEXT_DIM };
        let time_pos = pos2(rect.center().x, rect.top() + Self::TIME_Y);
        Draw::text(
            painter,
            time_pos,
            Align2::CENTER_CENTER,
            &Self::label(elapsed_ms),
            Typeface::mono_small(),
            time_colour,
        );
        response
    }
}
