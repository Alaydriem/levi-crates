use egui::epaint::Shadow;
use egui::{Color32, Context, Painter, Rect, vec2};
use levi_i18n::text::Text;

use super::KeyCapSize;
use crate::draw::{Corner, Draw, TextFit};
use crate::text::UiText;
use crate::tokens::{ColorToken, SpacingToken, color, spacing};

/// One key of a key chord (Paper: Chip sheet, "Key chord"; Tooltip sheet, shortcut).
pub struct KeyCap {
    pub text: Text,
    pub size: KeyCapSize,
    pub conflict: bool,
}

impl KeyCap {
    /// Paper: Chip sheet, the cap's hard under-edge, `0 2px 0 --color-shadow`.
    const UNDER_EDGE: i8 = 2;

    pub fn desired_width(&self, ctx: &Context) -> f32 {
        let label = Draw::measure(ctx, &UiText::resolve(ctx, &self.text), self.size.text()).x;
        (label + <spacing::S as SpacingToken>::VALUE * 2.0).max(self.size.min_width()).ceil()
    }

    pub fn paint(&self, painter: &Painter, rect: Rect) {
        let (fill, border, text) = self.colors();
        if self.size == KeyCapSize::Regular {
            let edge = Shadow {
                offset: [0, Self::UNDER_EDGE],
                blur: 0,
                spread: 0,
                color: <color::Shadow as ColorToken>::VALUE,
            };
            painter.add(edge.as_shape(rect, Corner::Small.radius(rect)));
        }
        Draw::fill(painter, rect, Corner::Small, fill);
        if let Some(border) = border {
            Draw::stroke(painter, rect, Corner::Small, border);
        }
        let label = UiText::resolve(painter.ctx(), &self.text);
        let galley = Draw::galley(painter, &label, self.size.text(), text, TextFit::Line);
        painter.galley(rect.center() - galley.size() / 2.0, galley, text);
    }

    pub fn size(&self, ctx: &Context) -> egui::Vec2 {
        vec2(self.desired_width(ctx), self.size.height())
    }

    // (fill, border, text)
    fn colors(&self) -> (Color32, Option<Color32>, Color32) {
        match (self.size, self.conflict) {
            (KeyCapSize::Small, _) => {
                (<color::BgField as ColorToken>::VALUE, None, <color::TextDim as ColorToken>::VALUE)
            }
            (KeyCapSize::Regular, false) => (
                <color::BgField as ColorToken>::VALUE,
                Some(<color::LineStrong as ColorToken>::VALUE),
                <color::Text as ColorToken>::VALUE,
            ),
            (KeyCapSize::Regular, true) => (
                <color::DangerSoft as ColorToken>::VALUE,
                Some(<color::Danger as ColorToken>::VALUE),
                <color::Danger as ColorToken>::VALUE,
            ),
        }
    }
}
