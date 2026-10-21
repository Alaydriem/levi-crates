use egui::Color32;

use crate::theme::StudioPalette;

// the colours of one chip: fill, outline, text and an optional leading icon
#[derive(Clone, Copy)]
pub struct ChipStyle {
    pub fill: Color32,
    pub stroke: Color32,
    pub text: Color32,
    pub icon: Option<char>,
}

impl ChipStyle {
    pub const NEUTRAL: ChipStyle = ChipStyle {
        fill: StudioPalette::BG_FIELD,
        stroke: StudioPalette::LINE_STRONG,
        text: StudioPalette::TEXT_DIM,
        icon: None,
    };
    pub const ACCENT: ChipStyle = ChipStyle {
        fill: StudioPalette::ACCENT_SOFT,
        stroke: Color32::TRANSPARENT,
        text: StudioPalette::ACCENT_HOVER,
        icon: None,
    };
    pub const WARNING: ChipStyle = ChipStyle {
        fill: StudioPalette::WARNING_SOFT,
        stroke: Color32::TRANSPARENT,
        text: StudioPalette::WARNING,
        icon: None,
    };
    pub const DANGER: ChipStyle = ChipStyle {
        fill: StudioPalette::DANGER_SOFT,
        stroke: Color32::TRANSPARENT,
        text: StudioPalette::DANGER,
        icon: None,
    };
    pub const SUCCESS: ChipStyle = ChipStyle {
        fill: Color32::from_rgba_unmultiplied_const(0x4F, 0xD1, 0xA5, 34),
        stroke: Color32::TRANSPARENT,
        text: StudioPalette::SUCCESS,
        icon: None,
    };

    pub fn with_icon(self, icon: char) -> ChipStyle {
        ChipStyle { icon: Some(icon), ..self }
    }
}
