use crate::structs::registry::ModIndex;

/// One capture the shim is to take at the next Present. Plain data, so a fixed list of them copies without
/// allocating.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaptureRequest {
    pub sequence: u32,
    pub mod_index: ModIndex,
}

impl CaptureRequest {
    pub fn new(sequence: u32, mod_index: ModIndex) -> Self {
        Self { sequence, mod_index }
    }
}
