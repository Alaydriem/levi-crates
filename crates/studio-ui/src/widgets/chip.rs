use egui::{Align2, Color32, Painter, Pos2, Rect, Vec2, pos2, vec2};

use crate::theme::{Metrics, StudioPalette, Typeface};
use crate::widgets::{ChipStyle, Draw};

// a small rounded label (capability chips, status pills, key caps)
pub struct Chip;

impl Chip {
    pub const HEIGHT: f32 = Typeface::CAPTION_SIZE + 10.0;
    // one key cap of a chord
    pub const KEY_HEIGHT: f32 = Typeface::MONO_SIZE + 12.0;
    // the "+" between two key caps
    const JOINT: f32 = 16.0;
    const PAD: f32 = 7.0;
    const KEY_PAD: f32 = 16.0;
    const KEY_MIN: f32 = 28.0;
    const ICON_GAP: f32 = 4.0;

    pub fn width(painter: &Painter, text: &str, style: ChipStyle) -> f32 {
        let galley = Draw::galley(painter, text, Typeface::caption(), style.text);
        galley.size().x + Self::PAD * 2.0 + Self::icon_width(style)
    }

    // paints the chip with its left edge at `left`, centred on `y`; returns its rect
    pub fn paint(painter: &Painter, left: f32, y: f32, text: &str, style: ChipStyle) -> Rect {
        let galley = Draw::galley(painter, text, Typeface::caption(), style.text);
        let icon_w = Self::icon_width(style);
        let size = vec2(galley.size().x + Self::PAD * 2.0 + icon_w, Self::HEIGHT);
        let rect = Rect::from_min_size(pos2(left, y - size.y * 0.5), size);
        Draw::fill(painter, rect, Metrics::RADIUS_SMALL, style.fill);
        if style.stroke != Color32::TRANSPARENT {
            Draw::outline(painter, rect, Metrics::RADIUS_SMALL, 1.0, style.stroke);
        }
        let mut x = rect.left() + Self::PAD;
        if let Some(icon) = style.icon {
            let centre = pos2(x + Typeface::ICON_SMALL * 0.5, rect.center().y);
            Draw::icon(painter, centre, icon, Typeface::ICON_SMALL, style.text);
            x += icon_w;
        }
        painter.galley(Pos2::new(x, rect.center().y - galley.size().y * 0.5), galley, style.text);
        rect
    }

    // the size `chord` paints for these keys: each key cap, and a joint between caps
    pub fn chord_size(painter: &Painter, keys: &[&str]) -> Vec2 {
        let caps: f32 = keys
            .iter()
            .map(|key| Self::cap_width(Draw::galley(painter, key, Typeface::mono_text(), StudioPalette::TEXT).size().x))
            .sum();
        vec2(caps + Self::JOINT * keys.len().saturating_sub(1) as f32, Self::KEY_HEIGHT)
    }

    // a key chord as separate key caps joined by "+"; returns the right edge
    pub fn chord(painter: &Painter, left: f32, y: f32, keys: &[&str], conflict: bool) -> f32 {
        let (fill, stroke, text) = if conflict {
            (StudioPalette::DANGER_SOFT, StudioPalette::alpha(StudioPalette::DANGER, 0.7), StudioPalette::DANGER)
        } else {
            (StudioPalette::BG_FIELD, StudioPalette::LINE_STRONG, StudioPalette::TEXT)
        };
        let mut x = left;
        for (index, key) in keys.iter().enumerate() {
            if index > 0 {
                let joint = pos2(x + Self::JOINT * 0.5, y);
                Draw::text(painter, joint, Align2::CENTER_CENTER, "+", Typeface::note(), StudioPalette::TEXT_FAINT);
                x += Self::JOINT;
            }
            let galley = Draw::galley(painter, key, Typeface::mono_text(), text);
            let w = Self::cap_width(galley.size().x);
            let rect = Rect::from_min_size(pos2(x, y - Self::KEY_HEIGHT * 0.5), vec2(w, Self::KEY_HEIGHT));
            Draw::fill(painter, rect.translate(vec2(0.0, 2.0)), Metrics::RADIUS_SMALL, StudioPalette::BG_PANEL);
            Draw::fill(painter, rect, Metrics::RADIUS_SMALL, fill);
            Draw::outline(painter, rect, Metrics::RADIUS_SMALL, 1.0, stroke);
            Draw::place(painter, galley, rect.center(), Align2::CENTER_CENTER);
            x += w;
        }
        x
    }

    fn cap_width(text_width: f32) -> f32 {
        (text_width + Self::KEY_PAD).max(Self::KEY_MIN)
    }

    fn icon_width(style: ChipStyle) -> f32 {
        if style.icon.is_some() { Typeface::ICON_SMALL + Self::ICON_GAP } else { 0.0 }
    }
}
