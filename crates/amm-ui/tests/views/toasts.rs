use amm_ui::views::{MenuButtonView, ToastsView};
use egui::{Pos2, Rect, vec2};

fn screen() -> Rect {
    Rect::from_min_size(Pos2::ZERO, vec2(1920.0, 1080.0))
}

#[test]
fn toasts_stack_above_the_menu_button_while_it_shows() {
    let button = MenuButtonView::rect(screen());

    let (right, bottom) = ToastsView::anchor(screen(), None, Some(button), 84.0);

    assert!(bottom < button.top(), "the stack ends at {bottom}, the button starts at {}", button.top());
    assert_eq!(right, screen().right() - ToastsView::MARGIN);
}

#[test]
fn toasts_sit_in_the_corner_without_the_menu_button() {
    let (_, bottom) = ToastsView::anchor(screen(), None, None, 84.0);

    assert_eq!(bottom, screen().bottom() - ToastsView::MARGIN);
}
