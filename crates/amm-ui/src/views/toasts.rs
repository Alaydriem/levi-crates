use amm_core::structs::toast::{Toast, ToastKind};
use egui::{Align2, Color32, Context, Id, LayerId, Order, Painter, Rect, pos2, vec2};
use studio_ui::theme::{Icon, Metrics, StudioPalette, Typeface};
use studio_ui::widgets::Draw;

use crate::model::{PanelModel, ToastText};
use crate::views::MenuButtonView;

/// The toast stack in the bottom-right corner, newest at the bottom, inside a 24 px screen margin.
/// While the menu button shows, the stack sits above it, so no toast covers the button. While the
/// panel is open and the stack does not fit below it, the stack moves inside the panel's
/// bottom-right corner, so no toast ever crosses the panel's bottom edge.
pub struct ToastsView;

impl ToastsView {
    pub const MARGIN: f32 = 24.0;
    const WIDTH: f32 = 480.0;
    const GAP: f32 = 8.0;
    // from the card top to the title, and between the lines of text
    const TOP: f32 = 14.0;
    const LINE_GAP: f32 = 2.0;
    // the progress track and the space above it
    const BOTTOM: f32 = 16.0;

    pub fn show(ctx: &Context, screen: Rect, panel: Option<Rect>, model: &PanelModel) {
        let count = model.toasts().count();
        if count == 0 {
            return;
        }
        let painter = ctx.layer_painter(LayerId::new(Order::Middle, Id::unique("amm.toasts")));
        let height = Self::height(&painter);
        let stack = count as f32 * (height + Self::GAP) - Self::GAP;
        let button = model.menu_button_visible().then(|| MenuButtonView::rect(screen));
        let (right, bottom) = Self::anchor(screen, panel, button, stack);
        let mut y = bottom;
        let top_limit = screen.top() + Self::MARGIN;
        let toasts: Vec<&Toast> = model.toasts().collect();
        for toast in toasts.iter().rev() {
            if y - height < top_limit {
                break;
            }
            let card = Rect::from_min_max(pos2(right - Self::WIDTH, y - height), pos2(right, y));
            Self::card(&painter, card, toast, model);
            y -= height + Self::GAP;
        }
    }

    // one card: the title, the message and the hint lines, then the progress track
    fn height(painter: &Painter) -> f32 {
        let title = Draw::line_height(painter, &Typeface::label_strong());
        let message = Draw::line_height(painter, &Typeface::help());
        let hint = Draw::line_height(painter, &Typeface::note());
        Self::TOP + title + Self::LINE_GAP + message + Self::LINE_GAP + hint + Self::BOTTOM
    }

    /// The bottom-right corner of a stack `stack` points high, given the open panel's rect and the
    /// menu button's rect while it shows.
    pub fn anchor(screen: Rect, panel: Option<Rect>, menu_button: Option<Rect>, stack: f32) -> (f32, f32) {
        let right = screen.right() - Self::MARGIN;
        let corner = screen.bottom() - Self::MARGIN;
        let bottom = menu_button.map_or(corner, |button| corner.min(button.top() - Self::GAP));
        let Some(panel) = panel else {
            return (right, bottom);
        };
        if bottom - stack >= panel.bottom() + Self::MARGIN {
            return (right, bottom);
        }
        (right.min(panel.right() - Self::MARGIN), bottom.min(panel.bottom() - Self::MARGIN))
    }

    fn card(painter: &Painter, card: Rect, toast: &Toast, model: &PanelModel) {
        Draw::shadow(painter, card, Metrics::RADIUS_PANEL, 24.0, vec2(0.0, 6.0), StudioPalette::SHADOW);
        Draw::fill(painter, card, Metrics::RADIUS_PANEL, StudioPalette::BG_RAISED);
        Draw::outline(painter, card, Metrics::RADIUS_PANEL, 1.0, StudioPalette::LINE_STRONG);
        let (icon, tint, soft) = Self::look(&toast.kind);
        let icon_centre = pos2(card.left() + 32.0, card.top() + 30.0);
        painter.circle_filled(icon_centre, 16.0, soft);
        Draw::icon(painter, icon_centre, icon, Typeface::ICON_SIZE, tint);
        let text = ToastText::render(&toast.kind, model);
        let (title, rest) = match text.split_once(". ") {
            Some((title, rest)) => (format!("{title}."), rest.to_string()),
            None => (text, String::new()),
        };
        let x = card.left() + 60.0;
        let title = Draw::text(
            painter,
            pos2(x, card.top() + Self::TOP),
            Align2::LEFT_TOP,
            &title,
            Typeface::label_strong(),
            StudioPalette::TEXT,
        );
        let message_top = title.bottom() + Self::LINE_GAP;
        let message_height = Draw::line_height(painter, &Typeface::help());
        Draw::text(painter, pos2(x, message_top), Align2::LEFT_TOP, &rest, Typeface::help(), StudioPalette::TEXT);
        if let Some(hint) = Self::hint(&toast.kind) {
            let hint_top = message_top + message_height + Self::LINE_GAP;
            Draw::text(painter, pos2(x, hint_top), Align2::LEFT_TOP, hint, Typeface::note(), StudioPalette::TEXT_DIM);
        }
        let age = model.now_ms().saturating_sub(toast.created_ms);
        let age_text = if age < 1000 { "now".to_string() } else { format!("{} s", age / 1000) };
        Draw::text(
            painter,
            pos2(card.right() - 14.0, card.top() + Self::TOP),
            Align2::RIGHT_TOP,
            &age_text,
            Typeface::note(),
            StudioPalette::TEXT_FAINT,
        );
        let track = Rect::from_min_max(
            pos2(card.left() + 12.0, card.bottom() - 6.0),
            pos2(card.right() - 12.0, card.bottom() - 4.0),
        );
        Draw::fill(painter, track, 1, StudioPalette::LINE);
        let left = 1.0 - (age as f32 / PanelModel::TOAST_TTL_MS as f32).clamp(0.0, 1.0);
        let progress = Rect::from_min_max(track.min, pos2(track.left() + track.width() * left, track.bottom()));
        Draw::fill(painter, progress, 1, tint);
    }

    fn look(kind: &ToastKind) -> (char, Color32, Color32) {
        match kind {
            ToastKind::Denied { .. } | ToastKind::Blocked { .. } => {
                (Icon::BLOCKED, StudioPalette::DANGER, StudioPalette::DANGER_SOFT)
            }
            ToastKind::FrameDisabled(_) | ToastKind::SaveFailed => {
                (Icon::WARNING, StudioPalette::WARNING, StudioPalette::WARNING_SOFT)
            }
        }
    }

    fn hint(kind: &ToastKind) -> Option<&'static str> {
        match kind {
            ToastKind::Denied { .. } | ToastKind::Blocked { .. } => {
                Some("Change the rule in the panel under Conflicts")
            }
            ToastKind::FrameDisabled(_) => Some("Turn it off and on in the panel to try again"),
            ToastKind::SaveFailed => None,
        }
    }
}
