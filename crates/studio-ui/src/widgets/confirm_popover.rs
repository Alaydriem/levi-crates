use std::sync::Arc;

use egui::{Align2, Galley, Rect, Response, Sense, Ui, pos2, vec2};

use crate::theme::{Metrics, StudioPalette, Typeface};
use crate::widgets::{ConfirmChoice, Draw};

// a small card that asks before a destructive action: title, body, a keep button and a red confirm button
pub struct ConfirmPopover;

impl ConfirmPopover {
    pub const WIDTH: f32 = 272.0;
    const PAD: f32 = 16.0;
    const GAP: f32 = 10.0;
    const BUTTON_HEIGHT: f32 = 30.0;
    const CONFIRM_WIDTH: f32 = 96.0;
    const BUTTON_GAP: f32 = 6.0;

    // shows the popover at the cursor; returns the choice made this frame and the card rect
    pub fn show(ui: &mut Ui, title: &str, body: &str, keep: &str, confirm: &str) -> (ConfirmChoice, Rect) {
        let (title_galley, body_galley, height) = Self::measure(ui, title, body);
        let (card, _) = ui.allocate_exact_size(vec2(Self::WIDTH, height), Sense::hover());
        let (keep_rect, confirm_rect) = Self::buttons(card);
        let keep_response = ui.interact(keep_rect, ui.scope_id().with("confirm-popover-keep"), Sense::click());
        let confirm_response = ui.interact(confirm_rect, ui.scope_id().with("confirm-popover-confirm"), Sense::click());

        let painter = ui.painter();
        Draw::card(painter, card);
        let title_pos = pos2(card.left() + Self::PAD, card.top() + Self::PAD);
        let title_rect = Draw::place(painter, title_galley, title_pos, Align2::LEFT_TOP);
        Draw::place(painter, body_galley, pos2(title_pos.x, title_rect.bottom() + Self::GAP), Align2::LEFT_TOP);
        Self::button(ui, &keep_response, keep, false);
        Self::button(ui, &confirm_response, confirm, true);

        let choice = if confirm_response.clicked() {
            ConfirmChoice::Confirm
        } else if keep_response.clicked() {
            ConfirmChoice::Keep
        } else {
            ConfirmChoice::None
        };
        (choice, card)
    }

    // where the confirm button would sit if the popover were shown at the cursor now
    pub fn confirm_rect(ui: &Ui, title: &str, body: &str) -> Rect {
        let (_, _, height) = Self::measure(ui, title, body);
        let card = Rect::from_min_size(ui.cursor().min, vec2(Self::WIDTH, height));
        Self::buttons(card).1
    }

    fn measure(ui: &Ui, title: &str, body: &str) -> (Arc<Galley>, Arc<Galley>, f32) {
        let painter = ui.painter();
        let inner = Self::WIDTH - Self::PAD * 2.0;
        let title_galley = Draw::galley(painter, title, Typeface::label_strong(), StudioPalette::TEXT);
        let body_galley = painter.layout(body.to_owned(), Typeface::help(), StudioPalette::TEXT_DIM, inner);
        let height = Self::PAD
            + title_galley.size().y
            + Self::GAP
            + body_galley.size().y
            + Metrics::GAP_M
            + Self::BUTTON_HEIGHT
            + Self::PAD;
        (title_galley, body_galley, height)
    }

    fn buttons(card: Rect) -> (Rect, Rect) {
        let bottom = card.bottom() - Self::PAD;
        let confirm = Rect::from_min_max(
            pos2(card.right() - Self::PAD - Self::CONFIRM_WIDTH, bottom - Self::BUTTON_HEIGHT),
            pos2(card.right() - Self::PAD, bottom),
        );
        let keep = Rect::from_min_max(
            pos2(card.left() + Self::PAD, confirm.top()),
            pos2(confirm.left() - Self::BUTTON_GAP, bottom),
        );
        (keep, confirm)
    }

    fn button(ui: &Ui, response: &Response, label: &str, danger: bool) {
        let hot = response.hovered() || response.has_focus();
        let fill = match (danger, hot) {
            (true, false) => StudioPalette::DANGER,
            (true, true) => StudioPalette::mix(StudioPalette::DANGER, StudioPalette::WHITE, 0.12),
            (false, false) => StudioPalette::BG_HOVER,
            (false, true) => StudioPalette::BG_ACTIVE,
        };
        let ink = if danger { StudioPalette::WHITE } else { StudioPalette::TEXT };
        let painter = ui.painter();
        Draw::fill(painter, response.rect, Metrics::RADIUS_CONTROL, fill);
        Draw::text(painter, response.rect.center(), Align2::CENTER_CENTER, label, Typeface::help_strong(), ink);
    }
}
