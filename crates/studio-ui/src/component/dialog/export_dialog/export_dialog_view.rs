use levi_i18n::text::Text;

use super::{ExportFormat, ExportFrameRate, ExportResolution};

/// What the export dialog shows. The words beside the controls come from the caller: the quality name (as in
/// "High"), the motion blur and sidecar notes, the output folder and file, and the note under the settings.
#[derive(Debug, Clone)]
pub struct ExportDialogView {
    /// The line under the title, as in "Arrival · 00:00:28.80 → 00:00:37.40 · output 6.2 s".
    pub subtitle: Text,
    pub format: ExportFormat,
    pub resolution: ExportResolution,
    pub frame_rate: ExportFrameRate,
    /// From 0 to 1.
    pub quality: f64,
    pub quality_label: Text,
    pub motion_blur: bool,
    pub motion_blur_note: Text,
    pub sidecar: bool,
    pub sidecar_note: Text,
    pub folder: Text,
    pub file: Text,
    pub note: Text,
}
