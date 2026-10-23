use levi_ui::icon::{Icon, IconShape};
use levi_ui::tokens::SpacingToken;
use studio_ui::component::bar::{Transport, TransportAction, TransportLayout, TransportView};
use studio_ui::tokens::spacing;

use crate::glyph::words;
use crate::support::UiProbe;

fn view(playing: bool) -> TransportView {
    TransportView {
        playing,
        timecode: words("00:00:32.95"),
        duration: None,
        speed: words("1x"),
        looping: false,
        snap: true,
        guides: false,
        zoom: 0.65,
    }
}

fn glyph(icon: Icon) -> String {
    match icon.shape() {
        IconShape::Font(code) => code.to_string(),
        IconShape::Drawn(_) => panic!("{icon:?} is drawn"),
    }
}

#[test]
fn transport_is_compact_below_the_small_breakpoint() {
    assert_eq!(TransportLayout::for_width(1279.0), TransportLayout::Compact);
    assert_eq!(TransportLayout::for_width(1280.0), TransportLayout::Full);
}

#[test]
fn transport_fills_its_token_height() {
    for layout in [TransportLayout::Full, TransportLayout::Compact] {
        let mut probe = UiProbe::new(1490.0);
        let view = view(false);
        let (height, _) = probe.frame(Vec::new(), |ui| {
            Transport::new(&view).layout(layout).show(ui);
            ui.min_rect().height()
        });
        assert_eq!(height, <spacing::Transport as SpacingToken>::VALUE, "{layout:?}");
    }
}

#[test]
fn play_click_sends_toggle_play() {
    for (playing, icon) in [(false, Icon::Play), (true, Icon::Pause)] {
        let mut probe = UiProbe::new(1490.0);
        let view = view(playing);
        let shapes = probe.shapes(|ui| Transport::new(&view).show(ui));
        let at = UiProbe::text_rect(&shapes, &glyph(icon)).unwrap_or_else(|| panic!("no {icon:?}")).center();
        assert_eq!(probe.click(at, |ui| Transport::new(&view).show(ui)), vec![TransportAction::TogglePlay]);
    }
}

#[test]
fn add_camera_key_is_disabled_while_playing() {
    for (playing, expected) in [(false, vec![TransportAction::AddCameraKey]), (true, Vec::new())] {
        let mut probe = UiProbe::new(1490.0);
        let view = view(playing);
        let shapes = probe.shapes(|ui| Transport::new(&view).show(ui));
        let at = UiProbe::text_rect(&shapes, "Add camera key").unwrap_or_else(|| panic!("no add key")).center();
        assert_eq!(probe.click(at, |ui| Transport::new(&view).show(ui)), expected, "playing {playing}");
    }
}

#[test]
fn transport_has_no_timelapse_control() {
    for layout in [TransportLayout::Full, TransportLayout::Compact] {
        let mut probe = UiProbe::new(1490.0);
        let view = view(false);
        let shapes = probe.shapes(|ui| Transport::new(&view).layout(layout).show(ui));
        let texts = UiProbe::texts(&shapes);
        assert!(texts.iter().all(|text| !text.to_lowercase().contains("timelapse")), "{texts:?}");
    }
}

#[test]
fn compact_transport_drops_the_add_key_and_fit_words() {
    let mut probe = UiProbe::new(1236.0);
    let view = view(false);
    let shapes = probe.shapes(|ui| Transport::new(&view).layout(TransportLayout::Compact).show(ui));
    let texts = UiProbe::texts(&shapes);
    assert!(!texts.iter().any(|text| text == "Add camera key" || text == "Fit"), "{texts:?}");
    assert!(texts.contains(&glyph(Icon::Fit)), "the compact Fit is an icon: {texts:?}");
}

#[test]
fn the_duration_shows_after_the_timecode_when_given() {
    let mut probe = UiProbe::new(1920.0);
    let mut view = view(false);
    view.duration = Some(words("02:14:08.40"));
    let shapes = probe.shapes(|ui| Transport::new(&view).show(ui));
    let time = UiProbe::text_rect(&shapes, "00:00:32.95").unwrap_or_else(|| panic!("no timecode"));
    let duration = UiProbe::text_rect(&shapes, "/ 02:14:08.40").unwrap_or_else(|| panic!("no duration"));
    assert!(duration.left() > time.right());
}

#[test]
fn narrow_compact_transport_keeps_its_groups_apart() {
    for width in [760.0, 900.0] {
        let mut probe = UiProbe::new(width);
        let view = view(false);
        let show = |ui: &mut egui::Ui| Transport::new(&view).layout(TransportLayout::Compact).show(ui);
        let shapes = probe.shapes(show);
        let guides = UiProbe::text_rect(&shapes, &glyph(Icon::Grid)).unwrap_or_else(|| panic!("no guides at {width}"));
        for icon in [Icon::ZoomOut, Icon::ZoomIn] {
            if let Some(rect) = UiProbe::text_rect(&shapes, &glyph(icon)) {
                assert!(rect.left() > guides.right(), "{icon:?} {rect:?} over the guides {guides:?} at {width}");
            }
        }
        assert_eq!(probe.click(guides.center(), show), vec![TransportAction::ToggleGuides], "at {width}");
    }
}
