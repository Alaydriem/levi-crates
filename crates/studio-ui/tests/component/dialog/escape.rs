use egui::{Key, Ui};
use studio_ui::component::dialog::{
    ConfirmAction, ConfirmDialog, ConfirmDialogView, CoverageFilter, CoveragePanel, CoveragePanelAction,
    CoveragePanelView, ExportDialog, ExportDialogAction, ExportDialogView, ExportFormat, ExportFrameRate,
    ExportResolution, JobStatus, RenameAction, RenameDialog, RenderQueue, RenderQueueAction,
};

use super::queue;
use crate::glyph::words;
use crate::support::UiProbe;

// shows a dialog for a frame, then a frame with Escape pressed; returns the second frame's action
fn escape<T>(mut show: impl FnMut(&mut Ui) -> Option<T>) -> Option<T> {
    let mut probe = UiProbe::sized(1280.0, 900.0);
    probe.frame(Vec::new(), &mut show);
    probe.frame(vec![UiProbe::key(Key::Escape)], &mut show).0
}

#[test]
fn escape_cancels_each_dialog() {
    let export = ExportDialogView {
        subtitle: words("Arrival · output 6.2 s"),
        format: ExportFormat::H265,
        resolution: ExportResolution::Uhd,
        frame_rate: ExportFrameRate::Sixty,
        quality: 0.75,
        quality_label: words("High"),
        motion_blur: true,
        motion_blur_note: words("Shutter 180°"),
        sidecar: true,
        sidecar_note: words(".dejavu.json: scene id, wall-clock span"),
        folder: words("C:\\Videos\\DejaVu\\"),
        file: words("nether-trip_arrival_4k60.mp4"),
        note: words("Not rendered yet."),
    };
    assert_eq!(escape(|ui| ExportDialog::new(&export).show(ui.ctx())), Some(ExportDialogAction::Cancel));
    let jobs = queue(JobStatus::Queued, 0.0);
    assert_eq!(escape(|ui| RenderQueue::new(&jobs).show(ui.ctx())), Some(RenderQueueAction::Close));
    let coverage = CoveragePanelView { subtitle: words("Coverage"), filter: CoverageFilter::All, groups: Vec::new() };
    let mut search = String::new();
    assert_eq!(
        escape(|ui| CoveragePanel::new(&coverage, &mut search).show(ui.ctx())),
        Some(CoveragePanelAction::Close)
    );
    let confirm = ConfirmDialogView {
        title: words("Delete “Portal run”?"),
        subtitle: words("Undo can bring it back"),
        body: words("The scene and its 11 keys go away."),
        confirm: words("Delete scene"),
    };
    assert_eq!(escape(|ui| ConfirmDialog::new(&confirm).show(ui.ctx())), Some(ConfirmAction::Cancel));
    let title = words("Rename scene");
    let mut value = "Portal run".to_owned();
    assert_eq!(escape(|ui| RenameDialog::new(&title, &mut value).show(ui.ctx())), Some(RenameAction::Cancel));
}
