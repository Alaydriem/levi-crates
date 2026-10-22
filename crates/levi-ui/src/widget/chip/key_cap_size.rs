use crate::draw::{TextStyle, TypeSize};
use crate::font::FontFace;
use crate::tokens::{SpacingToken, spacing};

/// The two key cap looks in Paper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum KeyCapSize {
    /// Chip sheet, "Key chord": `--spacing-status-bar` high, help text, border and an under-edge.
    #[default]
    Regular,
    /// Tooltip sheet, "With shortcut": `--spacing-control-small` high, micro semibold, no border.
    Small,
}

impl KeyCapSize {
    pub fn height(self) -> f32 {
        match self {
            Self::Regular => <spacing::StatusBar as SpacingToken>::VALUE,
            Self::Small => <spacing::ControlSmall as SpacingToken>::VALUE,
        }
    }

    /// The narrowest a cap gets (a one-letter key).
    pub fn min_width(self) -> f32 {
        match self {
            Self::Regular => <spacing::Row as SpacingToken>::VALUE,
            Self::Small => 0.0,
        }
    }

    pub fn text(self) -> TextStyle {
        match self {
            Self::Regular => TextStyle::new(TypeSize::Help, FontFace::Regular),
            Self::Small => TextStyle::new(TypeSize::Micro, FontFace::Semibold),
        }
    }
}
