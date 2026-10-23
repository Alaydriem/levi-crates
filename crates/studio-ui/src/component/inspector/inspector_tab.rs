use levi_i18n::text::Text;
use levi_ui::icon::Icon;

use crate::message::StudioUiMessage;

/// The inspector's tabs, in strip order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InspectorTab {
    Key,
    Scene,
    Players,
    Track,
}

impl InspectorTab {
    pub const ALL: [Self; 4] = [Self::Key, Self::Scene, Self::Players, Self::Track];

    pub fn label(self) -> Text {
        Text::message(match self {
            Self::Key => StudioUiMessage::InspectorKey,
            Self::Scene => StudioUiMessage::InspectorScene,
            Self::Players => StudioUiMessage::InspectorPlayers,
            Self::Track => StudioUiMessage::InspectorTrack,
        })
    }

    /// The tab's icon on the inspector rail.
    pub fn icon(self) -> Icon {
        match self {
            Self::Key => Icon::Edit,
            Self::Scene => Icon::Video,
            Self::Players => Icon::People,
            Self::Track => Icon::List,
        }
    }

    pub fn index(self) -> usize {
        Self::ALL.iter().position(|tab| *tab == self).unwrap_or(0)
    }
}
