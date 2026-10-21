use egui::Context;

use super::{FrameInputView, FrameReply};

/// The mod's egui pass. The bridge calls it on the render thread inside the host's frame, once per layer the
/// manager gives the mod in that frame. Each layer has its own egui context.
pub trait FrameHandler {
    /// Paints one pass of the layer `input.layer()` into that layer's `ctx`. `input` is the routed input of
    /// this layer for this frame.
    fn frame(&mut self, ctx: &Context, input: &FrameInputView) -> FrameReply;

    /// The host lost every texture. The bridge has already reset egui's own textures of every layer; a mod that
    /// uploads its own textures sends them again on its next frame.
    fn textures_lost(&mut self) {}
}
