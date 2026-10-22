use egui::{Align2, Rect, Sense, Ui, pos2, vec2};
use levi_i18n::text::UserText;

use crate::draw::{Draw, TextFit, TextStyle, TypeSize};
use crate::font::FontFace;
use crate::gallery::{GalleryState, SheetGrid};
use crate::tokens::{ColorToken, SpacingToken, color, spacing};
use crate::widget::{Avatar, AvatarSize};

/// Paper: Mod manager, `Widget / Avatar` (`DGE-0`). The three sizes and the off look, each over
/// its caption, on one bottom line.
pub struct AvatarSheet;

impl AvatarSheet {
    /// Sheet chrome: the gap between the samples.
    const GAP: f32 = 40.0;
    const SAMPLES: [(&'static str, AvatarSize, bool, &'static str); 4] = [
        ("Freecam", AvatarSize::Large, false, "Large · page header"),
        ("Minimap", AvatarSize::Regular, false, "Regular · list row"),
        ("Keystrokes", AvatarSize::Small, false, "Small · players tab"),
        ("World Edit", AvatarSize::Regular, true, "Off"),
    ];

    pub fn show(ui: &mut Ui, _state: &mut GalleryState) {
        SheetGrid::page(ui, "Avatar", |ui| {
            SheetGrid::panel(ui, |ui| {
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing = vec2(Self::GAP, 0.0);
                    for (name, size, off, caption) in Self::SAMPLES {
                        Self::sample(ui, name, size, off, caption);
                    }
                });
            });
        });
    }

    // the avatar centred over its caption, with the tallest avatar's space above the caption
    fn sample(ui: &mut Ui, name: &str, size: AvatarSize, off: bool, caption: &str) {
        let style = TextStyle::new(TypeSize::Caption, FontFace::Regular);
        let caption_size = Draw::measure(ui.ctx(), caption, style);
        let tallest = AvatarSize::Large.side();
        let gap = <spacing::S as SpacingToken>::VALUE;
        let side = size.side();
        let width = caption_size.x.max(side);
        let (rect, _) = ui.allocate_exact_size(vec2(width, tallest + gap + caption_size.y), Sense::hover());
        let avatar =
            Rect::from_min_size(pos2(rect.center().x - side / 2.0, rect.top() + tallest - side), vec2(side, side));
        Avatar::new(&UserText::from_model(name.to_owned()), size).off(off).paint(ui.painter(), avatar);
        let faint = <color::TextFaint as ColorToken>::VALUE;
        let galley = Draw::galley(ui.painter(), caption, style, faint, TextFit::Line);
        let at = Align2::CENTER_BOTTOM.anchor_size(rect.center_bottom(), galley.size());
        ui.painter().galley(at.min, galley, faint);
    }
}
