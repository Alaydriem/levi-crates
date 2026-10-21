/// The tabs of a mod's page.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PanelTab {
    #[default]
    General,
    Keybinds,
    Capabilities,
}
