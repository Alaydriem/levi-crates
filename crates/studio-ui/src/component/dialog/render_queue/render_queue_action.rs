use egui::Pos2;

/// What the render queue asks for. Job indexes are into `RenderQueueView::jobs`. Pause, resume and cancel of one job
/// are items of the job menu, which the host opens (Plan 08).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenderQueueAction {
    Retry(usize),
    ShowFile(usize),
    OpenJobMenu(usize, Pos2),
    ClearFinished,
    PauseAll,
    ResumeAll,
    /// The Close button, the close mark, Escape or a click outside.
    Close,
}
