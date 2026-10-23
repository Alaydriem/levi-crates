mod page_snapshot;

use std::path::Path;

use page_snapshot::PageSnapshot;
use studio_ui::gallery::GalleryPage;

#[test]
fn every_gallery_page_has_a_snapshot() {
    let folder = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("snapshots");
    let missing: Vec<&str> = GalleryPage::ALL
        .iter()
        .map(|page| page.file_name())
        .filter(|name| !folder.join(format!("{name}.png")).is_file())
        .collect();
    assert!(missing.is_empty(), "no snapshot for {missing:?}: run `mise run studio-ui:snapshots`");
}

#[test]
fn snapshot_glyphs() {
    PageSnapshot::check(GalleryPage::Glyphs);
}

#[test]
fn snapshot_status_badge() {
    PageSnapshot::check(GalleryPage::StatusBadge);
}

#[test]
fn snapshot_curve_plot() {
    PageSnapshot::check(GalleryPage::CurvePlot);
}

#[test]
fn snapshot_top_bar() {
    PageSnapshot::check(GalleryPage::TopBar);
}

#[test]
fn snapshot_transport() {
    PageSnapshot::check(GalleryPage::Transport);
}

#[test]
fn snapshot_status_bar() {
    PageSnapshot::check(GalleryPage::StatusBar);
}

#[test]
fn snapshot_scene_switcher_chip() {
    PageSnapshot::check(GalleryPage::SceneSwitcherChip);
}

#[test]
fn snapshot_viewer() {
    PageSnapshot::check(GalleryPage::Viewer);
}

#[test]
fn snapshot_viewer_chips() {
    PageSnapshot::check(GalleryPage::ViewerChips);
}

#[test]
fn snapshot_path_map() {
    PageSnapshot::check(GalleryPage::PathMap);
}

#[test]
fn snapshot_timeline() {
    PageSnapshot::check(GalleryPage::Timeline);
}

#[test]
fn snapshot_ruler() {
    PageSnapshot::check(GalleryPage::Ruler);
}

#[test]
fn snapshot_scene_bands() {
    PageSnapshot::check(GalleryPage::SceneBands);
}

#[test]
fn snapshot_realm_row() {
    PageSnapshot::check(GalleryPage::RealmRow);
}

#[test]
fn snapshot_track_headers() {
    PageSnapshot::check(GalleryPage::TrackHeaders);
}

#[test]
fn snapshot_lanes() {
    PageSnapshot::check(GalleryPage::Lanes);
}

#[test]
fn snapshot_box_select() {
    PageSnapshot::check(GalleryPage::BoxSelect);
}

#[test]
fn snapshot_inspector_tabs() {
    PageSnapshot::check(GalleryPage::InspectorTabs);
}

#[test]
fn snapshot_inspector_locked() {
    PageSnapshot::check(GalleryPage::InspectorLocked);
}

#[test]
fn snapshot_undo_history() {
    PageSnapshot::check(GalleryPage::UndoHistory);
}

#[test]
fn snapshot_easing_popover() {
    PageSnapshot::check(GalleryPage::EasingPopover);
}

#[test]
fn snapshot_export_dialog() {
    PageSnapshot::check(GalleryPage::ExportDialog);
}

#[test]
fn snapshot_render_queue() {
    PageSnapshot::check(GalleryPage::RenderQueue);
}

#[test]
fn snapshot_coverage_panel() {
    PageSnapshot::check(GalleryPage::CoveragePanel);
}

#[test]
fn snapshot_confirm_dialog() {
    PageSnapshot::check(GalleryPage::ConfirmDialog);
}

#[test]
fn snapshot_rename_dialog() {
    PageSnapshot::check(GalleryPage::RenameDialog);
}

#[test]
fn snapshot_capture_rail() {
    PageSnapshot::check(GalleryPage::CaptureRail);
}
