use egui::{Event, Key, Rect, Shape, pos2};
use levi_ui::state::InteractionState;
use levi_ui::tokens::{ColorToken, color};
use levi_ui::widget::{FieldLook, TextField, TextFieldKind};

use crate::support::{ClickProbe, PaintCapture, Sample, ShapeText};

struct FieldPaint;

impl FieldPaint {
    const RECT: Rect = Rect::from_min_max(pos2(20.0, 20.0), pos2(300.0, 54.0));

    fn shapes(value: &str, kind: TextFieldKind, state: InteractionState, look: FieldLook) -> Vec<Shape> {
        let mut value = value.to_owned();
        PaintCapture::shapes(|painter| {
            TextField::new(egui::Id::NULL, &mut value, kind, Sample::text("Search replays")).paint(
                painter,
                Self::RECT,
                state,
                &look,
            );
        })
    }
}

#[test]
fn invalid_draws_danger_stroke_and_reason() {
    let look = FieldLook { selection: None, invalid: Some(Sample::text("A scene with this name already exists")) };
    let shapes = FieldPaint::shapes("Sunset pan", TextFieldKind::Plain, InteractionState::Rest, look);
    let rects = PaintCapture::rects(&shapes);
    assert!(rects.iter().any(|r| r.rect == FieldPaint::RECT && r.stroke.color == color::Danger::VALUE));
    let reason = shapes.iter().find_map(|s| match s {
        Shape::Text(t) if t.galley.text().starts_with("A scene") => {
            Some((t.pos.y, t.galley.job.sections[0].format.color))
        }
        _ => None,
    });
    let (top, colour) = reason.unwrap_or((0.0, egui::Color32::TRANSPARENT));
    assert_eq!(colour, color::Danger::VALUE);
    assert!(top >= FieldPaint::RECT.bottom() + 8.0 - 0.5, "the reason is {top}, not below the field");
}

#[test]
fn typing_look_has_selection_and_caret() {
    let look = FieldLook { selection: Some(7..10), invalid: None };
    let shapes = FieldPaint::shapes("Sunset pan", TextFieldKind::Plain, InteractionState::Focused, look);
    let rects = PaintCapture::rects(&shapes);
    assert!(rects.iter().any(|r| r.fill == color::Selection::VALUE), "no selection fill");
    assert!(rects.iter().any(|r| r.fill == color::Caret::VALUE && r.rect.width() == 1.0), "no caret");
}

#[test]
fn search_has_icon_and_clear_only_with_text() {
    let with_text = FieldPaint::shapes("nether", TextFieldKind::Search, InteractionState::Rest, FieldLook::default());
    let texts = ShapeText::all(&with_text);
    assert!(texts.contains(&"\u{E721}".to_owned()) && texts.contains(&"\u{E8BB}".to_owned()));
    let empty = FieldPaint::shapes("", TextFieldKind::Search, InteractionState::Rest, FieldLook::default());
    let texts = ShapeText::all(&empty);
    assert!(!texts.contains(&"\u{E8BB}".to_owned()));
    assert!(texts.contains(&"Search replays".to_owned()), "no placeholder");
}

#[test]
fn hover_steps_the_fill() {
    let shapes = FieldPaint::shapes("Sunset pan", TextFieldKind::Plain, InteractionState::Hover, FieldLook::default());
    assert!(PaintCapture::rects(&shapes).iter().any(|r| r.rect == FieldPaint::RECT && r.fill == color::BgHover::VALUE));
}

#[test]
fn typing_edits_the_value() {
    let mut harness = ClickProbe::harness_with(String::new(), |ui, value| {
        TextField::new(egui::Id::unique(1), value, TextFieldKind::Plain, Sample::text("Scene name")).show(ui, 280.0)
    });
    ClickProbe::click(&mut harness);
    harness.event(Event::Text("Arrival".into()));
    harness.run();
    assert_eq!(harness.state().value, "Arrival");
}

#[test]
fn escape_restores_the_start_value() {
    let mut harness = ClickProbe::harness_with("Arrival".to_owned(), |ui, value| {
        TextField::new(egui::Id::unique(1), value, TextFieldKind::Plain, Sample::text("Scene name")).show(ui, 280.0)
    });
    ClickProbe::click(&mut harness);
    harness.event(Event::Text(" two".into()));
    harness.run();
    harness.key_press(Key::Escape);
    harness.run();
    assert_eq!(harness.state().value, "Arrival");
}

#[test]
fn search_clear_empties_the_text() {
    let mut harness = ClickProbe::harness_with("nether".to_owned(), |ui, value| {
        TextField::new(egui::Id::unique(1), value, TextFieldKind::Search, Sample::text("Scene name")).show(ui, 280.0)
    });
    // the clear button sits at the right end, inside `--spacing-s`
    let rect = harness.state().rect;
    let clear = pos2(rect.right() - 8.0 - 10.0, rect.top() + 17.0);
    ClickProbe::click_at(&mut harness, clear);
    assert_eq!(harness.state().value, "");
}

#[test]
fn two_fields_keep_separate_text() {
    let mut harness = ClickProbe::harness_with((String::new(), String::new()), |ui, (first, second)| {
        let a = TextField::new(egui::Id::unique(1), first, TextFieldKind::Plain, Sample::text("Scene name"))
            .show(ui, 280.0);
        TextField::new(egui::Id::unique(2), second, TextFieldKind::Plain, Sample::text("Scene name")).show(ui, 280.0);
        a
    });
    ClickProbe::click(&mut harness);
    harness.event(Event::Text("one".into()));
    harness.run();
    assert_eq!(harness.state().value, ("one".to_owned(), String::new()));
}
