use super::{ExportFormat, ExportFrameRate, ExportResolution};

/// What the export dialog asks for.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportDialogAction {
    SetFormat(ExportFormat),
    SetResolution(ExportResolution),
    SetFrameRate(ExportFrameRate),
    SetQuality(f64),
    SetMotionBlur(bool),
    SetSidecar(bool),
    /// A click on the output field: pick a folder.
    ChooseOutput,
    OpenQueue,
    AddToQueue,
    RenderNow,
    /// The close mark, Escape or a click outside.
    Cancel,
}
