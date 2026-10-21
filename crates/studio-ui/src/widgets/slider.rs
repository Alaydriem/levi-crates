use std::ops::RangeInclusive;

use egui::{Rect, Response, Sense, Stroke, Ui, pos2, vec2};

use crate::theme::{Metrics, StudioPalette};
use crate::widgets::Draw;

// a horizontal value slider that snaps to a step from the start of its range
pub struct Slider<'a> {
    value: &'a mut f64,
    range: RangeInclusive<f64>,
    step: f64,
}

impl<'a> Slider<'a> {
    pub const HEIGHT: f32 = 20.0;
    const TRACK: f32 = 4.0;
    const THUMB: f32 = 14.0;

    pub fn new(value: &'a mut f64, range: RangeInclusive<f64>, step: f64) -> Self {
        Self { value, range, step }
    }

    // `raw` rounded to the nearest multiple of `step` above the start of `range`, clamped into `range`
    pub fn snap(raw: f64, range: &RangeInclusive<f64>, step: f64) -> f64 {
        let (start, end) = (*range.start(), *range.end());
        let snapped = if step > 0.0 { start + ((raw - start) / step).round() * step } else { raw };
        snapped.clamp(start, end)
    }

    pub fn show(self, ui: &mut Ui, width: f32) -> Response {
        let (rect, mut response) = ui.allocate_exact_size(vec2(width, Self::HEIGHT), Sense::click_and_drag());
        let (start, end) = (*self.range.start(), *self.range.end());
        let span = end - start;
        let usable = (rect.width() - Self::THUMB).max(1.0);
        if let Some(pointer) = response.interact_pointer_pos().filter(|_| span > 0.0) {
            let fraction = f64::from(((pointer.x - rect.left() - Self::THUMB * 0.5) / usable).clamp(0.0, 1.0));
            let next = Self::snap(start + fraction * span, &self.range, self.step);
            if (next - *self.value).abs() > f64::EPSILON {
                *self.value = next;
                response.mark_changed();
            }
        }
        let fraction = if span > 0.0 { ((*self.value - start) / span).clamp(0.0, 1.0) as f32 } else { 0.0 };
        Self::paint(ui, rect, fraction, response.hovered() || response.dragged());
        response
    }

    fn paint(ui: &Ui, rect: Rect, fraction: f32, lit: bool) {
        let painter = ui.painter();
        let radius = Self::THUMB * 0.5;
        let x = rect.left() + radius + (rect.width() - Self::THUMB).max(0.0) * fraction;
        let track = Rect::from_center_size(rect.center(), vec2(rect.width(), Self::TRACK));
        Draw::fill(painter, track, Metrics::RADIUS_PILL, StudioPalette::BG_FIELD);
        let filled = Rect::from_min_max(track.min, pos2(x.max(track.left()), track.max.y));
        Draw::fill(painter, filled, Metrics::RADIUS_PILL, StudioPalette::ACCENT);
        let fill = if lit { StudioPalette::WHITE } else { StudioPalette::TEXT };
        painter.circle(pos2(x, rect.center().y), radius, fill, Stroke::new(1.0, StudioPalette::LINE_STRONG));
    }
}
