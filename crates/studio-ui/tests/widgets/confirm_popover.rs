use studio_ui::widgets::{ConfirmChoice, ConfirmPopover};

use super::headless::Headless;

const TITLE: &str = "Discard this recording?";
const BODY: &str = "12:41 of capture will be deleted. You cannot undo this.";

#[test]
fn confirm_popover_is_272_points_wide() {
    let mut headless = Headless::new();

    let rect = headless.frame(Vec::new(), |ui| ConfirmPopover::show(ui, TITLE, BODY, "Keep recording", "Discard").1);

    assert_eq!(rect.width(), ConfirmPopover::WIDTH);
    assert_eq!(ConfirmPopover::WIDTH, 272.0);
}

#[test]
fn confirm_popover_reports_no_choice_without_a_click() {
    let mut headless = Headless::new();

    let choice = headless.frame(Vec::new(), |ui| ConfirmPopover::show(ui, TITLE, BODY, "Keep recording", "Discard").0);

    assert_eq!(choice, ConfirmChoice::None);
}

#[test]
fn confirm_popover_reports_confirm_click() {
    let mut headless = Headless::new();
    let rect = headless.frame(Vec::new(), |ui| ConfirmPopover::confirm_rect(ui, TITLE, BODY));

    let choice =
        headless.click(rect.center(), |ui| ConfirmPopover::show(ui, TITLE, BODY, "Keep recording", "Discard").0);

    assert_eq!(choice, ConfirmChoice::Confirm);
}
