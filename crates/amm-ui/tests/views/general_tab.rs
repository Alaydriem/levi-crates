use amm_core::structs::frame::FrameInputData;
use amm_core::structs::mesh::MeshBuffers;
use amm_ui::model::{PanelAction, PanelModel, PanelTab};
use amm_ui::runtime::PanelRuntime;
use amm_ui::views::PanelLayout;
use egui::{Pos2, Rect, vec2};
use studio_ui::theme::{FontSource, Metrics};

use crate::support::{DEMO, FourMods};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const DEMO_HELP: &str = "Also registers alaydriem-demo-b, which claims the camera exclusively, to show a suspension, a denial and a key clash";

fn input(time_secs: f64) -> FrameInputData<'static> {
    FrameInputData {
        width: WIDTH,
        height: HEIGHT,
        pixels_per_point: 1.0,
        time_secs,
        events: &[],
        allowed_layers: FrameInputData::LAYER_OVERLAY,
        input_flags: FrameInputData::ALLOW_POINTER | FrameInputData::ALLOW_KEYBOARD | FrameInputData::ALLOW_NAV_KEYS,
    }
}

// The right edge of the panel's content: text drawn past it crosses the card edge.
fn content_right() -> f32 {
    let screen = Rect::from_min_size(Pos2::ZERO, vec2(WIDTH as f32, HEIGHT as f32));
    PanelLayout::panel_rect(screen).shrink(Metrics::GAP_L + 4.0).right()
}

// The right-most glyph vertex: glyphs sample the font atlas, fills sample the white pixel at uv (0, 0).
fn rightmost_glyph(out: &MeshBuffers) -> f32 {
    out.vertices.iter().filter(|vertex| vertex.uv != [0.0, 0.0]).map(|vertex| vertex.pos[0]).fold(0.0, f32::max)
}

#[test]
fn a_long_help_text_wraps_inside_the_card() {
    let long_help = format!("{DEMO_HELP}, and then some more words so it runs well past any card width at this size");
    let descriptor = DEMO.replace(DEMO_HELP, &long_help);
    let mut mods = FourMods::new(&descriptor);
    mods.manager.set_panel_open(true);
    let model = PanelModel::new(&mods.manager, "1.4.1", 0);
    let mut runtime = PanelRuntime::new(FontSource::BuiltIn);
    runtime.controller_mut().on_action(&PanelAction::Select(Some(mods.minimap)));
    runtime.controller_mut().on_action(&PanelAction::SelectTab(PanelTab::General));
    let mut out = MeshBuffers::new();

    for frame in 0..3 {
        runtime.run(&input(f64::from(frame) / 60.0), &model, &mut out);
    }

    let right = rightmost_glyph(&out);
    assert!(right <= content_right(), "a glyph ends at {right}, past the content edge {}", content_right());
}
