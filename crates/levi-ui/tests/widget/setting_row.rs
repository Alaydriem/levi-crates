use egui::{Color32, Rect, Shape, pos2};
use levi_ui::draw::TypeSize;
use levi_ui::state::InteractionState;
use levi_ui::tokens::{ColorToken, color};
use levi_ui::widget::{Chip, ChipKind, ChipTone, SettingControl, SettingRow, Toggle};

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

#[test]
fn a_chip_with_no_room_beside_the_label_goes_under_it() {
    let harness = ClickProbe::harness_with(true, |ui, on| {
        ui.allocate_ui(egui::vec2(280.0, 400.0), |ui| {
            let chip = Chip::new(ChipKind::Display(ChipTone::Outline), Sample::text("Applies after world reload"));
            SettingRow::new(Sample::text("Scan radius"), SettingControl::Toggle(on)).chip(chip).show(ui)
        })
        .inner
    });
    let label = ClickProbe::text_rect(&harness, "Scan radius").unwrap_or_else(|| panic!("no label"));
    let chip = ClickProbe::text_rect(&harness, "Applies after world reload").unwrap_or_else(|| panic!("no chip"));

    assert!(label.height() < 1.5 * TypeSize::Body.line_height(), "the label wraps: {label:?}");
    assert!(chip.top() >= label.bottom(), "the chip {chip:?} is not under the label {label:?}");
}

#[test]
fn a_chip_under_its_label_is_cut_to_the_text_column() {
    let harness = ClickProbe::harness_with(true, |ui, on| {
        ui.allocate_ui(egui::vec2(200.0, 400.0), |ui| {
            let words = "Applies after the world reloads, and only then";
            let chip = Chip::new(ChipKind::Display(ChipTone::Outline), Sample::text(words));
            SettingRow::new(Sample::text("Scan radius"), SettingControl::Toggle(on)).chip(chip).show(ui)
        })
        .inner
    });
    let row = harness.state().rect;
    let chip = ClickProbe::text_rect(&harness, "Applies after the world reloads, and only then")
        .unwrap_or_else(|| panic!("no chip"));

    // the text column ends before the toggle
    assert!(chip.right() <= row.right() - Toggle::TRACK.x, "the chip {chip:?} reaches the toggle in {row:?}");
}
