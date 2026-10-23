//! Paper: Menus + Dialogs, the studio dialogs, each in the levi-ui `ModalFrame`: `Export dialog` (`971-0`),
//! `Render queue` (`9A9-0`, `9D1-0`), `Coverage panel` (`9GC-0`, `9H2-0`, `9HR-0`), `Confirm delete` (`9PB-0`) and
//! `Rename` (`9QD-0`). Each has `show(ctx)` for the screen and `show_card(ui)` for the gallery.

mod confirm_dialog;
mod coverage_panel;
mod export_dialog;
mod rename_dialog;
mod render_queue;

pub use confirm_dialog::{ConfirmAction, ConfirmDialog, ConfirmDialogView};
pub use coverage_panel::{
    CoverageFilter, CoverageGroup, CoveragePanel, CoveragePanelAction, CoveragePanelView, CoverageRow,
};
pub use export_dialog::{
    ExportDialog, ExportDialogAction, ExportDialogView, ExportFormat, ExportFrameRate, ExportResolution,
};
pub use rename_dialog::{RenameAction, RenameDialog};
pub use render_queue::{JobStatus, RenderJobRow, RenderQueue, RenderQueueAction, RenderQueueView};
