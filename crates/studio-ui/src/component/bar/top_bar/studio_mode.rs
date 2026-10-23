/// The studio's mode, set only by the top bar's Edit / Preview switch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StudioMode {
    /// The free camera; keys and scenes can change.
    Edit,
    /// The scene camera drives; edits are locked.
    Preview,
}
