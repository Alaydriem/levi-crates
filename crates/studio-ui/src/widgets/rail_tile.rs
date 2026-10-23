use egui::{Align2, Color32, Painter, Rect, Response, Sense, Ui, Vec2, pos2};
use levi_ui::draw::{Corner, Draw, TextFit, TextStyle, TypeSize};
use levi_ui::font::FontFace;
use levi_ui::state::InteractionState;
use levi_ui::tokens::rule::{Elevation, FocusRing};
use levi_ui::tokens::{ColorToken, RadiusToken, SpacingToken, TextSizeToken, color, radius, spacing, text_size};

use crate::widgets::{RailIcon, RailTone};

/// Paper: Entry Point, `Components · capture rail & corner buttons` (`A89-0`), "RAIL TILE". One square tile of the
/// capture rail: an icon over a short label on a floating card. Hover, pressed, focus and disabled follow the
/// levi-ui state model; Resume is the accent tile and Discard the danger one.
pub struct RailTile;

impl RailTile {
    /// Paper: the tile is `--spacing-tile-large` square.
    pub const SIDE: f32 = <spacing::TileLarge as SpacingToken>::VALUE;
    /// Paper: `--radius-panel`.
    pub const RADIUS: u8 = <radius::Panel as RadiusToken>::VALUE as u8;
    /// Paper: the icon is `--text-icon`.
    pub const ICON: f32 = <text_size::Icon as TextSizeToken>::VALUE;
    // Paper: the record dot is 20 across; the icon and label are 8 apart
    const DOT: f32 = 20.0;
    const GAP: f32 = 8.0;
    const LABEL: TextStyle = TextStyle::new(TypeSize::Help, FontFace::Semibold);

    pub fn show(ui: &mut Ui, icon: RailIcon, label: &str, tone: RailTone, enabled: bool) -> Response {
        let sense = if enabled { Sense::click() } else { Sense::hover() };
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(Self::SIDE), sense);
        Self::paint(ui.painter(), rect, icon, label, tone, InteractionState::of(&response, enabled));
        response
    }

    /// Paints the tile in `rect` in `state`.
    pub fn paint(painter: &Painter, rect: Rect, icon: RailIcon, label: &str, tone: RailTone, state: InteractionState) {
        let painter = state.painter(painter);
        if state == InteractionState::Focused {
            FocusRing::paint(&painter, rect, Corner::Panel.radius(rect));
        }
        let (fill, border, ink, label_ink) = Self::colours(tone, state.inner());
        Draw::surface(&painter, rect, Corner::Panel, fill, Some(border), Elevation::Floating);
        let galley = Draw::galley(&painter, label, Self::LABEL, label_ink, TextFit::Truncate(rect.width()));
        let mark = match icon {
            RailIcon::Glyph(_) => Self::ICON,
            RailIcon::RecordDot => Self::DOT,
        };
        let top = rect.center().y - (mark + Self::GAP + galley.size().y) / 2.0;
        let centre = pos2(rect.center().x, top + mark / 2.0);
        match icon {
            RailIcon::Glyph(glyph) => {
                let font = FontFace::Icons.id(Self::ICON);
                painter.text(centre, Align2::CENTER_CENTER, glyph, font, ink);
            }
            RailIcon::RecordDot => {
                let dot = if state.inner() == InteractionState::Rest {
                    <color::Playhead as ColorToken>::VALUE
                } else {
                    <color::PlayheadHover as ColorToken>::VALUE
                };
                painter.circle_filled(centre, Self::DOT / 2.0, dot);
            }
        }
        let at = Align2::CENTER_TOP.anchor_size(pos2(rect.center().x, top + mark + Self::GAP), galley.size());
        painter.galley(at.min, galley, label_ink);
    }

    // (fill, border, icon, label) for a tone in a state: Paper `A89-0` Default, Hover, Pressed, Resume and Discard
    fn colours(tone: RailTone, state: InteractionState) -> (Color32, Color32, Color32, Color32) {
        let text = <color::Text as ColorToken>::VALUE;
        match tone {
            RailTone::Normal => {
                let (fill, border) = match state {
                    InteractionState::Hover => {
                        (<color::BgHover as ColorToken>::VALUE, <color::LineStrong as ColorToken>::VALUE)
                    }
                    InteractionState::Pressed => {
                        (<color::BgActive as ColorToken>::VALUE, <color::LineStrong as ColorToken>::VALUE)
                    }
                    _ => (<color::Card as ColorToken>::VALUE, <color::LineOverlay as ColorToken>::VALUE),
                };
                (fill, border, text, text)
            }
            RailTone::Active => {
                let fill = match state {
                    InteractionState::Hover => <color::AccentSoftHover as ColorToken>::VALUE,
                    InteractionState::Pressed => <color::AccentSoftPress as ColorToken>::VALUE,
                    _ => <color::AccentSoft as ColorToken>::VALUE,
                };
                (fill, <color::Accent as ColorToken>::VALUE, <color::AccentHover as ColorToken>::VALUE, text)
            }
            RailTone::Danger => {
                let danger = <color::Danger as ColorToken>::VALUE;
                if state == InteractionState::Rest {
                    (<color::Card as ColorToken>::VALUE, <color::LineOverlay as ColorToken>::VALUE, danger, danger)
                } else {
                    (<color::DangerSoft as ColorToken>::VALUE, danger, danger, danger)
                }
            }
        }
    }
}
