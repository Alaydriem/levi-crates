use levi_i18n::text::Text;

use super::StudioMode;
use crate::component::bar::SceneSwitcherChipView;

/// What the top bar shows.
pub struct TopBarView {
    /// The replay's title, in the breadcrumb.
    pub title: Text,
    pub mode: StudioMode,
    pub can_undo: bool,
    pub can_redo: bool,
    /// The undo history list is open.
    pub history_open: bool,
    pub scene: SceneSwitcherChipView,
}
