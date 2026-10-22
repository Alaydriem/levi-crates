use levi_i18n::message::{Domain, Message};

/// The words levi-ui widgets show on their own, the same in every mod. Everything else a widget
/// shows comes from the caller as `Text`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LeviUiMessage {
    Close,
    Dismiss,
    ClearSearch,
    Now,
    SecondsAgo,
    MoreItems,
    PressAKey,
    EscCancels,
    AlsoBoundBy,
    ChangedInGame,
    InvalidColour,
    EnterANumber,
}

impl Message for LeviUiMessage {
    const DOMAIN: Domain = Domain::new("levi-ui");
    const ALL: &'static [Self] = &[
        Self::Close,
        Self::Dismiss,
        Self::ClearSearch,
        Self::Now,
        Self::SecondsAgo,
        Self::MoreItems,
        Self::PressAKey,
        Self::EscCancels,
        Self::AlsoBoundBy,
        Self::ChangedInGame,
        Self::InvalidColour,
        Self::EnterANumber,
    ];

    fn index(self) -> usize {
        self as usize
    }

    fn msgid(self) -> &'static str {
        match self {
            Self::Close => "Close",
            Self::Dismiss => "Dismiss",
            Self::ClearSearch => "Clear search",
            Self::Now => "now",
            Self::SecondsAgo => "{count} s",
            Self::MoreItems => "+{count}",
            Self::PressAKey => "Press a key",
            Self::EscCancels => "Esc cancels",
            Self::AlsoBoundBy => "Also bound by {name}",
            Self::ChangedInGame => "Changed in game to {value}",
            Self::InvalidColour => "Use #RRGGBB or #RRGGBBAA.",
            Self::EnterANumber => "Enter a number",
        }
    }

    fn context(self) -> Option<&'static str> {
        None
    }

    fn plural(self) -> Option<&'static str> {
        match self {
            Self::SecondsAgo => Some("{count} s"),
            _ => None,
        }
    }

    fn note(self) -> Option<&'static str> {
        match self {
            Self::Now => Some("A toast's age when it is less than one second old"),
            Self::SecondsAgo => Some("A toast's age in whole seconds"),
            Self::MoreItems => Some("A chip that stands for the items that do not fit in the row"),
            Self::PressAKey => Some("Shown in a key chord cell while the key is being captured"),
            Self::AlsoBoundBy => Some("{name} is the other mod that uses the same key chord"),
            Self::ChangedInGame => Some("{value} is the setting value the game wrote while an edit was open"),
            _ => None,
        }
    }
}
