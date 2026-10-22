use studio_ui::theme::Icon;
use studio_ui::widgets::{RailIcon, RailTile, RailTone};

use super::headless::Headless;

#[test]
fn rail_tile_is_58_points_square() {
    let mut headless = Headless::new();

    let rect = headless.rect(|ui| RailTile::show(ui, RailIcon::Glyph(Icon::PAUSE), "Pause", RailTone::Normal, true));

    assert_eq!(rect.size(), egui::vec2(RailTile::SIDE, RailTile::SIDE));
    assert_eq!(RailTile::SIDE, 58.0);
}

#[test]
fn enabled_rail_tile_reports_click() {
    let mut headless = Headless::new();
    let rect = headless.rect(|ui| RailTile::show(ui, RailIcon::RecordDot, "Record", RailTone::Normal, true));

    let clicked = headless.click(Headless::centre(rect), |ui| {
        RailTile::show(ui, RailIcon::RecordDot, "Record", RailTone::Normal, true).clicked()
    });

    assert!(clicked);
}

#[test]
fn disabled_rail_tile_ignores_click() {
    let mut headless = Headless::new();
    let rect = headless.rect(|ui| RailTile::show(ui, RailIcon::Glyph(Icon::PAUSE), "Pause", RailTone::Normal, false));

    let clicked = headless.click(Headless::centre(rect), |ui| {
        RailTile::show(ui, RailIcon::Glyph(Icon::PAUSE), "Pause", RailTone::Normal, false).clicked()
    });

    assert!(!clicked);
}
