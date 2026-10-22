use egui::Color32;
use levi_ui::tokens::ColorToken;
use studio_ui::tokens::color;
use studio_ui::tokens::selector::{AxisColor, ModeColor, SceneColor, StatusColor, TrackColor};

#[test]
fn scene_lime_is_b5e853() {
    assert_eq!(SceneColor::Lime.color(), Color32::from_rgb(0xB5, 0xE8, 0x53));
}

#[test]
fn selectors_list_every_variant_in_paper_order() {
    assert_eq!(SceneColor::ALL.len(), 6);
    assert_eq!(SceneColor::ALL[5], SceneColor::Sand);
    assert_eq!(TrackColor::ALL.len(), 5);
    assert_eq!(StatusColor::ALL.len(), 5);
    assert_eq!(ModeColor::ALL, [ModeColor::Edit, ModeColor::Preview, ModeColor::ReadOnly, ModeColor::Busy]);
    assert_eq!(AxisColor::ALL.len(), 3);
}

#[test]
fn studio_aliases_resolve_to_their_source() {
    assert_eq!(StatusColor::Works.color(), <levi_ui::tokens::color::Success as ColorToken>::VALUE);
    assert_eq!(ModeColor::Preview.color(), TrackColor::Camera.color());
    assert_eq!(<color::SceneCamera as ColorToken>::VALUE, <color::TrackCamera as ColorToken>::VALUE);
}
