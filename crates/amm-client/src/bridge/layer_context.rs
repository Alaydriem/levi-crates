use amm_abi::types::{AmmCursorShape, AmmFrameOutput, AmmMeshView};
use amm_core::services::egui_bridge::{MeshFlattener, RawInputBuilder};
use amm_core::structs::frame::LayerKind;
use amm_core::structs::mesh::MeshBuffers;
use egui::{Context, RawInput};
use studio_ui::theme::{FontSource, StudioTheme};

use crate::link::{FrameHandler, FrameInputView, FrameReply};

/// The egui runtime of one layer of a mod: its context, raw-input builder and the mesh buffers the host reads.
/// The buffers keep their capacity across frames. Managed texture ids leave with the layer bit of
/// `MeshFlattener`, so the ids of the Hud and Overlay contexts never meet in the host.
pub struct LayerContext {
    layer: LayerKind,
    ctx: Context,
    builder: RawInputBuilder,
    mesh: MeshBuffers,
}

impl LayerContext {
    const FONTS: FontSource = FontSource::Embedded;

    pub fn new(layer: LayerKind) -> Self {
        Self { layer, ctx: Self::context(), builder: RawInputBuilder::new(), mesh: MeshBuffers::new() }
    }

    /// Runs one pass of `handler` for this layer and fills `out`. `out.mesh` points at this layer's own buffers,
    /// in physical pixels, and stays valid until the next `run` of this layer.
    pub fn run<H: FrameHandler>(&mut self, handler: &mut H, input: &FrameInputView, out: &mut AmmFrameOutput) {
        let mut raw = RawInput::default();
        self.builder.build(input.data(), &mut raw);
        let mut reply = None;
        let mut output = self.ctx.run_pass(raw, |ctx| reply = Some(handler.frame(ctx, input)));
        let ppp = output.pixels_per_point;
        let primitives = self.ctx.tessellate(std::mem::take(&mut output.shapes), ppp);
        MeshFlattener::flatten_layer(&primitives, &output.textures_delta, ppp, self.layer, &mut self.mesh);
        output.textures_delta.clear();
        self.fill(out, reply, ppp);
    }

    /// Replaces the context, so the next pass sends the font atlas again in full under the same ids. Used when the
    /// host lost its textures and after a panic left the context mid-pass.
    pub fn reset(&mut self) {
        self.ctx = Self::context();
    }

    fn context() -> Context {
        let ctx = Context::default();
        StudioTheme::install(&ctx, Self::FONTS);
        ctx
    }

    fn fill(&self, out: &mut AmmFrameOutput, reply: Option<FrameReply>, ppp: f32) {
        out.mesh = AmmMeshView::from(&self.mesh);
        out.wants_pointer = u8::from(self.ctx.egui_wants_pointer_input());
        out.wants_keyboard = u8::from(self.ctx.egui_wants_keyboard_input());
        let (cursor, rect) = match reply {
            Some(reply) => (AmmCursorShape::from(reply.cursor), reply.layer_rect),
            None => (AmmCursorShape::Default, egui::Rect::NOTHING),
        };
        out.cursor = cursor as u8;
        // An empty or inverted rect, such as Rect::NOTHING, reaches the host as an empty rect at the origin
        out.layer_rect = if rect.is_positive() {
            [rect.min.x * ppp, rect.min.y * ppp, rect.max.x * ppp, rect.max.y * ppp]
        } else {
            [0.0; 4]
        };
    }
}
