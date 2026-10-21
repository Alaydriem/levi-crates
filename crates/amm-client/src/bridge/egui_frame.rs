use amm_abi::types::{AmmFrameInput, AmmFrameOutput, AmmStatus};
use amm_core::structs::frame::{FrameInputData, LayerKind, UiEvent};

use super::LayerContext;
use crate::link::{FrameHandler, FrameInputView};

/// A mod's egui runtime: the handler and one `LayerContext` per layer kind. A layer's context is made the first
/// time the host calls the mod for that layer, so a mod has at most `LayerKind::COUNT` contexts.
pub struct EguiFrame<H: FrameHandler> {
    handler: H,
    layers: [Option<LayerContext>; LayerKind::COUNT],
    running: Option<LayerKind>,
    panic_logged: bool,
}

impl<H: FrameHandler> EguiFrame<H> {
    pub fn new(handler: H) -> Self {
        Self { handler, layers: [const { None }; LayerKind::COUNT], running: None, panic_logged: false }
    }

    /// Runs one pass of the handler for the layer named by `input.allowed_layers` and fills `out`. `out.mesh`
    /// points at that layer's own buffers, in physical pixels, and stays valid until the next `run` of the same
    /// layer.
    ///
    /// # Safety
    ///
    /// `input.events` is null, or points at `input.event_count` valid events that live for the call.
    pub unsafe fn run(&mut self, input: &AmmFrameInput, out: &mut AmmFrameOutput) -> AmmStatus {
        // SAFETY: the caller guarantees the events pointer and count
        let data = unsafe { Self::input_data(input) };
        let layer = LayerKind::from_bits(data.allowed_layers);
        self.running = Some(layer);
        let context = self.layers[layer.slot()].get_or_insert_with(|| LayerContext::new(layer));
        context.run(&mut self.handler, &FrameInputView::new(data, layer), out);
        self.running = None;
        AmmStatus::Ok
    }

    /// The host dropped every texture: each layer's context is replaced, so each sends its font atlas again, then
    /// the handler is told.
    pub fn textures_lost(&mut self) {
        for context in self.layers.iter_mut().flatten() {
            context.reset();
        }
        self.handler.textures_lost();
    }

    /// Called after the handler panicked. The context of the layer in flight may be left mid-pass, so it is
    /// replaced. When no layer is known to be in flight, every existing context is replaced. Returns true the
    /// first time, so the panic is logged once.
    pub(crate) fn recover(&mut self) -> bool {
        match self.running.take() {
            Some(layer) => {
                if let Some(context) = self.layers[layer.slot()].as_mut() {
                    context.reset();
                }
            }
            None => {
                for context in self.layers.iter_mut().flatten() {
                    context.reset();
                }
            }
        }
        !std::mem::replace(&mut self.panic_logged, true)
    }

    unsafe fn input_data(input: &AmmFrameInput) -> FrameInputData<'_> {
        let events: &[UiEvent] = if input.events.is_null() || input.event_count == 0 {
            &[]
        } else {
            // SAFETY: the caller guarantees event_count valid events for the call; UiEvent is #[repr(C)] with
            // the same fields as AmmUiEvent, which the amm-abi layout tests check
            unsafe { std::slice::from_raw_parts(input.events.cast::<UiEvent>(), input.event_count) }
        };
        FrameInputData {
            width: input.width,
            height: input.height,
            pixels_per_point: input.pixels_per_point,
            time_secs: input.time_secs,
            events,
            allowed_layers: input.allowed_layers,
            input_flags: input.input_flags,
        }
    }
}
