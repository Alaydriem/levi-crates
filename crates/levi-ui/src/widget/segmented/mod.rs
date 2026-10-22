mod segment;

pub use segment::Segment;

use egui::{Color32, CursorIcon, Id, Key, Painter, Rect, Response, Sense, Stroke, Ui, pos2, vec2};

use crate::draw::{Corner, Draw, TextFit, TextStyle, TypeSize};
use crate::font::FontFace;
use crate::icon::IconSize;
use crate::id::LeviUiId;
use crate::motion::Motion;
use crate::state::{ArrowKeys, DisabledReason, InteractionState, StateStyle};
use crate::text::UiText;
use crate::tokens::rule::FocusRing;
use crate::tokens::{ColorToken, SpacingToken, color, spacing};

/// Paper: Widgets, `Widget / Segmented`. Segments in a `--color-bg-field` track; the selected one
/// has the soft fill and slides when the choice changes.
pub struct Segmented<'a> {
    id: Id,
    selected: &'a mut usize,
    segments: Vec<Segment>,
    disabled: Option<DisabledReason>,
}

impl<'a> Segmented<'a> {
    /// Paper: Segmented sheet, the inset between the track and the segments.
    const INSET: f32 = 2.0;
    /// Paper: Segmented sheet, the divider height.
    const DIVIDER_HEIGHT: f32 = 14.0;

    pub fn new(id: Id, selected: &'a mut usize, segments: Vec<Segment>) -> Self {
        Self { id, selected, segments, disabled: None }
    }

    pub fn enabled(mut self, enabled: bool, reason: DisabledReason) -> Self {
        self.disabled = if enabled { None } else { Some(reason) };
        self
    }

    /// Shows the control `width` wide; an all-icon control hugs its segments instead.
    pub fn show(self, ui: &mut Ui, width: f32) -> Response {
        let enabled = self.disabled.is_none();
        let width = if self.all_icons() { self.icon_width() } else { width };
        let (rect, mut response) = ui.allocate_exact_size(vec2(width, Self::height()), Sense::hover());
        let cells = self.cells(rect);
        let mut states = Vec::with_capacity(cells.len());
        let mut focused = None;
        for (index, cell) in cells.iter().enumerate() {
            let sense = if enabled { Sense::click() } else { Sense::hover() };
            let segment = ui.interact(*cell, self.id.with(index), sense);
            if segment.clicked() && *self.selected != index {
                *self.selected = index;
                response.mark_changed();
            }
            if segment.has_focus() {
                ArrowKeys::keep(ui, segment.id);
                focused = Some(index);
            }
            states.push(InteractionState::of(&segment, enabled));
            if enabled {
                response |= segment.on_hover_cursor(CursorIcon::PointingHand);
            }
        }
        if let Some(index) = focused {
            let step = ui.input(|input| {
                i32::from(input.key_pressed(Key::ArrowRight)) - i32::from(input.key_pressed(Key::ArrowLeft))
            });
            let target = (index as i64 + i64::from(step)).clamp(0, cells.len() as i64 - 1) as usize;
            if step != 0 && target != *self.selected {
                *self.selected = target;
                response.mark_changed();
                ui.memory_mut(|memory| memory.request_focus(self.id.with(target)));
            }
        }
        let target_left = cells.get(*self.selected).map_or(rect.left(), |cell| cell.left());
        let slide = Motion::value(ui.ctx(), self.id.with(LeviUiId::SegmentSlide), target_left);
        self.paint_with(ui.painter(), rect, &states, slide);
        match &self.disabled {
            None => response,
            Some(reason) => reason.tooltip().attach(response),
        }
    }

    /// Paints the control with `state` on segment `target` and the others at rest, for the gallery.
    /// Disabled applies to the whole control.
    pub fn paint(&self, painter: &Painter, rect: Rect, target: usize, state: InteractionState) {
        let cells = self.cells(rect);
        let states: Vec<_> = (0..cells.len())
            .map(|index| match state {
                InteractionState::Disabled => InteractionState::Disabled,
                _ if index == target => state,
                _ => InteractionState::Rest,
            })
            .collect();
        let left = cells.get(*self.selected).map_or(rect.left(), |cell| cell.left());
        self.paint_with(painter, rect, &states, left);
    }

    /// The width an all-icon control takes.
    pub fn icon_width(&self) -> f32 {
        self.segments.len() as f32 * <spacing::Button as SpacingToken>::VALUE + Self::INSET * 2.0
    }

    fn paint_with(&self, painter: &Painter, rect: Rect, states: &[InteractionState], slide_left: f32) {
        let disabled = states.contains(&InteractionState::Disabled);
        let painter = if disabled { InteractionState::Disabled.painter(painter) } else { painter.clone() };
        let cells = self.cells(rect);
        let selected = *self.selected;
        Draw::fill(&painter, rect, Corner::Control, <color::BgField as ColorToken>::VALUE);
        // the ring goes under the focused segment, which then repaints the track behind it
        for (index, cell) in cells.iter().enumerate() {
            if states.get(index) == Some(&InteractionState::Focused) {
                FocusRing::paint(&painter, *cell, Corner::Small.radius(*cell));
                Draw::fill(&painter, *cell, Corner::Small, <color::BgField as ColorToken>::VALUE);
            }
        }
        for (index, cell) in cells.iter().enumerate() {
            let state = states.get(index).copied().unwrap_or(InteractionState::Rest).inner();
            if index == selected {
                let moved = cell.translate(vec2(slide_left - cell.left(), 0.0));
                Draw::fill(&painter, moved, Corner::Small, StateStyle::Soft.fill(state));
            } else {
                Draw::fill(&painter, *cell, Corner::Small, StateStyle::Clear.fill(state));
            }
        }
        self.paint_dividers(&painter, &cells, states);
        for (index, (cell, segment)) in cells.iter().zip(&self.segments).enumerate() {
            let state = states.get(index).copied().unwrap_or(InteractionState::Rest);
            self.paint_segment(&painter, *cell, segment, index == selected, state);
        }
    }

    // a divider shows between two neighbours only when neither is selected or touched
    fn paint_dividers(&self, painter: &Painter, cells: &[Rect], states: &[InteractionState]) {
        let quiet = |index: usize| {
            index != *self.selected
                && matches!(states.get(index), Some(InteractionState::Rest | InteractionState::Disabled) | None)
        };
        let stroke = Stroke::new(<spacing::Divider as SpacingToken>::VALUE, <color::LineStrong as ColorToken>::VALUE);
        for (index, cell) in cells.iter().enumerate().skip(1) {
            if quiet(index - 1) && quiet(index) {
                let x = cell.left().round() - stroke.width / 2.0;
                let y = cell.center().y;
                painter.line_segment(
                    [pos2(x, y - Self::DIVIDER_HEIGHT / 2.0), pos2(x, y + Self::DIVIDER_HEIGHT / 2.0)],
                    stroke,
                );
            }
        }
    }

    fn paint_segment(&self, painter: &Painter, cell: Rect, segment: &Segment, selected: bool, state: InteractionState) {
        let color: Color32 = match (selected, state) {
            (true, _) => <color::AccentHover as ColorToken>::VALUE,
            (false, InteractionState::Hover | InteractionState::Pressed | InteractionState::Focused) => {
                <color::Text as ColorToken>::VALUE
            }
            (false, InteractionState::Rest | InteractionState::Disabled) => <color::TextDim as ColorToken>::VALUE,
        };
        match segment {
            Segment::Icon(icon) => icon.paint(painter, cell, IconSize::Regular, color),
            Segment::Text(text) => {
                let face = if selected { FontFace::Semibold } else { FontFace::Regular };
                let label = UiText::resolve(painter.ctx(), text);
                let galley = Draw::galley(
                    painter,
                    &label,
                    TextStyle::new(TypeSize::Note, face),
                    color,
                    TextFit::Truncate(cell.width()),
                );
                painter.galley(cell.center() - galley.size() / 2.0, galley, color);
            }
        }
    }

    // the segment rects inside the inset: icon segments are `--spacing-button` wide, text segments
    // share the width equally
    fn cells(&self, rect: Rect) -> Vec<Rect> {
        let inner = rect.shrink(Self::INSET);
        let count = self.segments.len().max(1);
        let width =
            if self.all_icons() { <spacing::Button as SpacingToken>::VALUE } else { inner.width() / count as f32 };
        (0..self.segments.len())
            .map(|index| {
                Rect::from_min_size(pos2(inner.left() + width * index as f32, inner.top()), vec2(width, inner.height()))
            })
            .collect()
    }

    fn all_icons(&self) -> bool {
        !self.segments.is_empty() && self.segments.iter().all(|segment| matches!(segment, Segment::Icon(_)))
    }

    fn height() -> f32 {
        <spacing::Field as SpacingToken>::VALUE
    }
}
