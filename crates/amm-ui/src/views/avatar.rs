use amm_core::structs::registry::ModIndex;
use egui::{Align2, Color32, Painter, Rect};
use studio_ui::theme::{StudioPalette, Typeface};
use studio_ui::widgets::Draw;

/// The coloured square with a mod's first letter. The colour follows the mod's slot.
pub struct Avatar;

impl Avatar {
    const COLOURS: [Color32; 5] = [
        StudioPalette::AXIS_Z,
        StudioPalette::ACCENT,
        StudioPalette::WARNING,
        StudioPalette::SUCCESS,
        StudioPalette::DANGER,
    ];

    pub fn colour(index: ModIndex) -> Color32 {
        Self::COLOURS[usize::from(index.0) % Self::COLOURS.len()]
    }

    pub fn paint(painter: &Painter, rect: Rect, index: ModIndex, name: &str, radius: u8, size: f32) {
        let colour = Self::colour(index);
        Draw::fill(painter, rect, radius, StudioPalette::alpha(colour, 0.16));
        let mut buffer = [0u8; 4];
        let letter = name.chars().next().map_or("?", |c| c.encode_utf8(&mut buffer));
        Draw::text(painter, rect.center(), Align2::CENTER_CENTER, letter, Typeface::semibold(size), colour);
    }
}
