use egui::{Color32, Rect, Shape, pos2};
use levi_ui::state::InteractionState;
use levi_ui::tokens::{ColorToken, color};
use levi_ui::widget::{SettingControl, SettingRow};

use crate::support::{ClickProbe, PaintCapture, Sample};

#[test]
fn row_click_flips_the_toggle() {
    let mut harness = ClickProbe::harness_with(false, |ui, on| {
        SettingRow::new(Sample::text("Hide chat"), SettingControl::Toggle(on)).show(ui)
    });
    // click the label side of the row, far from the switch
    let left = harness.state().rect.left_center() + egui::vec2(20.0, 0.0);
    harness.hover_at(left);
    harness.drag_at(left);
    harness.run();
    harness.drop_at(left);
    harness.run();
    assert!(harness.state().value);
    assert_eq!(harness.state().changes, 1);
}

#[test]
fn row_is_field_height() {
    let harness = ClickProbe::harness_with(false, |ui, on| {
        SettingRow::new(Sample::text("Hide chat"), SettingControl::Toggle(on)).show(ui)
    });
    assert_eq!(harness.state().rect.height(), 34.0);
}

#[test]
fn row_hover_shows_hovered_toggle() {
    let rect = Rect::from_min_max(pos2(10.0, 10.0), pos2(380.0, 44.0));
    let mut on = true;
    let shapes = PaintCapture::shapes(|painter| {
        SettingRow::new(Sample::text("Hide chat"), SettingControl::Toggle(&mut on)).paint(
            painter,
            rect,
            InteractionState::Hover,
        );
    });
    let rects = PaintCapture::rects(&shapes);
    assert!(rects.iter().any(|r| r.rect == rect && r.fill == color::BgHover::VALUE), "row has no hover fill");
    assert!(rects.iter().any(|r| r.fill == color::AccentHover::VALUE), "toggle is not hovered");
    let knob = shapes.iter().any(|s| matches!(s, Shape::Circle(c) if c.fill == color::OnAccent::VALUE));
    assert!(knob);
}

#[test]
fn rest_row_has_no_fill() {
    let rect = Rect::from_min_max(pos2(10.0, 10.0), pos2(380.0, 44.0));
    let mut on = false;
    let shapes = PaintCapture::shapes(|painter| {
        SettingRow::new(Sample::text("Hide commands"), SettingControl::Toggle(&mut on)).paint(
            painter,
            rect,
            InteractionState::Rest,
        );
    });
    let fill = PaintCapture::rects(&shapes).into_iter().find(|r| r.rect == rect).map(|r| r.fill);
    assert!(fill.is_none() || fill == Some(Color32::TRANSPARENT));
}
