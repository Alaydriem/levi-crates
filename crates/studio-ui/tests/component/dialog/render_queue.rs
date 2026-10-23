use egui::Shape;
use levi_ui::tokens::{ColorToken, color};
use studio_ui::component::dialog::{JobStatus, RenderQueue, RenderQueueView};

use super::queue;
use crate::glyph::words;
use crate::support::UiProbe;

// the width of the accent progress fill
fn bar(progress: f32) -> f32 {
    let view = queue(JobStatus::Rendering, progress);
    let shapes = UiProbe::sized(1000.0, 800.0).shapes(|ui| RenderQueue::new(&view).show_card(ui));
    let accent = <color::Accent as ColorToken>::VALUE;
    shapes
        .iter()
        .filter_map(|shape| match shape {
            Shape::Rect(rect) if rect.fill == accent => Some(rect.rect.width()),
            _ => None,
        })
        .sum()
}

#[test]
fn progress_is_clamped() {
    let full = bar(1.0);
    assert!(full > 0.0);
    assert_eq!(bar(f32::NAN), 0.0);
    assert_eq!(bar(-0.5), 0.0);
    assert_eq!(bar(1.5), full);
}

#[test]
fn queue_with_no_jobs() {
    let view = RenderQueueView { subtitle: words("No jobs"), paused: false, jobs: Vec::new() };
    let texts = UiProbe::texts(&UiProbe::sized(1000.0, 800.0).shapes(|ui| RenderQueue::new(&view).show_card(ui)));
    assert!(texts.iter().any(|text| text == "No render jobs"), "{texts:?}");
}
