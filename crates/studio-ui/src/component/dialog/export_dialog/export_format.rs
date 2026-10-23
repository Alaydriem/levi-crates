use levi_i18n::text::Text;

use crate::message::StudioUiMessage;

/// Paper: `Export dialog` (`971-0`), the Format row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExportFormat {
    H264,
    H265,
    ProRes422,
    PngSequence,
}

impl ExportFormat {
    pub const ALL: [Self; 4] = [Self::H264, Self::H265, Self::ProRes422, Self::PngSequence];

    pub fn label(self) -> Text {
        Text::message(match self {
            Self::H264 => StudioUiMessage::FormatH264,
            Self::H265 => StudioUiMessage::FormatH265,
            Self::ProRes422 => StudioUiMessage::FormatProRes,
            Self::PngSequence => StudioUiMessage::FormatPng,
        })
    }
}
