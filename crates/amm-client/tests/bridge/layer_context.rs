use std::sync::{Arc, Mutex};

use amm_abi::types::{AmmFrameInput, AmmStatus};
use amm_client::bridge::{ClipboardBackend, EguiFrame};
use amm_core::structs::frame::FrameInputData;

use crate::support::{ClipboardPass, ClipboardProbeHandler, FakeTable, FocusAction, FocusProbeHandler, FocusTarget};

fn keyboard_after_pass(frame: &mut EguiFrame<FocusProbeHandler>) -> u8 {
    let input = AmmFrameInput {
        width: 640,
        height: 480,
        pixels_per_point: 1.0,
        time_secs: 0.0,
        events: std::ptr::null(),
        event_count: 0,
        allowed_layers: FrameInputData::LAYER_OVERLAY,
        input_flags: 7,
    };
    let mut out = FakeTable::empty_output();
    // SAFETY: the input has no events, so it points at nothing
    let status = unsafe { frame.run(&input, &mut out) };
    assert_eq!(status, AmmStatus::Ok);
    out.wants_keyboard
}

#[test]
fn wants_keyboard_is_set_while_a_text_field_has_focus() {
    let (handler, action) = FocusProbeHandler::new(FocusTarget::TextField);
    let mut frame = EguiFrame::new(handler);
    assert_eq!(keyboard_after_pass(&mut frame), 0);

    action.set(FocusAction::Request);
    keyboard_after_pass(&mut frame);

    assert_eq!(keyboard_after_pass(&mut frame), 1);
}

#[test]
fn a_focused_button_does_not_set_wants_keyboard() {
    let (handler, action) = FocusProbeHandler::new(FocusTarget::Button);
    let mut frame = EguiFrame::new(handler);
    keyboard_after_pass(&mut frame);

    action.set(FocusAction::Request);
    keyboard_after_pass(&mut frame);

    assert_eq!(keyboard_after_pass(&mut frame), 0);
}

#[test]
fn wants_keyboard_clears_when_the_field_loses_focus() {
    let (handler, action) = FocusProbeHandler::new(FocusTarget::TextField);
    let mut frame = EguiFrame::new(handler);
    keyboard_after_pass(&mut frame);
    action.set(FocusAction::Request);
    keyboard_after_pass(&mut frame);
    assert_eq!(keyboard_after_pass(&mut frame), 1);

    action.set(FocusAction::Surrender);
    keyboard_after_pass(&mut frame);

    assert_eq!(keyboard_after_pass(&mut frame), 0);
}

#[test]
fn wants_keyboard_stays_set_while_the_focused_field_is_scrolled_out_of_view() {
    let (handler, action) = FocusProbeHandler::new(FocusTarget::ScrolledOutTextField);
    let mut frame = EguiFrame::new(handler);
    keyboard_after_pass(&mut frame);

    action.set(FocusAction::Request);
    keyboard_after_pass(&mut frame);

    assert_eq!(keyboard_after_pass(&mut frame), 1);
}

#[test]
fn ctrl_c_in_a_focused_text_field_copies_its_selection() {
    let memory = Arc::new(Mutex::new(String::new()));
    let (handler, action, _) = ClipboardProbeHandler::new("hello world");
    let mut frame = EguiFrame::with_clipboard(handler, ClipboardBackend::Memory(Arc::clone(&memory)));
    ClipboardPass::focus(&mut frame, &action);
    ClipboardPass::select_all(&mut frame);

    ClipboardPass::ctrl_c(&mut frame);

    assert!(ClipboardPass::wait_for_text(&memory, "hello world"));
}

#[test]
fn ctrl_v_pastes_into_the_focused_field_on_a_later_pass() {
    let memory = Arc::new(Mutex::new("pasted".to_owned()));
    let (handler, action, text) = ClipboardProbeHandler::new("");
    let mut frame = EguiFrame::with_clipboard(handler, ClipboardBackend::Memory(memory));
    ClipboardPass::focus(&mut frame, &action);

    ClipboardPass::ctrl_v(&mut frame);

    assert!(ClipboardPass::run_until(&mut frame, || text.borrow().as_str() == "pasted"));
}
