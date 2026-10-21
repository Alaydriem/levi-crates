use amm_core::structs::frame::UiEvent;
use amm_core::structs::input::{InputEvent, Modifiers};

use crate::services::{Boundary, HostCell};
use crate::types::{AmmHostPointerKind, AmmHostRoute};

/// A key from the window or the game. `mods` is a mask: 1 Ctrl, 2 Shift, 4 Alt. `consume` in the answer means the
/// game must not see the key.
#[unsafe(no_mangle)]
pub extern "C" fn amm_host_key(vk: u32, down: bool, repeat: bool, mods: u8, now_ms: u64) -> AmmHostRoute {
    let mods = Modifiers {
        ctrl: mods & UiEvent::MOD_CTRL != 0,
        shift: mods & UiEvent::MOD_SHIFT != 0,
        alt: mods & UiEvent::MOD_ALT != 0,
    };
    let ev = InputEvent::Key { vk, down, repeat, mods };
    Boundary::guard(|| HostCell::global().with(|runtime| Ok(runtime.route(ev, now_ms)))).unwrap_or(AmmHostRoute::PASS)
}

/// A pointer event in physical pixels. `kind` is an `AmmHostPointerKind` value; any other value passes to the
/// game. `button` and `down` are read for `Button`, `wheel` for `Wheel`, and `x`, `y` are the deltas for `Delta`.
#[unsafe(no_mangle)]
pub extern "C" fn amm_host_pointer(kind: u8, x: f32, y: f32, button: u8, down: bool, wheel: f32) -> AmmHostRoute {
    Boundary::guard(|| {
        let ev = match AmmHostPointerKind::try_from(kind)? {
            AmmHostPointerKind::Move => InputEvent::PointerMove { x, y },
            AmmHostPointerKind::Button => InputEvent::PointerButton { button, down, x, y },
            AmmHostPointerKind::Wheel => InputEvent::Wheel { delta: wheel },
            AmmHostPointerKind::Delta => InputEvent::PointerDelta { dx: x, dy: y },
        };
        HostCell::global().with(|runtime| Ok(runtime.route(ev, 0)))
    })
    .unwrap_or(AmmHostRoute::PASS)
}

/// One character of text input, as a Unicode code point.
#[unsafe(no_mangle)]
pub extern "C" fn amm_host_char(ch: u32) -> AmmHostRoute {
    Boundary::guard(|| HostCell::global().with(|runtime| Ok(runtime.route(InputEvent::Char(ch), 0))))
        .unwrap_or(AmmHostRoute::PASS)
}

/// The game window lost focus: every held key and button is released to the mods that got its press.
#[unsafe(no_mangle)]
pub extern "C" fn amm_host_focus_lost() {
    let _ = Boundary::guard(|| HostCell::global().with(|runtime| Ok(runtime.route(InputEvent::FocusLost, 0))));
}
