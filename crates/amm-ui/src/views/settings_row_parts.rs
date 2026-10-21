use egui::Rect;

/// Where the label, the help and the control of one settings row sit, in screen points.
#[derive(Clone, Copy, Debug)]
pub struct SettingsRowParts {
    pub label: Rect,
    pub help: Rect,
    pub control: Rect,
}
