use egui::Response;

/// What a list row reported this frame.
#[derive(Debug, Clone)]
pub struct ListRowResponse {
    /// The row itself: a click selects it. A click on the toggle is not a row click.
    pub row: Response,
    /// The toggle was switched.
    pub toggled: bool,
}
