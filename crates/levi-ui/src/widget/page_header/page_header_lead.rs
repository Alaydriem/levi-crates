use egui::epaint::RectShape;
use egui::{Color32, CornerRadius, Painter, Rect};
use levi_i18n::text::UserText;

use crate::draw::{Corner, Draw};
use crate::icon::{Icon, IconSize};
use crate::tokens::{SpacingToken, spacing};
use crate::widget::{Avatar, AvatarImage, AvatarSize, ChipTone};

/// The part at the left of a page header.
pub enum PageHeaderLead {
    /// An icon on a tile in a chip tone's fill and text colour.
    Tile(Icon, ChipTone),
    /// A large avatar with the name's first letter, or the picture when there is one.
    Avatar(UserText, Option<AvatarImage>),
    /// A picture on the tile's square with no fill or border, for a mark that brings its own shape.
    Picture(AvatarImage),
}

impl PageHeaderLead {
    /// The side of the square. The panel form's tile is `--spacing-tile`; every other lead is
    /// `--spacing-tile-large`.
    pub fn side(&self, panel: bool) -> f32 {
        match (self, panel) {
            (Self::Tile(..) | Self::Picture(_), true) => <spacing::Tile as SpacingToken>::VALUE,
            (Self::Tile(..) | Self::Picture(_), false) => <spacing::TileLarge as SpacingToken>::VALUE,
            (Self::Avatar(..), _) => AvatarSize::Large.side(),
        }
    }

    pub fn paint(&self, painter: &Painter, rect: Rect, panel: bool) {
        match self {
            Self::Tile(icon, tone) => {
                let corner = if panel { Corner::Control } else { Corner::Panel };
                Draw::fill(painter, rect, corner, tone.fill());
                icon.paint(painter, rect, IconSize::Regular, tone.text());
            }
            Self::Avatar(name, image) => Avatar::new(name, AvatarSize::Large).image(image.clone()).paint(painter, rect),
            Self::Picture(image) => {
                painter.add(RectShape {
                    brush: Some(image.brush()),
                    ..RectShape::filled(rect, CornerRadius::ZERO, Color32::WHITE)
                });
            }
        }
    }
}
