use egui::{Align2, CursorIcon, Rect, Sense, Stroke, StrokeKind, Ui, pos2, vec2};
use levi_i18n::text::Text;
use levi_ui::draw::{Corner, Draw, TextFit, TextStyle, TypeSize};
use levi_ui::font::FontFace;
use levi_ui::text::UiText;
use levi_ui::tokens::{ColorToken, SpacingToken, color, spacing};

use super::{StudioMode, TopBarAction, TopBarControl};
use crate::message::StudioUiMessage;
use crate::tokens::color as studio;

/// Paper: Editor components, `Top bar` (`G-0`), the Edit / Preview switch. A `--color-bg-field` track with a 1 px
/// `--color-line` border; the chosen half is `--color-bg-active` with semibold text, and Preview shows the mode dot.
pub(crate) struct ModeSwitch;

impl ModeSwitch {
    // Paper: the halves sit 2 inside the track and 2 apart; the preview dot is 6 across
    const INSET: f32 = 2.0;
    const GAP: f32 = 2.0;
    const DOT: f32 = 6.0;

    /// Paints the switch in `rect` for `mode` and pushes `SetMode` when the other half is clicked.
    pub(crate) fn show(
        ui: &mut Ui,
        rect: Rect,
        mode: StudioMode,
        actions: &mut Vec<TopBarAction>,
        places: &mut Vec<(TopBarControl, Rect)>,
    ) {
        let painter = ui.painter().clone();
        Draw::fill(&painter, rect, Corner::Control, <color::BgField as ColorToken>::VALUE);
        let border = Stroke::new(1.0, <color::Line as ColorToken>::VALUE);
        painter.rect_stroke(rect, Corner::Control.radius(rect), border, StrokeKind::Inside);
        let inner = rect.shrink(Self::INSET);
        let half = (inner.width() - Self::GAP) / 2.0;
        let halves = [
            (StudioMode::Edit, StudioUiMessage::ModeEdit, Rect::from_min_size(inner.min, vec2(half, inner.height()))),
            (
                StudioMode::Preview,
                StudioUiMessage::ModePreview,
                Rect::from_min_size(pos2(inner.left() + half + Self::GAP, inner.top()), vec2(half, inner.height())),
            ),
        ];
        for (choice, word, cell) in halves {
            let control = if choice == StudioMode::Edit { TopBarControl::Edit } else { TopBarControl::Preview };
            places.push((control, cell));
            let response = ui.allocate_rect(cell, Sense::click()).on_hover_cursor(CursorIcon::PointingHand);
            let chosen = choice == mode;
            if response.clicked() && !chosen {
                actions.push(TopBarAction::SetMode(choice));
            }
            let fill = match (chosen, response.hovered()) {
                (true, _) => Some(<color::BgActive as ColorToken>::VALUE),
                (false, true) => Some(<color::BgHover as ColorToken>::VALUE),
                (false, false) => None,
            };
            if let Some(fill) = fill {
                Draw::fill(&painter, cell, Corner::Small, fill);
            }
            Self::label(ui, cell, word, chosen, chosen && choice == StudioMode::Preview);
        }
    }

    // the half's word, centred, with the preview dot before it when `dot`
    fn label(ui: &Ui, cell: Rect, word: StudioUiMessage, chosen: bool, dot: bool) {
        let painter = ui.painter();
        let (face, ink) = if chosen {
            (FontFace::Semibold, <color::Text as ColorToken>::VALUE)
        } else {
            (FontFace::Regular, <color::TextDim as ColorToken>::VALUE)
        };
        let text = UiText::resolve(ui.ctx(), &Text::message(word));
        let galley = Draw::galley(painter, &text, TextStyle::new(TypeSize::Help, face), ink, TextFit::Line);
        let gap = <spacing::S as SpacingToken>::VALUE;
        let lead = if dot { Self::DOT + gap } else { 0.0 };
        let left = cell.center().x - (lead + galley.size().x) / 2.0;
        if dot {
            let colour = <studio::ModePreview as ColorToken>::VALUE;
            painter.circle_filled(pos2(left + Self::DOT / 2.0, cell.center().y), Self::DOT / 2.0, colour);
        }
        let at = Align2::LEFT_CENTER.anchor_size(pos2(left + lead, cell.center().y), galley.size());
        painter.galley(at.min, galley, ink);
    }
}
