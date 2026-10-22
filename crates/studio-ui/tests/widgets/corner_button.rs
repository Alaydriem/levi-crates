use studio_ui::theme::{Icon, Metrics};
use studio_ui::widgets::CornerButton;

use super::headless::Headless;

#[test]
fn corner_icon_button_is_36_points_square() {
    let mut headless = Headless::new();

    let rect = headless.rect(|ui| CornerButton::icon(ui, Icon::PUZZLE));

    assert_eq!(rect.size(), egui::vec2(Metrics::BUTTON, Metrics::BUTTON));
}

#[test]
fn corner_labelled_button_is_36_points_tall_and_wider_than_icon() {
    let mut headless = Headless::new();

    let rect = headless.rect(|ui| CornerButton::labelled(ui, "DV", "Dejavu"));

    assert_eq!(rect.height(), Metrics::BUTTON);
    assert!(rect.width() > Metrics::BUTTON * 2.0, "width {}", rect.width());
}

#[test]
fn corner_labelled_width_matches_measure() {
    let mut headless = Headless::new();

    let (rect, measured) = headless.frame(Vec::new(), |ui| {
        let measured = CornerButton::labelled_width(ui, "Dejavu");
        (CornerButton::labelled(ui, "DV", "Dejavu").rect, measured)
    });

    assert!((rect.width() - measured).abs() < 0.01, "{} vs {}", rect.width(), measured);
}
