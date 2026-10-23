use amm_core::structs::registry::ModIndex;

use crate::model::PanelTab;

/// The egui ids the mod manager panel keeps state under. An id is `Id::unique(AmmId::…)`, never a salt string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AmmId {
    /// The sidebar's scroll area of installed mods.
    ModList,
    /// The scroll area of one tab of one mod's page.
    Pane(ModIndex, PanelTab),
    /// The tab row of a mod's page.
    Tabs,
    /// The Conflicts page's scroll area.
    Conflicts,
    /// The resolution dropdown of the clash card at this position.
    Resolution(usize),
    /// The Keybinds page's scroll area.
    Keybinds,
    /// The Manager settings page's scroll area.
    ManagerSettings,
    /// The language dropdown on the Manager settings page.
    Language,
    /// The layer the toast stack draws on.
    Toasts,
    /// The control of the setting field at this position on a mod's General tab.
    Setting(usize),
}
