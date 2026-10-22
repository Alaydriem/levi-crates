mod list_row_response;

pub use list_row_response::ListRowResponse;

use egui::{CursorIcon, Painter, Pos2, Rect, Sense, Ui, UiBuilder, pos2, vec2};
use levi_i18n::text::{Args, Text, UserText};

use crate::LeviUiMessage;
use crate::draw::{Corner, Draw, TextFit, TextStyle, TypeSize};
use crate::font::FontFace;
use crate::icon::Icon;
use crate::state::{InteractionState, StateStyle};
use crate::text::UiText;
use crate::tokens::rule::FocusRing;
use crate::tokens::{ColorToken, SpacingToken, color, spacing};
use crate::widget::{Avatar, Chip, ChipKind, ChipTone, Toggle};

/// Paper: Mod manager, `Widget / List row`. A mod in a list: an avatar, the name and version with
/// an optional conflict badge, a line of chips, and an optional toggle at the right. The whole row
/// selects; the toggle is its own click target. The name is cut to one line, the version never;
/// chips that do not fit become one "+N" chip.
pub struct ListRow<'a> {
    avatar: Avatar,
    name: Text,
    version: Option<Text>,
    badge: u32,
    chips: Vec<Chip>,
    toggle: Option<&'a mut bool>,
    selected: bool,
    off: bool,
}

impl<'a> ListRow<'a> {
    /// Paper: List row, the gap between the name line and the chip line.
    const LINE_GAP: f32 = 6.0;
    const NAME: TextStyle = TextStyle::new(TypeSize::Body, FontFace::Semibold);
    const VERSION: TextStyle = TextStyle::new(TypeSize::Note, FontFace::Regular);

    pub fn new(avatar: Avatar, name: Text) -> Self {
        Self { avatar, name, version: None, badge: 0, chips: Vec::new(), toggle: None, selected: false, off: false }
    }

    pub fn version(mut self, text: Text) -> Self {
        self.version = Some(text);
        self
    }

    /// A warning chip with the count after the version, shown when `count` is above 0.
    pub fn badge(mut self, count: u32) -> Self {
        self.badge = count;
        self
    }

    pub fn chips(mut self, chips: Vec<Chip>) -> Self {
        self.chips = chips;
        self
    }

    pub fn toggle(mut self, on: &'a mut bool) -> Self {
        self.toggle = Some(on);
        self
    }

    /// The open row: `--color-bg-active` and the accent bar at the left.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// The switched-off mod: the name, the avatar letter and the chip text dim.
    pub fn off(mut self, off: bool) -> Self {
        self.off = off;
        self
    }

    /// The row's height: the avatar or the two text lines, whichever is taller, and the padding.
    pub fn height(&self) -> f32 {
        self.avatar.side().max(self.text_height()) + 2.0 * <spacing::M as SpacingToken>::VALUE
    }

    pub fn show(mut self, ui: &mut Ui) -> ListRowResponse {
        let (rect, row) = ui.allocate_exact_size(vec2(ui.available_width(), self.height()), Sense::click());
        let state = InteractionState::of(&row, true);
        let toggle = self.toggle.take();
        self.paint_body(ui.painter(), rect, state, toggle.is_some());
        let mut toggled = false;
        if let Some(on) = toggle {
            // a child, placed after the row, so the toggle takes its own clicks
            let track = Self::track(rect);
            toggled = Toggle::new(on).show(&mut ui.new_child(UiBuilder::new().max_rect(track))).changed();
        }
        ListRowResponse { row: row.on_hover_cursor(CursorIcon::PointingHand), toggled }
    }

    /// Paints the row in `state`, for the gallery. The toggle is painted at rest.
    pub fn paint(mut self, painter: &Painter, rect: Rect, state: InteractionState) {
        let toggle = self.toggle.take().map(|on| *on);
        self.paint_body(painter, rect, state, toggle.is_some());
        if let Some(mut on) = toggle {
            Toggle::new(&mut on).paint(painter, Self::track(rect), InteractionState::Rest);
        }
    }

    // everything but the toggle; `toggle` keeps its room at the right
    fn paint_body(self, painter: &Painter, rect: Rect, state: InteractionState, toggle: bool) {
        let pad = <spacing::M as SpacingToken>::VALUE;
        let gap = <spacing::M as SpacingToken>::VALUE;
        let side = self.avatar.side();
        if state == InteractionState::Focused {
            FocusRing::paint(painter, rect, Corner::Panel.radius(rect));
        }
        let fill = if self.selected { <color::BgActive as ColorToken>::VALUE } else { StateStyle::Clear.fill(state) };
        Draw::fill(painter, rect, Corner::Panel, fill);
        if self.selected {
            let bar = Rect::from_min_size(
                pos2(rect.left(), rect.top() + pad),
                vec2(<spacing::Indicator as SpacingToken>::VALUE, side),
            );
            Draw::fill(painter, bar, Corner::Full, <color::Accent as ColorToken>::VALUE);
        }
        let avatar = Rect::from_min_size(pos2(rect.left() + pad, rect.center().y - side / 2.0), vec2(side, side));
        let right = if toggle { Self::track(rect).left() - gap } else { rect.right() - pad };
        let text_height = self.text_height();
        let avatar_look = if self.off { self.avatar.off(true) } else { self.avatar };
        avatar_look.paint(painter, avatar);

        let left = avatar.right() + gap;
        let top = rect.center().y - text_height / 2.0;
        Self::paint_name_line(painter, (left, right), top, (&self.name, self.version.as_ref(), self.badge, self.off));
        if !self.chips.is_empty() {
            let chips_top = top + TypeSize::Body.line_height() + Self::LINE_GAP;
            Self::paint_chips(painter, (left, right), chips_top, self.chips, self.off);
        }
    }

    // the name line, and the chip line under it when there are chips
    fn text_height(&self) -> f32 {
        let name_line = TypeSize::Body.line_height();
        if self.chips.is_empty() { name_line } else { name_line + Self::LINE_GAP + Chip::height() }
    }

    // the toggle's track, at the right inside the padding, centred on the height
    fn track(rect: Rect) -> Rect {
        let right = rect.right() - <spacing::M as SpacingToken>::VALUE;
        Rect::from_min_size(pos2(right - Toggle::TRACK.x, rect.center().y - Toggle::TRACK.y / 2.0), Toggle::TRACK)
    }

    // the name, cut to the room the version and the badge leave, then the version, then the badge
    fn paint_name_line(
        painter: &Painter,
        (left, right): (f32, f32),
        top: f32,
        (name, version, badge, off): (&Text, Option<&Text>, u32, bool),
    ) {
        let gap = <spacing::S as SpacingToken>::VALUE;
        let centre = top + TypeSize::Body.line_height() / 2.0;
        let faint = <color::TextFaint as ColorToken>::VALUE;
        let version = version.map(|version| {
            let version = UiText::resolve(painter.ctx(), version);
            Draw::galley(painter, &version, Self::VERSION, faint, TextFit::Line)
        });
        let badge = (badge > 0).then(|| {
            let count = Text::user(UserText::from_model(badge.to_string()));
            Chip::new(ChipKind::Display(ChipTone::Warning), count).icon(Icon::Warning)
        });
        let badge_width = badge.as_ref().map_or(0.0, |chip| gap + chip.desired_width(painter.ctx()));
        let version_width = version.as_ref().map_or(0.0, |galley| gap + galley.size().x);
        let room = (right - left - version_width - badge_width).max(0.0);
        let colour = if off { <color::TextDim as ColorToken>::VALUE } else { <color::Text as ColorToken>::VALUE };
        let name = UiText::resolve(painter.ctx(), name);
        let name = Draw::galley(painter, &name, Self::NAME, colour, TextFit::Truncate(room));
        let mut x = left + name.size().x;
        painter.galley(pos2(left, centre - name.size().y / 2.0), name, colour);
        if let Some(version) = version {
            x += gap;
            let width = version.size().x;
            painter.galley(pos2(x, centre - version.size().y / 2.0), version, faint);
            x += width;
        }
        if let Some(chip) = badge {
            x += gap;
            let size = vec2(chip.desired_width(painter.ctx()), Chip::height());
            chip.paint(painter, Rect::from_min_size(pos2(x, centre - size.y / 2.0), size), InteractionState::Rest);
        }
    }

    // the chips from `left`; a chip goes in only if the "+N" chip for the rest still fits after it
    fn paint_chips(painter: &Painter, (left, right): (f32, f32), top: f32, chips: Vec<Chip>, off: bool) {
        let gap = <spacing::Xs as SpacingToken>::VALUE;
        let count = chips.len();
        let mut x = left;
        for (index, chip) in chips.into_iter().enumerate() {
            let rest = count - index - 1;
            let reserve = if rest > 0 { gap + Self::more(rest).desired_width(painter.ctx()) } else { 0.0 };
            let width = chip.desired_width(painter.ctx());
            if x + width + reserve > right {
                // this chip and the rest become one "+N" chip, in the room kept for it
                let more = Self::more(count - index);
                let width = more.desired_width(painter.ctx());
                Self::paint_chip(painter, more.off(off), pos2(x, top), width);
                return;
            }
            Self::paint_chip(painter, chip.off(off), pos2(x, top), width);
            x += width + gap;
        }
    }

    fn paint_chip(painter: &Painter, chip: Chip, at: Pos2, width: f32) {
        chip.paint(painter, Rect::from_min_size(at, vec2(width, Chip::height())), InteractionState::Rest);
    }

    // the neutral chip that stands for `count` chips that do not fit
    fn more(count: usize) -> Chip {
        let count = u64::try_from(count).unwrap_or(u64::MAX);
        let text = Text::format(LeviUiMessage::MoreItems, Args::new().uint("count", count));
        Chip::new(ChipKind::Display(ChipTone::Neutral), text)
    }
}
