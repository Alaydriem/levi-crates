use egui::{Color32, Rect, Shape, pos2, vec2};
use levi_ui::state::InteractionState;
use levi_ui::tokens::{ColorToken, SpacingToken, color, spacing};
use studio_ui::theme::Icon;
use studio_ui::widgets::{RailIcon, RailTile, RailTone};

use super::headless::Headless;
use crate::support::PaintCapture;

// the tile's rect in the paint tests
fn tile() -> Rect {
    Rect::from_min_size(pos2(20.0, 20.0), vec2(RailTile::SIDE, RailTile::SIDE))
}

// what a Pause tile paints in `state`
fn paint(state: InteractionState) -> Vec<Shape> {
    PaintCapture::shapes(|painter| {
        RailTile::paint(painter, tile(), RailIcon::Glyph(Icon::PAUSE), "Pause", RailTone::Normal, state)
    })
}

// the fill of the tile's own rect (not its blurred shadow or its border)
fn fill(shapes: &[Shape]) -> Color32 {
    shapes
        .iter()
        .find_map(|shape| match shape {
            Shape::Rect(rect) if rect.rect == tile() && rect.blur_width == 0.0 && rect.fill.a() > 0 => Some(rect.fill),
            _ => None,
        })
        .unwrap_or(Color32::TRANSPARENT)
}

#[test]
fn rail_tile_side_is_the_large_tile_token() {
    let mut headless = Headless::new();

    let rect = headless.rect(|ui| RailTile::show(ui, RailIcon::Glyph(Icon::PAUSE), "Pause", RailTone::Normal, true));

    assert_eq!(RailTile::SIDE, <spacing::TileLarge as SpacingToken>::VALUE);
    assert_eq!(rect.size(), egui::vec2(RailTile::SIDE, RailTile::SIDE));
}

#[test]
fn rail_tile_states_follow_the_state_model() {
    // Paper `A89-0`: rest is `--color-card`, hover `--color-bg-hover`, pressed `--color-bg-active`
    assert_eq!(fill(&paint(InteractionState::Rest)), <color::Card as ColorToken>::VALUE);
    assert_eq!(fill(&paint(InteractionState::Hover)), <color::BgHover as ColorToken>::VALUE);
    assert_eq!(fill(&paint(InteractionState::Pressed)), <color::BgActive as ColorToken>::VALUE);
    // focus adds the ring; disabled fades the tile to `--opacity-muted`
    let ring = <color::FocusRing as ColorToken>::VALUE;
    let focused = paint(InteractionState::Focused);
    assert!(focused.iter().any(|shape| matches!(shape, Shape::Rect(rect) if rect.fill == ring)), "no focus ring");
    let disabled = fill(&paint(InteractionState::Disabled));
    assert!(disabled.a() < <color::Card as ColorToken>::VALUE.a(), "{disabled:?}");
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
