use egui::{Key, Rect, Shape, pos2};
use levi_ui::state::InteractionState;
use levi_ui::tokens::{ColorToken, color};
use levi_ui::widget::{Segment, Segmented};

use crate::support::{ClickProbe, PaintCapture, Sample};

struct SegmentPaint;

impl SegmentPaint {
    const RECT: Rect = Rect::from_min_max(pos2(20.0, 20.0), pos2(260.0, 54.0));

    fn segments(words: &[&str]) -> Vec<Segment> {
        words.iter().map(|word| Segment::Text(Sample::text(word))).collect()
    }

    // the divider segments painted (short vertical lines in line-strong)
    fn dividers(words: &[&str], selected: usize, target: usize, state: InteractionState) -> usize {
        let mut selected = selected;
        let shapes = PaintCapture::shapes(|painter| {
            Segmented::new(egui::Id::NULL, &mut selected, Self::segments(words)).paint(
                painter,
                Self::RECT,
                target,
                state,
            );
        });
        shapes
            .iter()
            .filter(|s| matches!(s, Shape::LineSegment { stroke, .. } if stroke.color == color::LineStrong::VALUE))
            .count()
    }
}

#[test]
fn divider_rule() {
    let three = ["24", "30", "60"];
    // selected last, rest: one divider, between 24 and 30
    assert_eq!(SegmentPaint::dividers(&three, 2, 0, InteractionState::Rest), 1);
    // the first segment hovered: that divider goes
    assert_eq!(SegmentPaint::dividers(&three, 2, 0, InteractionState::Hover), 0);
    let five = ["Recorded", "Sunrise", "Noon", "Sunset", "Night"];
    assert_eq!(SegmentPaint::dividers(&five, 2, 0, InteractionState::Rest), 2);
    let four = ["Recorded", "Clear", "Rain", "Thunder"];
    assert_eq!(SegmentPaint::dividers(&four, 0, 1, InteractionState::Rest), 2);
}

#[test]
fn selected_is_soft_and_semibold() {
    let mut selected = 1;
    let shapes = PaintCapture::shapes(|painter| {
        Segmented::new(egui::Id::NULL, &mut selected, SegmentPaint::segments(&["Linear", "Smooth"])).paint(
            painter,
            SegmentPaint::RECT,
            1,
            InteractionState::Rest,
        );
    });
    assert!(PaintCapture::rects(&shapes).iter().any(|r| r.fill == color::AccentSoft::VALUE));
    let smooth = shapes.iter().find_map(|s| match s {
        Shape::Text(t) if t.galley.text() == "Smooth" => Some(t.galley.job.sections[0].format.clone()),
        _ => None,
    });
    let format = smooth.unwrap_or_default();
    assert_eq!(format.color, color::AccentHover::VALUE);
    assert_eq!(format.font_id.family, levi_ui::font::FontFace::Semibold.family());
}

#[test]
fn click_selects() {
    let mut harness = ClickProbe::harness_with(0_usize, |ui, selected| {
        Segmented::new(egui::Id::unique(7), selected, SegmentPaint::segments(&["24", "30", "60"])).show(ui, 240.0)
    });
    let rect = harness.state().rect;
    let third = pos2(rect.right() - 20.0, rect.center().y);
    harness.hover_at(third);
    harness.drag_at(third);
    harness.run();
    harness.drop_at(third);
    harness.run();
    assert_eq!(harness.state().value, 2);
    assert_eq!(harness.state().changes, 1);
}

#[test]
fn arrow_moves_selection() {
    let mut harness = ClickProbe::harness_with(0_usize, |ui, selected| {
        Segmented::new(egui::Id::unique(7), selected, SegmentPaint::segments(&["24", "30", "60"])).show(ui, 240.0)
    });
    ClickProbe::tab_and_press(&mut harness, Key::ArrowRight);
    assert_eq!(harness.state().value, 1);
}

#[test]
fn each_arrow_moves_one_segment() {
    let mut harness = ClickProbe::harness_with(0_usize, |ui, selected| {
        let segments = SegmentPaint::segments(&["24", "30", "60", "120"]);
        Segmented::new(egui::Id::unique(7), selected, segments).show(ui, 320.0)
    });
    ClickProbe::tab_and_press(&mut harness, Key::ArrowRight);
    harness.key_press(Key::ArrowRight);
    harness.run();
    assert_eq!(harness.state().value, 2);
}
