mod avatar_size;

pub use avatar_size::AvatarSize;

use egui::{Color32, Painter, Rect, Response, Sense, Ui, vec2};
use levi_i18n::text::UserText;

use crate::draw::{Draw, TextFit};
use crate::tokens::{ColorToken, color};

/// Paper: Mod manager, `Widget / Avatar`. A square tile with the first letter of a name, in one
/// neutral look; Paper has no per-mod colours.
pub struct Avatar {
    letter: String,
    size: AvatarSize,
    off: bool,
}

impl Avatar {
    /// The letter is the name's first character in upper case, or "?" for an empty name.
    pub fn new(name: &UserText, size: AvatarSize) -> Self {
        let letter = match name.as_str().chars().next() {
            Some(first) => first.to_uppercase().collect(),
            None => "?".to_owned(),
        };
        Self { letter, size, off: false }
    }

    /// The disabled-mod look: the letter in `--color-text-faint`.
    pub fn off(mut self, off: bool) -> Self {
        self.off = off;
        self
    }

    /// The side of the square.
    pub fn side(&self) -> f32 {
        self.size.side()
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let side = self.size.side();
        let (rect, response) = ui.allocate_exact_size(vec2(side, side), Sense::hover());
        self.paint(ui.painter(), rect);
        response
    }

    pub fn paint(&self, painter: &Painter, rect: Rect) {
        let corner = self.size.corner();
        Draw::fill(painter, rect, corner, <color::BgField as ColorToken>::VALUE);
        Draw::stroke(painter, rect, corner, <color::LineStrong as ColorToken>::VALUE);
        let colour = self.letter_color();
        let galley = Draw::galley(painter, &self.letter, self.size.letter(), colour, TextFit::Line);
        painter.galley(rect.center() - galley.size() / 2.0, galley, colour);
    }

    fn letter_color(&self) -> Color32 {
        if self.off { <color::TextFaint as ColorToken>::VALUE } else { <color::TextDim as ColorToken>::VALUE }
    }
}
