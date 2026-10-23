mod coverage_panel;
mod escape;
mod rename_dialog;
mod render_queue;

use studio_ui::component::dialog::{JobStatus, RenderJobRow, RenderQueueView};
use studio_ui::tokens::selector::SceneColor;

use crate::glyph::words;

// a queue of one job in `status` at `progress`
pub fn queue(status: JobStatus, progress: f32) -> RenderQueueView {
    RenderQueueView {
        subtitle: words("1 job · 1 to go"),
        paused: false,
        jobs: vec![RenderJobRow {
            name: words("Arrival"),
            meta: words("H.265 · 4K · 60 fps · 372 frames"),
            file: words("nether-trip_arrival_4k60.mp4"),
            scene: SceneColor::Lime,
            progress,
            status,
            detail: words("ETA 6 s"),
            note: words("Frame 106 / 372 · 45 fps"),
        }],
    }
}
