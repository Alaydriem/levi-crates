use egui::{Rect, Ui, UiBuilder, pos2, vec2};
use studio_ui::theme::Metrics;
use studio_ui::widgets::TextButton;

use crate::model::{PanelAction, PanelIntent};

/// The "Mods" button in the bottom-right corner while the game shows its own cursor. It sits above
/// the game's version text, so it never covers it. A click asks to open the panel.
pub struct MenuButtonView;

impl MenuButtonView {
    pub const LABEL: &'static str = "Mods";
    /// From the screen's right edge to the button's right edge, in points.
    pub const RIGHT_MARGIN: f32 = 24.0;
    /// From the screen's bottom edge to the button's bottom edge, in points: clear of the version text.
    pub const BOTTOM_MARGIN: f32 = 48.0;
    pub const WIDTH: f32 = 96.0;

    /// The button's rect on a screen, in points.
    pub fn rect(screen: Rect) -> Rect {
        let max = pos2(screen.right() - Self::RIGHT_MARGIN, screen.bottom() - Self::BOTTOM_MARGIN);
        Rect::from_min_max(max - vec2(Self::WIDTH, Metrics::BUTTON), max)
    }

    pub fn show(ui: &mut Ui, screen: Rect, out: &mut Vec<PanelIntent>) {
        let rect = Self::rect(screen);
        let clicked =
            ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| TextButton::primary(ui, Self::LABEL)).inner;
        if clicked.clicked() {
            out.push(PanelIntent::Action(PanelAction::Open));
        }
    }
}
