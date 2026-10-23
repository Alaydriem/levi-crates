use studio_ui::component::timeline::{Ruler, TimeAxis, TrackHeader};

use crate::glyph::words;
use crate::support::UiProbe;

// the painted time labels of a ruler 1490 wide
fn label_rects(length: f64, seconds_per_point: f64) -> Vec<egui::Rect> {
    let axis = TimeAxis { start: 0.0, seconds_per_point, left: TrackHeader::WIDTH };
    let title = words("Tracks");
    let shapes = UiProbe::new(1490.0).shapes(|ui| Ruler::new(&axis, length).title(&title).show(ui));
    let mut rects: Vec<egui::Rect> = shapes
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Text(text) if text.galley.text() != "Tracks" => {
                Some(text.galley.rect.translate(text.pos.to_vec2()))
            }
            _ => None,
        })
        .collect();
    rects.sort_by(|a, b| a.left().total_cmp(&b.left()));
    rects
}

#[test]
fn ruler_labels_never_overlap() {
    for length in [1.0, 600.0, 10_800.0] {
        // from one frame (1/20 s) across the lane width to the whole replay in 1280 px
        for seconds_per_point in [0.05 / 1268.0, 0.002, 0.01, 0.1, 1.0, length / 1280.0] {
            let rects = label_rects(length, seconds_per_point);
            assert!(!rects.is_empty(), "no labels for {length} s at {seconds_per_point}");
            for pair in rects.windows(2) {
                assert!(
                    pair[0].right() <= pair[1].left(),
                    "labels overlap for {length} s at {seconds_per_point}: {pair:?}"
                );
            }
        }
    }
}
