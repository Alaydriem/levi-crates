use levi_i18n::text::Text;
use levi_ui::widget::PillTone;

use crate::message::StudioUiMessage;

/// Paper: `Render queue` (`9A9-0`, `9D1-0`). Where a render job is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JobStatus {
    Rendering,
    Queued,
    Paused,
    Cancelled,
    Done,
    Failed,
}

impl JobStatus {
    /// The status pill's colour.
    pub fn tone(self) -> PillTone {
        match self {
            Self::Rendering => PillTone::Accent,
            Self::Queued => PillTone::Dim,
            Self::Paused => PillTone::Warning,
            Self::Cancelled => PillTone::Faint,
            Self::Done => PillTone::Success,
            Self::Failed => PillTone::Danger,
        }
    }

    pub fn label(self) -> Text {
        Text::message(match self {
            Self::Rendering => StudioUiMessage::JobRendering,
            Self::Queued => StudioUiMessage::JobQueued,
            Self::Paused => StudioUiMessage::JobPaused,
            Self::Cancelled => StudioUiMessage::JobCancelled,
            Self::Done => StudioUiMessage::JobDone,
            Self::Failed => StudioUiMessage::JobFailed,
        })
    }
}
