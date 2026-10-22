use crate::structs::menu::{CornerSlot, CornerTarget};

/// Lays out the corner row of the start and pause screens: the square Mods button at the bottom-right, then each
/// mod's entry to its left in registration order. All sizes are in points.
pub struct CornerRow;

impl CornerRow {
    pub const RIGHT_MARGIN: f32 = 24.0;
    pub const BOTTOM_MARGIN: f32 = 48.0;
    pub const HEIGHT: f32 = 36.0;
    pub const GAP: f32 = 6.0;

    /// Writes the slots for a screen of `width` by `height` into `out` (cleared first): Mods first, then one slot
    /// per entry with the width the view measured for it.
    pub fn layout(width: f32, height: f32, entries: &[(CornerTarget, f32)], out: &mut Vec<CornerSlot>) {
        out.clear();
        let bottom = height - Self::BOTTOM_MARGIN;
        let top = bottom - Self::HEIGHT;
        let mut right = width - Self::RIGHT_MARGIN;
        let mods = [(CornerTarget::Mods, Self::HEIGHT)];
        for (target, slot_width) in mods.iter().chain(entries) {
            let left = right - slot_width;
            out.push(CornerSlot { target: *target, rect: [left, top, right, bottom] });
            right = left - Self::GAP;
        }
    }

    /// The rect around every slot, or `None` for no slots.
    pub fn bounds(slots: &[CornerSlot]) -> Option<[f32; 4]> {
        slots
            .iter()
            .map(|slot| slot.rect)
            .reduce(|a, b| [a[0].min(b[0]), a[1].min(b[1]), a[2].max(b[2]), a[3].max(b[3])])
    }
}
