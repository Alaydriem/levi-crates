use levi_ui::icon::Icon;
use levi_ui::tokens::SpacingToken;
use studio_ui::component::bar::{StudioMode, TopBar, TopBarAction, TopBarControl, TopBarView};
use studio_ui::tokens::selector::SceneColor;
use studio_ui::tokens::spacing;

use super::arrival;
use crate::glyph::words;
use crate::support::UiProbe;

fn view(mode: StudioMode, can_undo: bool) -> TopBarView {
    TopBarView { title: words("Nether trip"), mode, can_undo, can_redo: false, history_open: false, scene: arrival() }
}

#[test]
fn top_bar_fills_its_token_height() {
    let mut probe = UiProbe::new(1920.0);
    let view = view(StudioMode::Edit, true);
    let (height, _) = probe.frame(Vec::new(), |ui| {
        TopBar::new(&view).show(ui);
        ui.min_rect().height()
    });
    assert_eq!(height, <spacing::TopBar as SpacingToken>::VALUE);
}

#[test]
fn mode_click_sends_set_mode() {
    let mut probe = UiProbe::new(1920.0);
    let view = view(StudioMode::Edit, true);
    let shapes = probe.shapes(|ui| TopBar::new(&view).show(ui));
    let preview =
        UiProbe::text_rect(&shapes, "Preview").unwrap_or_else(|| panic!("no Preview: {:?}", UiProbe::texts(&shapes)));
    let actions = probe.click(preview.center(), |ui| TopBar::new(&view).show(ui));
    assert_eq!(actions, vec![TopBarAction::SetMode(StudioMode::Preview)]);
}

#[test]
fn long_scene_name_stays_left_of_the_mode_switch() {
    let mut probe = UiProbe::new(1280.0);
    let mut view = view(StudioMode::Preview, true);
    view.scene.scene = Some((words("Portal run through the nether fortress and back"), SceneColor::Lime));
    let shapes = probe.shapes(|ui| TopBar::new(&view).show(ui));
    let edit = UiProbe::text_rect(&shapes, "Edit").unwrap_or_else(|| panic!("no Edit: {:?}", UiProbe::texts(&shapes)));
    let place = UiProbe::text_rect(&shapes, "1 of 5").map_or(f32::NEG_INFINITY, |rect| rect.right());
    let name = shapes
        .iter()
        .filter_map(|shape| match shape {
            egui::Shape::Text(text) if text.galley.text().starts_with("Portal") => {
                Some(text.galley.rect.translate(text.pos.to_vec2()).right())
            }
            _ => None,
        })
        .fold(f32::NEG_INFINITY, f32::max);
    assert!(name.max(place) < edit.left(), "the chip text ends at {} past Edit at {}", name.max(place), edit.left());
    let actions = probe.click(edit.center(), |ui| TopBar::new(&view).show(ui));
    assert_eq!(actions, vec![TopBarAction::SetMode(StudioMode::Edit)]);
}

#[test]
fn undo_is_disabled_without_history() {
    let undo = Icon::Undo.shape();
    let glyph = match undo {
        levi_ui::icon::IconShape::Font(code) => code.to_string(),
        levi_ui::icon::IconShape::Drawn(_) => panic!("undo is a font icon"),
    };
    for (can_undo, expected) in [(true, vec![TopBarAction::Undo]), (false, Vec::new())] {
        let mut probe = UiProbe::new(1920.0);
        let view = view(StudioMode::Edit, can_undo);
        let shapes = probe.shapes(|ui| TopBar::new(&view).show(ui));
        let at = UiProbe::text_rect(&shapes, &glyph).unwrap_or_else(|| panic!("no undo icon")).center();
        let actions = probe.click(at, |ui| TopBar::new(&view).show(ui));
        assert_eq!(actions, expected, "can_undo {can_undo}");
    }
}

#[test]
fn coverage_queue_and_export_send_their_actions() {
    for (label, expected) in [
        ("Coverage", TopBarAction::OpenCoverage),
        ("Queue", TopBarAction::OpenQueue),
        ("Export", TopBarAction::OpenExport),
    ] {
        let mut probe = UiProbe::new(1920.0);
        let view = view(StudioMode::Edit, true);
        let shapes = probe.shapes(|ui| TopBar::new(&view).show(ui));
        let at = UiProbe::text_rect(&shapes, label).unwrap_or_else(|| panic!("no {label}")).center();
        assert_eq!(probe.click(at, |ui| TopBar::new(&view).show(ui)), vec![expected]);
    }
}

#[test]
fn the_scene_chip_opens_the_scene_switcher() {
    let mut probe = UiProbe::new(1920.0);
    let view = view(StudioMode::Edit, true);
    let shapes = probe.shapes(|ui| TopBar::new(&view).show(ui));
    let at = UiProbe::text_rect(&shapes, "Arrival").unwrap_or_else(|| panic!("no chip")).center();
    assert_eq!(probe.click(at, |ui| TopBar::new(&view).show(ui)), vec![TopBarAction::OpenSceneSwitcher]);
}

#[test]
fn a_long_title_keeps_the_controls_in_the_bar() {
    let mut probe = UiProbe::new(1280.0);
    let mut view = view(StudioMode::Edit, true);
    view.title = words(&"A very long replay title ".repeat(12));
    let shapes = probe.shapes(|ui| TopBar::new(&view).show(ui));
    let export = UiProbe::text_rect(&shapes, "Export").unwrap_or_else(|| panic!("no Export"));
    assert!(export.right() <= 1280.0, "{export:?}");
    let preview = UiProbe::text_rect(&shapes, "Preview").unwrap_or_else(|| panic!("no Preview"));
    for (text, rect) in shapes.iter().filter_map(|shape| match shape {
        egui::Shape::Text(t) if t.galley.text().starts_with("A very") => {
            Some((t.galley.text().to_owned(), t.galley.rect.translate(t.pos.to_vec2())))
        }
        _ => None,
    }) {
        assert!(rect.right() < preview.left(), "{text} runs into the mode switch");
    }
}

fn placed(mode: StudioMode) -> (egui::Rect, Vec<(TopBarControl, egui::Rect)>) {
    let mut probe = UiProbe::new(1920.0);
    let view = TopBarView { can_redo: true, ..view(mode, true) };
    let show = |ui: &mut egui::Ui| {
        let (_, places) = TopBar::new(&view).show_placed(ui);
        (ui.min_rect(), places)
    };
    probe.frame(Vec::new(), show);
    probe.frame(Vec::new(), show).0
}

#[test]
fn top_bar_places_all_11_controls_inside_the_bar() {
    let (bar, places) = placed(StudioMode::Edit);
    assert_eq!(places.len(), 11, "{places:?}");
    for (control, rect) in &places {
        assert!(bar.contains_rect(*rect), "{control:?} {rect:?} outside {bar:?}");
    }
}

#[test]
fn a_click_at_the_centre_of_each_placed_control_gives_its_action() {
    let cases = [
        (TopBarControl::SceneChip, StudioMode::Edit, TopBarAction::OpenSceneSwitcher),
        (TopBarControl::Edit, StudioMode::Preview, TopBarAction::SetMode(StudioMode::Edit)),
        (TopBarControl::Preview, StudioMode::Edit, TopBarAction::SetMode(StudioMode::Preview)),
        (TopBarControl::Undo, StudioMode::Edit, TopBarAction::Undo),
        (TopBarControl::Redo, StudioMode::Edit, TopBarAction::Redo),
        (TopBarControl::UndoHistory, StudioMode::Edit, TopBarAction::OpenUndoHistory),
        (TopBarControl::Settings, StudioMode::Edit, TopBarAction::OpenSettings),
        (TopBarControl::Close, StudioMode::Edit, TopBarAction::CloseReplay),
        (TopBarControl::Coverage, StudioMode::Edit, TopBarAction::OpenCoverage),
        (TopBarControl::Queue, StudioMode::Edit, TopBarAction::OpenQueue),
        (TopBarControl::Export, StudioMode::Edit, TopBarAction::OpenExport),
    ];
    for (control, mode, expected) in cases {
        let (_, places) = placed(mode);
        let rect = places.iter().find(|(c, _)| *c == control).unwrap_or_else(|| panic!("no {control:?}")).1;
        let mut probe = UiProbe::new(1920.0);
        let view = TopBarView { can_redo: true, ..view(mode, true) };
        let show = |ui: &mut egui::Ui| TopBar::new(&view).show(ui);
        probe.shapes(show);
        assert_eq!(probe.click(rect.center(), show), vec![expected], "{control:?}");
    }
}
