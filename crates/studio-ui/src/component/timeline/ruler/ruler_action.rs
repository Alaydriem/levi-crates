use egui::Pos2;

/// What a click on the ruler asks for, at a time in seconds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RulerAction {
    Seek(f64),
    /// A right-click: the ruler menu ("New scene here") at the pointer.
    OpenMenu(f64, Pos2),
}
