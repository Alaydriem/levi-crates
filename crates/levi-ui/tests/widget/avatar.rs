use egui::{Color32, Rect, Shape, pos2, vec2};
use levi_i18n::text::UserText;
use levi_ui::draw::Corner;
use levi_ui::tokens::{ColorToken, color};
use levi_ui::widget::{Avatar, AvatarSize};

use crate::support::PaintCapture;

struct AvatarPaint;

impl AvatarPaint {
    fn shapes(name: &str, size: AvatarSize, off: bool) -> (Rect, Vec<Shape>) {
        let side = size.side();
        let rect = Rect::from_min_size(pos2(20.0, 20.0), vec2(side, side));
        let avatar = Avatar::new(&UserText::from_model(name.to_owned()), size).off(off);
        (rect, PaintCapture::shapes(|painter| avatar.paint(painter, rect)))
    }

    // (text, colour) of every text shape
    fn letters(shapes: &[Shape]) -> Vec<(String, Color32)> {
        shapes
            .iter()
            .filter_map(|shape| match shape {
                Shape::Text(text) => Some((text.galley.text().to_owned(), text.galley.job.sections[0].format.color)),
                _ => None,
            })
            .collect()
    }
}

#[test]
fn letter_is_first_char_upper() {
    for (name, letter) in [("freecam", "F"), ("", "?"), ("ßeta", "SS")] {
        let (_, shapes) = AvatarPaint::shapes(name, AvatarSize::Regular, false);
        let letters: Vec<_> = AvatarPaint::letters(&shapes).into_iter().map(|(text, _)| text).collect();
        assert_eq!(letters, vec![letter.to_owned()], "{name:?}");
    }
}

#[test]
fn off_uses_faint_letter() {
    let (_, on) = AvatarPaint::shapes("Minimap", AvatarSize::Regular, false);
    assert_eq!(AvatarPaint::letters(&on)[0].1, color::TextDim::VALUE);
    let (rect, off) = AvatarPaint::shapes("World Edit", AvatarSize::Regular, true);
    assert_eq!(AvatarPaint::letters(&off)[0].1, color::TextFaint::VALUE);
    // only the letter changes
    let tile = PaintCapture::rects(&off).into_iter().find(|r| r.rect == rect);
    assert_eq!(tile.map(|r| r.fill), Some(color::BgField::VALUE));
}

#[test]
fn large_uses_panel_corner() {
    for (size, corner) in [(AvatarSize::Large, Corner::Panel), (AvatarSize::Regular, Corner::Control)] {
        let (rect, shapes) = AvatarPaint::shapes("Freecam", size, false);
        let tile = PaintCapture::rects(&shapes).into_iter().find(|r| r.rect == rect && r.fill == color::BgField::VALUE);
        let Some(tile) = tile else { panic!("{size:?}: no tile on the rect") };
        assert_eq!(tile.corner_radius, corner.radius(rect), "{size:?}");
    }
}
