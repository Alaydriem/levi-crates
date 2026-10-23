mod job_status;
mod render_job_row;
mod render_queue_action;
mod render_queue_view;

pub use job_status::JobStatus;
pub use render_job_row::RenderJobRow;
pub use render_queue_action::RenderQueueAction;
pub use render_queue_view::RenderQueueView;

use egui::{Align2, Context, Id, Rect, Sense, Ui, UiBuilder, pos2, vec2};
use levi_i18n::text::{Args, Text};
use levi_ui::draw::{Corner, Draw, TextFit, TextStyle, TypeSize};
use levi_ui::font::FontFace;
use levi_ui::icon::Icon;
use levi_ui::state::InteractionState;
use levi_ui::tokens::{ColorToken, color};
use levi_ui::widget::{ButtonKind, EmptyState, IconButton, Leading, ModalFrame, ModalResponse, StatusPill, TextButton};

use crate::id::StudioUiId;
use crate::message::StudioUiMessage;

/// Paper: Menus + Dialogs, `Render queue` (`9A9-0` rendering, `9D1-0` paused). The jobs in the levi-ui `ModalFrame`:
/// a thumbnail with the scene's colour, the name, settings and file, and the status (a progress bar while
/// rendering, else a status pill), with Retry or Show file where they apply and the job menu button.
pub struct RenderQueue<'a> {
    view: &'a RenderQueueView,
}

impl<'a> RenderQueue<'a> {
    /// Paper: the queue card is 820 wide.
    const WIDTH: f32 = 820.0;
    // Paper: rows are 84 high, 8 apart, 12 in, parts 14 apart; the thumbnail 104 × 58 with a 2 px scene bar 6 in
    // and 2 up
    const ROW: f32 = 84.0;
    const ROW_GAP: f32 = 8.0;
    const ROW_PADDING: f32 = 12.0;
    const PART_GAP: f32 = 14.0;
    const THUMB: [f32; 2] = [104.0, 58.0];
    const SCENE_BAR: f32 = 2.0;
    const SCENE_INSET: f32 = 6.0;
    // Paper: the status column is 254 wide, the bar 6 high; the job menu button 28 square
    const STATUS: f32 = 254.0;
    const BAR: f32 = 6.0;
    const MENU: f32 = 28.0;
    const LINE_GAP: f32 = 5.0;
    // the empty queue's height in the card
    const EMPTY: f32 = 240.0;

    pub fn new(view: &'a RenderQueueView) -> Self {
        Self { view }
    }

    pub fn show(&self, ctx: &Context) -> Option<RenderQueueAction> {
        let (mut body, mut footer) = (None, None);
        let response = self.frame().show(ctx, |ui| body = self.body(ui), |ui| footer = self.footer(ui));
        Self::action(response, body.or(footer))
    }

    /// Shows the card where `ui` places it, for the gallery.
    pub fn show_card(&self, ui: &mut Ui) -> Option<RenderQueueAction> {
        let (mut body, mut footer) = (None, None);
        let response = self.frame().show_card(ui, |ui| body = self.body(ui), |ui| footer = self.footer(ui));
        Self::action(response, body.or(footer))
    }

    fn frame(&self) -> ModalFrame {
        ModalFrame::new(Id::unique(StudioUiId::RenderQueue), Text::message(StudioUiMessage::RenderQueue))
            .subtitle(self.view.subtitle.clone())
            .width(Self::WIDTH)
    }

    fn body(&self, ui: &mut Ui) -> Option<RenderQueueAction> {
        // Paper packs the rows `ROW_GAP` apart, not the modal body's gap
        ui.spacing_mut().item_spacing.y = 0.0;
        if self.view.jobs.is_empty() {
            let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), Self::EMPTY), Sense::hover());
            let title = Text::message(StudioUiMessage::QueueEmpty);
            let message = Text::message(StudioUiMessage::QueueEmptyHint);
            EmptyState::new(Icon::Video, title, message).show(&mut ui.new_child(UiBuilder::new().max_rect(rect)));
            return None;
        }
        let mut action = None;
        for (index, job) in self.view.jobs.iter().enumerate() {
            let (row, _) = ui.allocate_exact_size(vec2(ui.available_width(), Self::ROW), Sense::hover());
            action = self.row(ui, row, index, job).or(action);
            ui.add_space(Self::ROW_GAP);
        }
        let note = Text::message(StudioUiMessage::AudioNote);
        let dim = <color::TextDim as ColorToken>::VALUE;
        let (rect, _) =
            ui.allocate_exact_size(vec2(ui.available_width(), TypeSize::Note.line_height()), Sense::hover());
        let style = TextStyle::new(TypeSize::Note, FontFace::Regular);
        Draw::text(ui.painter(), rect.left_center(), Align2::LEFT_CENTER, &note, style, dim, TextFit::Line);
        action
    }

    fn row(&self, ui: &mut Ui, row: Rect, index: usize, job: &RenderJobRow) -> Option<RenderQueueAction> {
        let painter = ui.painter().clone();
        let border = if job.status == JobStatus::Rendering {
            <color::Accent as ColorToken>::VALUE
        } else {
            <color::Line as ColorToken>::VALUE
        };
        Draw::fill(&painter, row, Corner::Panel, <color::BgPanel as ColorToken>::VALUE);
        Draw::stroke(&painter, row, Corner::Panel, border);
        let inner = row.shrink2(vec2(Self::ROW_PADDING, 0.0));
        let thumb = Rect::from_min_size(
            pos2(inner.left(), row.center().y - Self::THUMB[1] / 2.0),
            vec2(Self::THUMB[0], Self::THUMB[1]),
        );
        Draw::fill(&painter, thumb, Corner::Control, <color::BgApp as ColorToken>::VALUE);
        let bar = Rect::from_min_max(
            pos2(thumb.left() + Self::SCENE_INSET, thumb.bottom() - Self::SCENE_BAR * 2.0),
            pos2(thumb.right() - Self::SCENE_INSET, thumb.bottom() - Self::SCENE_BAR),
        );
        Draw::fill(&painter, bar, Corner::Full, job.scene.color());
        let menu = Rect::from_min_size(
            pos2(inner.right() - Self::MENU, row.center().y - Self::MENU / 2.0),
            vec2(Self::MENU, Self::MENU),
        );
        let status = Rect::from_min_max(
            pos2(menu.left() - Self::PART_GAP - Self::STATUS, row.top()),
            pos2(menu.left() - Self::PART_GAP, row.bottom()),
        );
        let text = Rect::from_min_max(
            pos2(thumb.right() + Self::PART_GAP, row.top()),
            pos2(status.left() - Self::PART_GAP, row.bottom()),
        );
        Self::text(&painter, text, job);
        let mut action = self.status(ui, status, index, job);
        let button = IconButton::new(Icon::ChevronDown).tooltip(Text::message(StudioUiMessage::JobMenu));
        let response = button.show(&mut ui.new_child(UiBuilder::new().max_rect(menu)));
        if response.clicked() {
            action = Some(RenderQueueAction::OpenJobMenu(index, menu.left_bottom()));
        }
        action
    }

    // the name, the settings line and the file name, stacked in the middle of the row
    fn text(painter: &egui::Painter, rect: Rect, job: &RenderJobRow) {
        let lines = [
            (&job.name, TextStyle::new(TypeSize::Label, FontFace::Semibold), <color::Text as ColorToken>::VALUE),
            (&job.meta, TextStyle::new(TypeSize::Note, FontFace::Regular), <color::TextDim as ColorToken>::VALUE),
            (&job.file, TextStyle::new(TypeSize::Caption, FontFace::Regular), <color::TextDim as ColorToken>::VALUE),
        ];
        let height: f32 =
            lines.iter().map(|(_, style, _)| style.size.line_height()).sum::<f32>() + Self::LINE_GAP * 2.0;
        let mut y = rect.center().y - height / 2.0;
        for (text, style, ink) in lines {
            Draw::text(
                painter,
                pos2(rect.left(), y),
                Align2::LEFT_TOP,
                text,
                style,
                ink,
                TextFit::Truncate(rect.width().max(0.0)),
            );
            y += style.size.line_height() + Self::LINE_GAP;
        }
    }

    // the status column: the bar while rendering, else the pill, the detail and Retry or Show file
    fn status(&self, ui: &mut Ui, rect: Rect, index: usize, job: &RenderJobRow) -> Option<RenderQueueAction> {
        let painter = ui.painter().clone();
        let caption = TextStyle::new(TypeSize::Caption, FontFace::Regular);
        if job.status == JobStatus::Rendering {
            let percent = if job.progress.is_nan() { 0.0 } else { job.progress.clamp(0.0, 1.0) };
            let words = Text::format(
                StudioUiMessage::RenderingPercent,
                Args::new().uint("percent", (percent * 100.0).round() as u64),
            );
            let ink = <color::Text as ColorToken>::VALUE;
            let semibold = TextStyle::new(TypeSize::Note, FontFace::Semibold);
            let top = rect.center().y - Self::BAR / 2.0 - TypeSize::Note.line_height() - Self::LINE_GAP;
            let head =
                Draw::text(&painter, pos2(rect.left(), top), Align2::LEFT_TOP, &words, semibold, ink, TextFit::Line);
            let dim = <color::TextDim as ColorToken>::VALUE;
            Draw::text(
                &painter,
                pos2(rect.right(), head.center().y),
                Align2::RIGHT_CENTER,
                &job.detail,
                caption,
                dim,
                TextFit::Line,
            );
            let track = Rect::from_min_size(
                pos2(rect.left(), rect.center().y - Self::BAR / 2.0),
                vec2(rect.width(), Self::BAR),
            );
            Draw::fill(&painter, track, Corner::Full, <color::BgField as ColorToken>::VALUE);
            if percent > 0.0 {
                let fill = Rect::from_min_size(track.min, vec2(track.width() * percent, Self::BAR));
                Draw::fill(&painter, fill, Corner::Full, <color::Accent as ColorToken>::VALUE);
            }
            let micro = TextStyle::new(TypeSize::Micro, FontFace::Regular);
            let faint = <color::TextFaint as ColorToken>::VALUE;
            Draw::text(
                &painter,
                pos2(rect.left(), track.bottom() + Self::LINE_GAP),
                Align2::LEFT_TOP,
                &job.note,
                micro,
                faint,
                TextFit::Truncate(rect.width()),
            );
            return None;
        }
        let pill = StatusPill::new(job.status.tone(), job.status.label());
        let pill_rect = pill.paint(&painter, pos2(rect.left(), rect.center().y), Align2::LEFT_CENTER);
        let detail_ink = if job.status == JobStatus::Failed {
            crate::rule::LabelTint::of(<color::Danger as ColorToken>::VALUE)
        } else {
            <color::TextFaint as ColorToken>::VALUE
        };
        let at = pos2(pill_rect.right() + Self::LINE_GAP * 2.0, rect.center().y);
        let (kind, word, act): (ButtonKind, StudioUiMessage, fn(usize) -> RenderQueueAction) = match job.status {
            JobStatus::Failed | JobStatus::Cancelled => {
                (ButtonKind::Outline, StudioUiMessage::Retry, RenderQueueAction::Retry)
            }
            JobStatus::Done => (ButtonKind::Ghost, StudioUiMessage::ShowFile, RenderQueueAction::ShowFile),
            _ => {
                Draw::text(
                    &painter,
                    at,
                    Align2::LEFT_CENTER,
                    &job.detail,
                    caption,
                    detail_ink,
                    TextFit::Truncate((rect.right() - at.x).max(0.0)),
                );
                return None;
            }
        };
        let button = TextButton::new(kind, Text::message(word));
        let width = button.desired_width(ui.ctx());
        let place = Rect::from_min_size(
            pos2(rect.right() - width, rect.center().y - Self::MENU / 2.0),
            vec2(width, Self::MENU),
        );
        let room = (place.left() - Self::LINE_GAP - at.x).max(0.0);
        Draw::text(&painter, at, Align2::LEFT_CENTER, &job.detail, caption, detail_ink, TextFit::Truncate(room));
        // a child ui, so the button does not move the row cursor
        let mut child = ui.new_child(UiBuilder::new().max_rect(place));
        let response = child.allocate_rect(place, Sense::click());
        button.paint(&painter, place, InteractionState::of(&response, true));
        response.clicked().then(|| act(index))
    }

    // Close, then Pause or Resume and Clear finished (the footer runs right to left)
    fn footer(&self, ui: &mut Ui) -> Option<RenderQueueAction> {
        let close = TextButton::new(ButtonKind::Secondary, Text::message(StudioUiMessage::Close)).show(ui).clicked();
        let (icon, word, toggle) = if self.view.paused {
            (Icon::Play, StudioUiMessage::Resume, RenderQueueAction::ResumeAll)
        } else {
            (Icon::Pause, StudioUiMessage::Pause, RenderQueueAction::PauseAll)
        };
        let pause =
            TextButton::new(ButtonKind::Ghost, Text::message(word)).leading(Leading::Icon(icon)).show(ui).clicked();
        let clear =
            TextButton::new(ButtonKind::Ghost, Text::message(StudioUiMessage::ClearFinished)).show(ui).clicked();
        if close {
            Some(RenderQueueAction::Close)
        } else if pause {
            Some(toggle)
        } else {
            clear.then_some(RenderQueueAction::ClearFinished)
        }
    }

    fn action(response: ModalResponse, action: Option<RenderQueueAction>) -> Option<RenderQueueAction> {
        if response.closed { Some(RenderQueueAction::Close) } else { action }
    }
}
