mod export_dialog_action;
mod export_dialog_view;
mod export_format;
mod export_frame_rate;
mod export_resolution;

pub use export_dialog_action::ExportDialogAction;
pub use export_dialog_view::ExportDialogView;
pub use export_format::ExportFormat;
pub use export_frame_rate::ExportFrameRate;
pub use export_resolution::ExportResolution;

use egui::{Align, Align2, Context, CursorIcon, Id, Layout, Rect, Sense, Ui, pos2, vec2};
use levi_i18n::text::Text;
use levi_ui::draw::{Corner, Draw, TextFit, TextStyle, TypeSize};
use levi_ui::font::FontFace;
use levi_ui::icon::{Icon, IconSize};
use levi_ui::tokens::{ColorToken, color};
use levi_ui::widget::{ButtonKind, Leading, ModalFrame, ModalResponse, Segment, Segmented, Slider, TextButton, Toggle};

use crate::id::StudioUiId;
use crate::message::StudioUiMessage;

/// Paper: Menus + Dialogs, `Export dialog` (`971-0`). The export settings in the levi-ui `ModalFrame`: format,
/// resolution and frame rate as `Segmented` controls, quality as a `Slider`, motion blur and the sidecar file as
/// `Toggle`s, the output path, and a note; then Render now, Add to queue and Open queue.
pub struct ExportDialog<'a> {
    view: &'a ExportDialogView,
}

impl<'a> ExportDialog<'a> {
    /// Paper: the export card is 700 wide.
    const WIDTH: f32 = 700.0;
    // Paper: rows are 48 high with a 140 label column; controls are 400 wide, notes 12 to 14 after them
    const ROW: f32 = 48.0;
    const LABEL: f32 = 140.0;
    const CONTROL: f32 = 400.0;
    const NOTE_GAP: f32 = 12.0;
    // Paper: the output field is 32 high with 12 each side; the note panel 78 high with 16 each side
    const FIELD: f32 = 32.0;
    const FIELD_PADDING: f32 = 12.0;
    const PANEL: f32 = 78.0;
    const PANEL_PADDING: f32 = 16.0;

    pub fn new(view: &'a ExportDialogView) -> Self {
        Self { view }
    }

    pub fn show(&self, ctx: &Context) -> Option<ExportDialogAction> {
        let (mut body, mut footer) = (None, None);
        let response = self.frame().show(ctx, |ui| body = self.body(ui), |ui| footer = Self::footer(ui));
        Self::action(response, body.or(footer))
    }

    /// Shows the card where `ui` places it, for the gallery.
    pub fn show_card(&self, ui: &mut Ui) -> Option<ExportDialogAction> {
        let (mut body, mut footer) = (None, None);
        let response = self.frame().show_card(ui, |ui| body = self.body(ui), |ui| footer = Self::footer(ui));
        Self::action(response, body.or(footer))
    }

    fn frame(&self) -> ModalFrame {
        ModalFrame::new(Id::unique(StudioUiId::ExportDialog), Text::message(StudioUiMessage::ExportScene))
            .subtitle(self.view.subtitle.clone())
            .width(Self::WIDTH)
    }

    fn body(&self, ui: &mut Ui) -> Option<ExportDialogAction> {
        let view = self.view;
        let mut action = None;
        let mut format = ExportFormat::ALL.iter().position(|item| *item == view.format).unwrap_or(0);
        if Self::row(ui, StudioUiMessage::FormatLabel, None, |ui| {
            let segments = ExportFormat::ALL.iter().map(|item| Segment::Text(item.label())).collect();
            Segmented::new(Id::unique(StudioUiId::ExportFormat), &mut format, segments)
                .show(ui, Self::CONTROL)
                .changed()
        }) {
            action = Some(ExportDialogAction::SetFormat(ExportFormat::ALL[format]));
        }
        let mut resolution = ExportResolution::ALL.iter().position(|item| *item == view.resolution).unwrap_or(0);
        if Self::row(ui, StudioUiMessage::ResolutionLabel, Some(&view.resolution.size()), |ui| {
            let segments = ExportResolution::ALL.iter().map(|item| Segment::Text(item.label())).collect();
            let id = Id::unique(StudioUiId::ExportResolution);
            Segmented::new(id, &mut resolution, segments).show(ui, Self::CONTROL).changed()
        }) {
            action = Some(ExportDialogAction::SetResolution(ExportResolution::ALL[resolution]));
        }
        let mut rate = ExportFrameRate::ALL.iter().position(|item| *item == view.frame_rate).unwrap_or(0);
        if Self::row(ui, StudioUiMessage::FrameRateLabel, None, |ui| {
            let segments = ExportFrameRate::ALL.iter().map(|item| Segment::Text(item.label())).collect();
            Segmented::new(Id::unique(StudioUiId::ExportFrameRate), &mut rate, segments)
                .show(ui, Self::CONTROL)
                .changed()
        }) {
            action = Some(ExportDialogAction::SetFrameRate(ExportFrameRate::ALL[rate]));
        }
        let mut quality = view.quality.clamp(0.0, 1.0);
        if Self::row(ui, StudioUiMessage::QualityLabel, Some(&view.quality_label), |ui| {
            Slider::new(&mut quality, 0.0..=1.0).show(ui, Self::CONTROL).changed()
        }) {
            action = Some(ExportDialogAction::SetQuality(quality));
        }
        let mut blur = view.motion_blur;
        if Self::row(ui, StudioUiMessage::MotionBlurLabel, Some(&view.motion_blur_note), |ui| {
            Toggle::new(&mut blur).show(ui).changed()
        }) {
            action = Some(ExportDialogAction::SetMotionBlur(blur));
        }
        let mut sidecar = view.sidecar;
        if Self::row(ui, StudioUiMessage::SidecarLabel, Some(&view.sidecar_note), |ui| {
            Toggle::new(&mut sidecar).show(ui).changed()
        }) {
            action = Some(ExportDialogAction::SetSidecar(sidecar));
        }
        if Self::row(ui, StudioUiMessage::OutputLabel, None, |ui| self.output(ui)) {
            action = Some(ExportDialogAction::ChooseOutput);
        }
        self.panel(ui);
        action
    }

    // a settings row: its small-caps label, `control`, then `note` when given; returns what `control` returned
    fn row(ui: &mut Ui, label: StudioUiMessage, note: Option<&Text>, control: impl FnOnce(&mut Ui) -> bool) -> bool {
        let width = ui.available_width();
        ui.allocate_ui_with_layout(vec2(width, Self::ROW), Layout::left_to_right(Align::Center), |ui| {
            ui.set_min_height(Self::ROW);
            let (cell, _) = ui.allocate_exact_size(vec2(Self::LABEL, Self::ROW), Sense::hover());
            let head = TextStyle::new(TypeSize::Caption, FontFace::Semibold).tracked();
            let faint = <color::TextFaint as ColorToken>::VALUE;
            Draw::text(
                ui.painter(),
                cell.left_center(),
                Align2::LEFT_CENTER,
                &Text::message(label),
                head,
                faint,
                TextFit::Truncate(Self::LABEL),
            );
            let changed = control(ui);
            if let Some(note) = note {
                ui.add_space(Self::NOTE_GAP);
                let style = TextStyle::new(TypeSize::Note, FontFace::Regular);
                let dim = <color::TextDim as ColorToken>::VALUE;
                let room = ui.available_width().max(0.0);
                let (rect, _) = ui.allocate_exact_size(vec2(room, Self::ROW), Sense::hover());
                Draw::text(
                    ui.painter(),
                    rect.left_center(),
                    Align2::LEFT_CENTER,
                    note,
                    style,
                    dim,
                    TextFit::Truncate(room),
                );
            }
            changed
        })
        .inner
    }

    // the output field: a folder icon, the folder and the file name; true when clicked
    fn output(&self, ui: &mut Ui) -> bool {
        let width = ui.available_width();
        let (rect, response) = ui.allocate_exact_size(vec2(width, Self::FIELD), Sense::click());
        let response = response.on_hover_cursor(CursorIcon::PointingHand);
        let painter = ui.painter();
        let fill = if response.hovered() {
            <color::BgHover as ColorToken>::VALUE
        } else {
            <color::BgField as ColorToken>::VALUE
        };
        Draw::fill(painter, rect, Corner::Control, fill);
        let icon = IconSize::Small.points();
        let place = Rect::from_min_size(
            pos2(rect.left() + Self::FIELD_PADDING, rect.center().y - icon / 2.0),
            vec2(icon, icon),
        );
        let dim = <color::TextDim as ColorToken>::VALUE;
        Icon::Folder.paint(painter, place, IconSize::Small, dim);
        let caption = TextStyle::new(TypeSize::Caption, FontFace::Regular);
        let x = place.right() + Self::NOTE_GAP / 1.5;
        let room = (rect.right() - Self::FIELD_PADDING - x).max(0.0);
        let folder = Draw::text(
            painter,
            pos2(x, rect.center().y),
            Align2::LEFT_CENTER,
            &self.view.folder,
            caption,
            dim,
            TextFit::Truncate(room / 2.0),
        );
        let ink = <color::Text as ColorToken>::VALUE;
        let room = (rect.right() - Self::FIELD_PADDING - folder.right()).max(0.0);
        Draw::text(
            painter,
            pos2(folder.right(), rect.center().y),
            Align2::LEFT_CENTER,
            &self.view.file,
            caption,
            ink,
            TextFit::Truncate(room),
        );
        response.clicked()
    }

    // the note under the settings, in a panel
    fn panel(&self, ui: &mut Ui) {
        let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), Self::PANEL), Sense::hover());
        let painter = ui.painter();
        Draw::fill(painter, rect, Corner::Panel, <color::BgPanel as ColorToken>::VALUE);
        Draw::stroke(painter, rect, Corner::Panel, <color::Line as ColorToken>::VALUE);
        let style = TextStyle::new(TypeSize::Note, FontFace::Regular);
        let dim = <color::TextDim as ColorToken>::VALUE;
        let room = (rect.width() - Self::PANEL_PADDING * 2.0).max(0.0);
        let at = pos2(rect.left() + Self::PANEL_PADDING, rect.center().y);
        Draw::text(painter, at, Align2::LEFT_CENTER, &self.view.note, style, dim, TextFit::Truncate(room));
    }

    // Render now, Add to queue, then Open queue (the footer runs right to left)
    fn footer(ui: &mut Ui) -> Option<ExportDialogAction> {
        let render = TextButton::new(ButtonKind::Primary, Text::message(StudioUiMessage::RenderNow))
            .leading(Leading::Icon(Icon::Video))
            .show(ui)
            .clicked();
        let add = TextButton::new(ButtonKind::Outline, Text::message(StudioUiMessage::AddToQueue))
            .leading(Leading::Icon(Icon::Add))
            .show(ui)
            .clicked();
        let open = TextButton::new(ButtonKind::Ghost, Text::message(StudioUiMessage::OpenQueue)).show(ui).clicked();
        if render {
            Some(ExportDialogAction::RenderNow)
        } else if add {
            Some(ExportDialogAction::AddToQueue)
        } else {
            open.then_some(ExportDialogAction::OpenQueue)
        }
    }

    fn action(response: ModalResponse, action: Option<ExportDialogAction>) -> Option<ExportDialogAction> {
        if response.closed { Some(ExportDialogAction::Cancel) } else { action }
    }
}
