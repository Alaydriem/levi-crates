/// The egui ids of the studio-ui components, used with `Id::unique`. Each component task adds
/// the ids its component needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StudioUiId {
    /// The inspector tab strip (its underline slides).
    InspectorTabs,
    /// The inspector drawer's area at the small breakpoint.
    InspectorDrawer,
    /// The easing popover's family filter (its selection slides).
    EasingFamily,
    /// The dialogs' modals (egui keeps which modal is on top).
    ExportDialog,
    RenderQueue,
    CoveragePanel,
    ConfirmDialog,
    RenameDialog,
    /// The export dialog's format, resolution and frame rate controls (their selections slide).
    ExportFormat,
    ExportResolution,
    ExportFrameRate,
    /// The text fields (egui keeps their cursor and focus).
    CoverageSearch,
    RenameField,
    /// What a drag on the lanes started as (a key move or a box), kept until the drag ends.
    TimelineDrag,
    /// The scroll areas that keep their offset: the easing presets, the saved easings and the undo history.
    EasingGrid,
    EasingSaved,
    UndoHistory,
    /// The capture rail's Discard question.
    DiscardPopover,
}
