use egui::{Color32, Pos2, Rect, Shape, vec2};
use egui_kittest::Harness;
use levi_i18n::text::UserText;
use levi_ui::font::FontSource;
use levi_ui::theme::Theme;
use levi_ui::tokens::{ColorToken, color};
use levi_ui::widget::{Avatar, AvatarSize, Chip, ChipKind, ChipTone, ListRow, Toggle};

use crate::support::{PaintCapture, Sample};

// what one list row is built from
#[derive(Clone)]
struct RowSample {
    name: &'static str,
    chips: &'static [&'static str],
    off: bool,
}

// what the row reported over a run
#[derive(Debug)]
struct RowLog {
    on: bool,
    selects: usize,
    toggles: usize,
    rect: Rect,
}

// shows one list row in a `width` wide screen
struct RowProbe;

impl RowProbe {
    fn harness(width: f32, sample: RowSample) -> Harness<'static, RowLog> {
        let mut harness = Harness::builder()
            .with_size(vec2(width, 120.0))
            .allow_missing_glyphs()
            .with_context_setup(|ctx| Theme::install(ctx, FontSource::BuiltIn))
            .build_ui_state(
                move |ui, log: &mut RowLog| {
                    let avatar = Avatar::new(&UserText::from_model(sample.name.to_owned()), AvatarSize::Regular);
                    let chips = sample
                        .chips
                        .iter()
                        .map(|chip| Chip::new(ChipKind::Display(ChipTone::Neutral), Sample::text(chip)))
                        .collect();
                    let response = ListRow::new(avatar, Sample::text(sample.name))
                        .version(Sample::text("v2.11.0"))
                        .chips(chips)
                        .off(sample.off)
                        .toggle(&mut log.on)
                        .show(ui);
                    log.selects += usize::from(response.row.clicked());
                    log.toggles += usize::from(response.toggled);
                    log.rect = response.row.rect;
                },
                RowLog { on: true, selects: 0, toggles: 0, rect: Rect::NOTHING },
            );
        harness.run();
        harness
    }

    fn plain(name: &'static str) -> RowSample {
        RowSample { name, chips: &["Input", "HUD"], off: false }
    }

    fn shapes(harness: &Harness<'static, RowLog>) -> Vec<Shape> {
        harness.output().shapes.iter().map(|clipped| clipped.shape.clone()).collect()
    }

    // (rect, colour, cut) of every text shape, by text
    fn texts(shapes: &[Shape]) -> Vec<(String, Rect, Color32, bool)> {
        shapes
            .iter()
            .filter_map(|shape| match shape {
                Shape::Text(text) => Some((
                    text.galley.text().to_owned(),
                    text.galley.rect.translate(text.pos.to_vec2()),
                    text.galley.job.sections[0].format.color,
                    text.galley.elided,
                )),
                _ => None,
            })
            .collect()
    }

    fn click(harness: &mut Harness<'static, RowLog>, at: Pos2) {
        harness.hover_at(at);
        harness.run();
        harness.drag_at(at);
        harness.step();
        harness.drop_at(at);
        harness.run();
    }

    // the painted track of the toggle: the only rect the size of `Toggle::TRACK`
    fn track(harness: &Harness<'static, RowLog>) -> Rect {
        let track = PaintCapture::rects(&Self::shapes(harness))
            .into_iter()
            .find(|r| r.rect.size() == Toggle::TRACK)
            .map(|r| r.rect);
        let Some(track) = track else { panic!("no toggle track") };
        track
    }
}

#[test]
fn toggle_click_does_not_select() {
    let mut harness = RowProbe::harness(460.0, RowProbe::plain("Minimap"));
    let track = RowProbe::track(&harness);
    RowProbe::click(&mut harness, track.center());
    let log = harness.state();
    assert!(!log.on, "the toggle did not switch");
    assert_eq!((log.toggles, log.selects), (1, 0), "{log:?}");
}

#[test]
fn row_click_selects() {
    let mut harness = RowProbe::harness(460.0, RowProbe::plain("Minimap"));
    let rect = harness.state().rect;
    RowProbe::click(&mut harness, rect.left_center() + vec2(30.0, 0.0));
    let log = harness.state();
    assert!(log.on, "a row click switched the toggle");
    assert_eq!((log.toggles, log.selects), (0, 1), "{log:?}");
}

#[test]
fn overflow_chips_collapse_to_more() {
    let chips: &[&str] = &["Camera", "Input", "HUD", "Overlay", "World", "Audio"];
    let harness = RowProbe::harness(330.0, RowSample { name: "Freecam", chips, off: false });
    let track = RowProbe::track(&harness);
    let texts = RowProbe::texts(&RowProbe::shapes(&harness));
    let shown: Vec<_> = texts.iter().filter(|(text, ..)| chips.contains(&text.as_str())).collect();
    let more: Vec<_> = texts.iter().filter(|(text, ..)| text.starts_with('+')).collect();
    assert_eq!(more.len(), 1, "one +N chip: {texts:?}");
    let count: usize = more[0].0[1..].parse().expect("+N has a number");
    assert!(!shown.is_empty() && count > 0, "{texts:?}");
    assert_eq!(shown.len() + count, chips.len(), "{texts:?}");
    for (text, rect, ..) in shown.iter().chain(more.iter()) {
        assert!(rect.right() <= track.left(), "{text:?} runs under the toggle");
    }
}

#[test]
fn long_name_is_cut_version_kept() {
    let name = "World Edit Selection Tools and Brushes Extended For Every Builder";
    let harness = RowProbe::harness(460.0, RowProbe::plain(name));
    let track = RowProbe::track(&harness);
    let texts = RowProbe::texts(&RowProbe::shapes(&harness));
    let Some((_, name_rect, _, cut)) = texts.iter().find(|(text, ..)| text == name) else { panic!("no name") };
    assert!(cut, "the name is not cut");
    let Some((_, version, _, version_cut)) = texts.iter().find(|(text, ..)| text == "v2.11.0") else {
        panic!("no version")
    };
    assert!(!version_cut, "the version is cut");
    assert!(name_rect.right() <= version.left(), "the name runs over the version");
    assert!(version.right() <= track.left(), "the version runs under the toggle");
}

#[test]
fn off_dims_name_letter_and_chips() {
    let harness = RowProbe::harness(460.0, RowSample { name: "World Edit", chips: &["World"], off: true });
    let texts = RowProbe::texts(&RowProbe::shapes(&harness));
    let colour = |wanted: &str| match texts.iter().find(|(text, ..)| text == wanted) {
        Some((_, _, colour, _)) => *colour,
        None => panic!("no {wanted:?}: {texts:?}"),
    };
    assert_eq!(colour("World Edit"), color::TextDim::VALUE, "name");
    assert_eq!(colour("W"), color::TextFaint::VALUE, "avatar letter");
    assert_eq!(colour("World"), color::TextFaint::VALUE, "chip");
    assert_eq!(colour("v2.11.0"), color::TextFaint::VALUE, "version");
}
