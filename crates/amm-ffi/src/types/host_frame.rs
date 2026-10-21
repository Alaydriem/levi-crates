use amm_abi::types::AmmCursorShape;
use amm_core::structs::capture::PendingCaptures;

use super::AmmHostLayer;

/// The most captures one `AmmHostFrame` asks for, one per shim staging texture.
pub const AMM_HOST_MAX_CAPTURES: usize = 4;

const _: () = assert!(AMM_HOST_MAX_CAPTURES == PendingCaptures::CAPACITY);

/// The Presents a capture may take from the frame that offers it to `amm_host_capture_done`. The manager fails a
/// capture not reported by then, and the shim stops waiting on its staging copy after as many Presents.
pub const AMM_HOST_CAPTURE_MAX_FRAMES: u32 = 30;

/// What the shim draws for one Present: `layer_count` layers at `layers`, bottom to top, valid until the next
/// `amm_host_frame`. `freed_owners` has bit N set when mod slot N left since the last frame, so the shim drops
/// every texture of that owner. `cursor_free` releases the mouse grab and shows `cursor`; `hide_vanilla_hud`
/// hides the game HUD.
///
/// The first `capture_count` entries of `captures` are the sequences of the screenshots to take at this
/// Present: the shim copies the back buffer into a staging texture for each, after the game has drawn and before
/// it draws any layer. Each sequence is given once. When the copy can be read, the shim asks for a buffer with
/// `amm_host_capture_buffer`, copies the rows into it and calls `amm_host_capture_done`; a capture it cannot
/// copy it reports with `ok` 0. A capture not reported within 30 frames fails.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AmmHostFrame {
    pub layers: *const AmmHostLayer,
    pub layer_count: u32,
    pub freed_owners: u32,
    pub cursor_free: bool,
    pub cursor: AmmCursorShape,
    pub hide_vanilla_hud: bool,
    pub capture_count: u32,
    pub captures: [u32; AMM_HOST_MAX_CAPTURES],
}

impl AmmHostFrame {
    /// No layers, the cursor grabbed, the HUD shown and no capture.
    pub fn empty() -> Self {
        Self {
            layers: std::ptr::null(),
            layer_count: 0,
            freed_owners: 0,
            cursor_free: false,
            cursor: AmmCursorShape::Default,
            hide_vanilla_hud: false,
            capture_count: 0,
            captures: [0; AMM_HOST_MAX_CAPTURES],
        }
    }
}
