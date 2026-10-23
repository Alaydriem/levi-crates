mod key;
mod snap_magnet;
mod spinner;
mod spinner_size;
mod swatch_dot;

pub use key::{KeyAuto, KeyCustom, KeyEased, KeyHold, KeyLook};
pub use snap_magnet::SnapMagnet;
pub use spinner::Spinner;
pub use spinner_size::SpinnerSize;
pub use swatch_dot::SwatchDot;

use egui::{Color32, Painter, Rect};

/// A painted shape that a widget can lead with, such as a key glyph on a button.
///
/// levi-ui widgets take `&dyn Glyph`, so glyphs from other crates (studio-ui key shapes) can lead a
/// levi-ui widget without levi-ui depending on them.
pub trait Glyph {
    fn paint(&self, painter: &Painter, rect: Rect, color: Color32);
}

impl Glyph for SwatchDot {
    fn paint(&self, painter: &Painter, rect: Rect, color: Color32) {
        Self::paint(painter, rect, color);
    }
}

impl Glyph for SnapMagnet {
    fn paint(&self, painter: &Painter, rect: Rect, color: Color32) {
        Self::paint(painter, rect, color);
    }
}

impl Glyph for Spinner {
    fn paint(&self, painter: &Painter, rect: Rect, color: Color32) {
        Spinner::paint(self, painter, rect, color);
    }
}
