use amm_core::services::settings::ChordText;
use amm_core::structs::descriptor::KeyScope;
use egui::{Align2, Response, Sense, Ui, pos2, vec2};
use studio_ui::theme::{Icon, Metrics, StudioPalette, Typeface};
use studio_ui::widgets::{Chip, ChipStyle, Draw};

use crate::model::{CapabilityText, KeybindRow, PanelModel};
use crate::views::TableHeader;

/// One keybind row of a keybind table: label, chord, capability and scope, with the clash marker and
/// the "Press a key" prompt while the row waits for its new chord. The Keybinds tab and the
/// all-keybinds page both draw their rows with it.
pub struct KeybindRowView;

impl KeybindRowView {
    pub const COLS: [f32; 4] = [0.0, 0.40, 0.66, 0.84];
    pub const TITLES: [&'static str; 4] = ["Action", "Key chord", "Capability", "Scope"];
    // a plain row fits a key cap with a gap above and below
    const ROW: f32 = Chip::KEY_HEIGHT + 2.0 * Metrics::GAP_M;
    const LINE_GAP: f32 = 4.0;
    pub const PRESS_A_KEY: &'static str = "Press a key";

    /// Draws the row and returns its response; a click on it asks for a rebind. `prompt` is the text
    /// shown in place of the chord while the row waits for its new chord: "Press a key", or the
    /// modifiers held so far. `rule` draws the line above a plain row.
    pub fn show(
        ui: &mut Ui,
        model: &PanelModel,
        row: &KeybindRow,
        width: f32,
        prompt: Option<&str>,
        rule: bool,
    ) -> Response {
        let label_width = (width * Self::COLS[1] - TableHeader::INSET - Metrics::GAP_M).max(1.0);
        let label = ui.painter().layout(row.label.to_string(), Typeface::body(), StudioPalette::TEXT, label_width);
        let sub_height = Draw::line_height(ui.painter(), &Typeface::help());
        let text_height = match row.clash_with {
            Some(_) => label.size().y + Self::LINE_GAP + sub_height,
            None => label.size().y,
        };
        let prompt_lines = prompt.map(|_| {
            let first = Draw::line_height(ui.painter(), &Typeface::help_strong());
            (first, first + Self::LINE_GAP + Draw::line_height(ui.painter(), &Typeface::note()))
        });
        let prompt_height = prompt_lines.map_or(0.0, |(_, total)| total);
        let height = (text_height.max(prompt_height) + 2.0 * Metrics::GAP_M).max(Self::ROW);
        let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::click());
        let painter = ui.painter();
        if prompt.is_some() {
            Draw::fill(painter, rect, Metrics::RADIUS_CONTROL + 2, StudioPalette::ACCENT_SOFT);
            Draw::outline(painter, rect, Metrics::RADIUS_CONTROL + 2, 1.0, StudioPalette::ACCENT);
        } else if row.clash_with.is_some() {
            Draw::fill(painter, rect, Metrics::RADIUS_CONTROL + 2, StudioPalette::DANGER_SOFT);
            Draw::outline(
                painter,
                rect,
                Metrics::RADIUS_CONTROL + 2,
                1.0,
                StudioPalette::alpha(StudioPalette::DANGER, 0.45),
            );
        } else if response.hovered() {
            Draw::fill(painter, rect, Metrics::RADIUS_CONTROL + 2, StudioPalette::BG_HOVER);
        } else if rule {
            Draw::hline(painter, rect.left() + 8.0, rect.right() - 8.0, rect.top(), StudioPalette::LINE);
        }
        let x0 = rect.left() + TableHeader::INSET;
        let y = rect.center().y;
        let chord_x = TableHeader::column(rect.left(), width, Self::COLS[1]);
        let top = y - text_height * 0.5;
        let label_bottom = top + label.size().y;
        painter.galley(pos2(x0, top), label, StudioPalette::TEXT);
        if let Some(other) = row.clash_with {
            let sub = format!("Also bound by {}", model.display_name(other));
            let sub_y = label_bottom + Self::LINE_GAP + sub_height * 0.5;
            Draw::icon(painter, pos2(x0 + 6.0, sub_y), Icon::WARNING, Typeface::ICON_SMALL, StudioPalette::DANGER);
            Draw::text(
                painter,
                pos2(x0 + 20.0, sub_y),
                Align2::LEFT_CENTER,
                &sub,
                Typeface::help(),
                StudioPalette::DANGER,
            );
        }
        if let (Some(prompt), Some((first, total))) = (prompt, prompt_lines) {
            let prompt_top = y - total * 0.5;
            Draw::text(
                painter,
                pos2(chord_x, prompt_top),
                Align2::LEFT_TOP,
                prompt,
                Typeface::help_strong(),
                StudioPalette::ACCENT_HOVER,
            );
            Draw::text(
                painter,
                pos2(chord_x, prompt_top + first + Self::LINE_GAP),
                Align2::LEFT_TOP,
                "Esc cancels",
                Typeface::note(),
                StudioPalette::TEXT_DIM,
            );
        } else {
            let text = ChordText::format(row.chord);
            let keys: Vec<&str> = text.split('+').collect();
            Chip::chord(painter, chord_x, y, &keys, row.clash_with.is_some());
        }
        let cap_x = TableHeader::column(rect.left(), width, Self::COLS[2]);
        match row.capability {
            Some(capability) => {
                let style = if row.exclusive { ChipStyle::ACCENT.with_icon(Icon::LOCK) } else { ChipStyle::NEUTRAL };
                Chip::paint(painter, cap_x, y, CapabilityText::name(capability), style);
            }
            None => {
                Draw::text(
                    painter,
                    pos2(cap_x + 4.0, y),
                    Align2::LEFT_CENTER,
                    "\u{2014}",
                    Typeface::body(),
                    StudioPalette::TEXT_FAINT,
                );
            }
        }
        let scope = match row.scope {
            KeyScope::Global => "Global",
            KeyScope::WhileHeld => "While held",
        };
        let scope_x = TableHeader::column(rect.left(), width, Self::COLS[3]);
        Draw::text(painter, pos2(scope_x, y), Align2::LEFT_CENTER, scope, Typeface::help(), StudioPalette::TEXT_DIM);
        response
    }
}
