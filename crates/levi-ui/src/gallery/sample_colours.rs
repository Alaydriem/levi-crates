use egui::Color32;

use crate::tokens::{ColorToken, color};

/// The gallery's stand-ins for the six scene colours. The scene colours are studio-ui tokens, so a levi-ui page shows
/// six of its own colours in their place.
pub struct SampleColours;

impl SampleColours {
    pub fn scene() -> [Color32; 6] {
        [
            <color::Success as ColorToken>::VALUE,
            <color::Danger as ColorToken>::VALUE,
            <color::AccentHover as ColorToken>::VALUE,
            <color::Accent as ColorToken>::VALUE,
            <color::Warning as ColorToken>::VALUE,
            <color::TextDim as ColorToken>::VALUE,
        ]
    }
}
