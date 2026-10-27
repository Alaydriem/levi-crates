mod mode_switch;
mod studio_mode;
mod top_bar_action;
mod top_bar_control;
mod top_bar_view;

use mode_switch::ModeSwitch;
pub use studio_mode::StudioMode;
pub use top_bar_action::TopBarAction;
pub use top_bar_control::TopBarControl;
pub use top_bar_view::TopBarView;

use egui::{Align2, Rangef, Rect, Response, Sense, Ui, UiBuilder, pos2, vec2};
use levi_i18n::text::Text;
use levi_ui::brand::{DvMark, DvMarkSize};
use levi_ui::draw::{Draw, TextFit, TextStyle, TypeSize};
use levi_ui::font::FontFace;
use levi_ui::icon::{Icon, IconSize};
use levi_ui::state::DisabledReason;
use levi_ui::text::UiText;
use levi_ui::tokens::{ColorToken, SpacingToken, color, spacing};
use levi_ui::widget::{ButtonKind, IconButton, IconButtonSize, Leading, TextButton};

use super::{BarControl, SceneSwitcherChip};
use crate::message::StudioUiMessage;
use crate::tokens::color as studio;
use crate::tokens::spacing as studio_spacing;

/// Paper: Editor components, `Top bar` (`G-0`). The brand, the breadcrumb with the scene switcher chip, the
/// Edit / Preview switch, and the actions: undo, redo, undo history, settings, close, Coverage, Queue and Export.
pub struct TopBar<'a> {
    view: &'a TopBarView,
}

impl<'a> TopBar<'a> {
    // Paper `G-0`: padding 13 left and 12 right; the brand parts and the breadcrumb parts 10 apart
    const PADDING_LEFT: f32 = 13.0;
    const PADDING_RIGHT: f32 = 12.0;
    const GROUP_GAP: f32 = 10.0;
    // Paper: the breadcrumb starts 14 after the brand divider
    const BREADCRUMB_PADDING: f32 = 14.0;
    // Paper: the brand divider is 18 high, the action divider 20
    const BRAND_DIVIDER: f32 = 18.0;
    const ACTION_DIVIDER: f32 = 20.0;
    // Paper: the switch is 220 wide with a fixed 260 between it and the actions
    const MODE_WIDTH: f32 = 220.0;
    const MODE_GAP: f32 = 260.0;
    // Paper: undo, redo and history are 2 apart
    const HISTORY_GAP: f32 = 2.0;
    const NAME: TextStyle = TextStyle::new(TypeSize::Body, FontFace::Semibold);
    const TITLE: TextStyle = TextStyle::new(TypeSize::Help, FontFace::Regular);

    pub fn new(view: &'a TopBarView) -> Self {
        Self { view }
    }

    pub fn show(&self, ui: &mut Ui) -> Vec<TopBarAction> {
        self.show_placed(ui).0
    }

    /// Like `show`, with the rect of each control, in points.
    pub fn show_placed(&self, ui: &mut Ui) -> (Vec<TopBarAction>, Vec<(TopBarControl, Rect)>) {
        let height = <studio_spacing::TopBar as SpacingToken>::VALUE;
        let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
        self.paint_bar(ui, rect);
        let mut actions = Vec::new();
        let mut places = Vec::new();
        let actions_left = self.actions(ui, rect, &mut actions, &mut places);
        let brand_right = self.brand(ui, rect);
        // the space before the actions gives way first, then the title; the chip always fits
        let ideal = actions_left - Self::MODE_GAP - Self::MODE_WIDTH;
        let breadcrumb = brand_right + self.breadcrumb_width(ui.ctx()) + Self::GROUP_GAP;
        let latest = actions_left - Self::GROUP_GAP - Self::MODE_WIDTH;
        let mode_left = ideal.max(breadcrumb).min(latest);
        let mode = Rect::from_min_size(
            pos2(mode_left, rect.center().y - BarControl::HEIGHT / 2.0),
            vec2(Self::MODE_WIDTH, BarControl::HEIGHT),
        );
        ModeSwitch::show(ui, mode, self.view.mode, &mut actions, &mut places);
        self.breadcrumb(ui, rect, brand_right, mode.left() - Self::GROUP_GAP, &mut actions, &mut places);
        // `allocate_rect` moved the cursor to each control; the page goes on under the bar
        ui.advance_cursor_after_rect(rect);
        (actions, places)
    }

    fn paint_bar(&self, ui: &Ui, rect: Rect) {
        let painter = ui.painter();
        painter.rect_filled(rect, 0, <color::BgApp as ColorToken>::VALUE);
        let line = <color::Line as ColorToken>::VALUE;
        let bottom = if self.view.mode == StudioMode::Preview {
            painter.rect_filled(rect, 0, <studio::ModePreviewSoft as ColorToken>::VALUE);
            <studio::ModePreview as ColorToken>::VALUE
        } else {
            line
        };
        Draw::divider(painter, rect.x_range(), rect.top() + 0.5, line);
        Draw::divider(painter, rect.x_range(), rect.bottom() - 0.5, bottom);
    }

    // the DV mark, the studio name and the divider after them; returns the divider's right edge
    fn brand(&self, ui: &Ui, rect: Rect) -> f32 {
        let painter = ui.painter();
        let mark = DvMark { size: DvMarkSize::Regular };
        let side = DvMarkSize::Regular.side();
        let square = mark.paint(
            painter,
            Rect::from_center_size(
                pos2(rect.left() + Self::PADDING_LEFT + side / 2.0, rect.center().y),
                vec2(side, side),
            ),
        );
        let ink = <color::Text as ColorToken>::VALUE;
        let name = Text::message(StudioUiMessage::StudioName);
        let at = pos2(square.right() + Self::GROUP_GAP, rect.center().y);
        let name = Draw::text(painter, at, Align2::LEFT_CENTER, &name, Self::NAME, ink, TextFit::Line);
        let x = name.right() + Self::GROUP_GAP;
        let line =
            Rangef::new(rect.center().y - Self::BRAND_DIVIDER / 2.0, rect.center().y + Self::BRAND_DIVIDER / 2.0);
        painter.vline(x + 0.5, line, egui::Stroke::new(1.0, <color::LineStrong as ColorToken>::VALUE));
        x + 1.0
    }

    // the breadcrumb's width with the whole title
    fn breadcrumb_width(&self, ctx: &egui::Context) -> f32 {
        let title = Draw::measure(ctx, &UiText::resolve(ctx, &self.view.title), Self::TITLE).x;
        let chip = SceneSwitcherChip::new(&self.view.scene).width_in(ctx);
        Self::BREADCRUMB_PADDING + title + Self::GROUP_GAP * 2.0 + IconSize::Small.points() + chip
    }

    // the replay title (cut to fit), a chevron and the scene chip, between `left` and `right`
    fn breadcrumb(
        &self,
        ui: &mut Ui,
        rect: Rect,
        left: f32,
        right: f32,
        actions: &mut Vec<TopBarAction>,
        places: &mut Vec<(TopBarControl, Rect)>,
    ) {
        let x = left + Self::BREADCRUMB_PADDING;
        let chevron = IconSize::Small.points();
        // the chip may take all the room the title gives up, and no more
        let chip = SceneSwitcherChip::new(&self.view.scene).max_width(right - x - Self::GROUP_GAP * 2.0 - chevron);
        let chip_width = chip.width_in(ui.ctx());
        let room = (right - x - Self::GROUP_GAP * 2.0 - chevron - chip_width).max(0.0);
        let painter = ui.painter();
        let dim = <color::TextDim as ColorToken>::VALUE;
        let title = UiText::resolve(ui.ctx(), &self.view.title);
        let galley = Draw::galley(painter, &title, Self::TITLE, dim, TextFit::Truncate(room));
        let title_rect = Align2::LEFT_CENTER.anchor_size(pos2(x, rect.center().y), galley.size());
        painter.galley(title_rect.min, galley, dim);
        let icon = Rect::from_center_size(
            pos2(title_rect.right() + Self::GROUP_GAP + chevron / 2.0, rect.center().y),
            vec2(chevron, chevron),
        );
        Icon::Chevron.paint(painter, icon, IconSize::Small, <color::TextFaint as ColorToken>::VALUE);
        let chip_rect = Rect::from_min_size(
            pos2(icon.right() + Self::GROUP_GAP, rect.center().y - BarControl::HEIGHT / 2.0),
            vec2(chip_width, BarControl::HEIGHT),
        );
        places.push((TopBarControl::SceneChip, chip_rect));
        let response = ui.scope_builder(UiBuilder::new().max_rect(chip_rect), |ui| chip.show(ui)).inner;
        if response.clicked() {
            actions.push(TopBarAction::OpenSceneSwitcher);
        }
    }

    // the actions from the right edge leftwards; returns the left edge of the group
    fn actions(
        &self,
        ui: &mut Ui,
        rect: Rect,
        actions: &mut Vec<TopBarAction>,
        places: &mut Vec<(TopBarControl, Rect)>,
    ) -> f32 {
        let gap = <spacing::S as SpacingToken>::VALUE;
        let mut right = rect.right() - Self::PADDING_RIGHT;
        let buttons = [
            (
                ButtonKind::Primary,
                Icon::Share,
                StudioUiMessage::Export,
                TopBarAction::OpenExport,
                TopBarControl::Export,
            ),
            (ButtonKind::Outline, Icon::List, StudioUiMessage::Queue, TopBarAction::OpenQueue, TopBarControl::Queue),
            (
                ButtonKind::Outline,
                Icon::Checklist,
                StudioUiMessage::Coverage,
                TopBarAction::OpenCoverage,
                TopBarControl::Coverage,
            ),
        ];
        for (kind, icon, word, action, control) in buttons {
            let button = TextButton::new(kind, Text::message(word)).leading(Leading::Icon(icon));
            let width = button.desired_width(ui.ctx());
            let place = Rect::from_min_size(
                pos2(right - width, rect.center().y - BarControl::HEIGHT / 2.0),
                vec2(width, BarControl::HEIGHT),
            );
            places.push((control, place));
            if Self::place(ui, place, |ui| BarControl::text(ui, &button, true)).clicked() {
                actions.push(action);
            }
            right = place.left() - gap;
        }
        let icons = [
            (Icon::Back, StudioUiMessage::CloseReplay, TopBarAction::CloseReplay, TopBarControl::Close),
            (Icon::Settings, StudioUiMessage::Settings, TopBarAction::OpenSettings, TopBarControl::Settings),
        ];
        for (icon, word, action, control) in icons {
            let side = IconButtonSize::Regular.side();
            let place = Rect::from_min_size(pos2(right - side, rect.center().y - side / 2.0), vec2(side, side));
            let button = IconButton::new(icon).tooltip(Text::message(word));
            places.push((control, place));
            if Self::place(ui, place, |ui| button.show(ui)).clicked() {
                actions.push(action);
            }
            right = place.left() - gap;
        }
        let line =
            Rangef::new(rect.center().y - Self::ACTION_DIVIDER / 2.0, rect.center().y + Self::ACTION_DIVIDER / 2.0);
        ui.painter().vline(right - 0.5, line, egui::Stroke::new(1.0, <color::LineStrong as ColorToken>::VALUE));
        right -= 1.0 + gap;
        self.history(ui, rect, right, actions, places)
    }

    // undo, redo and the undo history button, ending at `right`; returns their left edge
    fn history(
        &self,
        ui: &mut Ui,
        rect: Rect,
        right: f32,
        actions: &mut Vec<TopBarAction>,
        places: &mut Vec<(TopBarControl, Rect)>,
    ) -> f32 {
        let small = IconButtonSize::Small.side();
        let side = IconButtonSize::Regular.side();
        let history = Rect::from_min_size(pos2(right - small, rect.center().y - small / 2.0), vec2(small, small));
        places.push((TopBarControl::UndoHistory, history));
        let button = IconButton::new(Icon::ChevronDown)
            .size(IconButtonSize::Small)
            .on(self.view.history_open)
            .tooltip(Text::message(StudioUiMessage::UndoHistory));
        if Self::place(ui, history, |ui| button.show(ui)).clicked() {
            actions.push(TopBarAction::OpenUndoHistory);
        }
        let redo = Rect::from_min_size(
            pos2(history.left() - Self::HISTORY_GAP - side, rect.center().y - side / 2.0),
            vec2(side, side),
        );
        let undo = redo.translate(vec2(-(side + Self::HISTORY_GAP), 0.0));
        let steps = [
            (
                undo,
                Icon::Undo,
                StudioUiMessage::Undo,
                StudioUiMessage::NothingToUndo,
                self.view.can_undo,
                TopBarAction::Undo,
                TopBarControl::Undo,
            ),
            (
                redo,
                Icon::Redo,
                StudioUiMessage::Redo,
                StudioUiMessage::NothingToRedo,
                self.view.can_redo,
                TopBarAction::Redo,
                TopBarControl::Redo,
            ),
        ];
        for (place, icon, word, reason, enabled, action, control) in steps {
            places.push((control, place));
            let button =
                IconButton::new(icon).tooltip(Text::message(word)).enabled(enabled, DisabledReason::message(reason));
            if Self::place(ui, place, |ui| button.show(ui)).clicked() && enabled {
                actions.push(action);
            }
        }
        undo.left()
    }

    // shows a control at `place`
    fn place(ui: &mut Ui, place: Rect, show: impl FnOnce(&mut Ui) -> Response) -> Response {
        ui.scope_builder(UiBuilder::new().max_rect(place), show).inner
    }
}
