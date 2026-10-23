mod rename_action;

pub use rename_action::RenameAction;

use std::cell::RefCell;

use egui::{Context, Id, Key, Ui};
use levi_i18n::text::Text;
use levi_ui::state::DisabledReason;
use levi_ui::widget::{ButtonKind, ModalFrame, ModalResponse, TextButton, TextField, TextFieldKind};

use crate::id::StudioUiId;
use crate::message::StudioUiMessage;

/// Paper: Menus + Dialogs, `Rename (scene, bookmark, preset)` (`9QD-0`). A name field in the levi-ui `ModalFrame`,
/// focused when it opens. Enter or Rename applies a name that is not empty; Escape cancels. The caller keeps the
/// text in `value`.
pub struct RenameDialog<'a> {
    title: &'a Text,
    value: &'a mut String,
    field: Id,
}

impl<'a> RenameDialog<'a> {
    /// Paper: the rename cards are 460 wide.
    const WIDTH: f32 = 460.0;

    pub fn new(title: &'a Text, value: &'a mut String) -> Self {
        Self { title, value, field: Id::unique(StudioUiId::RenameField) }
    }

    /// Another id for the field, for a second dialog on one screen (the gallery shows three).
    pub fn id(mut self, id: Id) -> Self {
        self.field = id;
        self
    }

    pub fn show(self, ctx: &Context) -> Option<RenameAction> {
        let frame = self.frame();
        let field = self.field;
        // the body edits the text and the footer reads it, one after the other
        let value = RefCell::new(self.value);
        let (mut body, mut footer) = (None, None);
        let response = frame.show(
            ctx,
            |ui| body = Self::body(ui, field, &mut value.borrow_mut()),
            |ui| footer = Self::footer(ui, &value.borrow()),
        );
        Self::action(response, body.or(footer))
    }

    /// Shows the card where `ui` places it, for the gallery.
    pub fn show_card(self, ui: &mut Ui) -> Option<RenameAction> {
        let frame = self.frame();
        let field = self.field;
        let value = RefCell::new(self.value);
        let (mut body, mut footer) = (None, None);
        let response = frame.show_card(
            ui,
            |ui| body = Self::body(ui, field, &mut value.borrow_mut()),
            |ui| footer = Self::footer(ui, &value.borrow()),
        );
        Self::action(response, body.or(footer))
    }

    fn frame(&self) -> ModalFrame {
        ModalFrame::new(Id::unique(StudioUiId::RenameDialog), self.title.clone())
            .subtitle(Text::message(StudioUiMessage::RenameHint))
            .width(Self::WIDTH)
    }

    // the field, focused when nothing else has the focus; Enter applies and Escape cancels
    fn body(ui: &mut Ui, field: Id, value: &mut String) -> Option<RenameAction> {
        if ui.memory(|memory| memory.focused().is_none()) {
            ui.memory_mut(|memory| memory.request_focus(field));
        }
        let placeholder = Text::message(StudioUiMessage::RenamePlaceholder);
        let response = TextField::new(field, value, TextFieldKind::Plain, placeholder).show(ui, ui.available_width());
        if !response.lost_focus() {
            return None;
        }
        if ui.input(|input| input.key_pressed(Key::Escape)) {
            Some(RenameAction::Cancel)
        } else if ui.input(|input| input.key_pressed(Key::Enter)) && !value.trim().is_empty() {
            Some(RenameAction::Confirm(value.trim().to_owned()))
        } else {
            None
        }
    }

    fn footer(ui: &mut Ui, value: &str) -> Option<RenameAction> {
        let empty = value.trim().is_empty();
        let reason = DisabledReason::message(StudioUiMessage::NameIsEmpty);
        let button =
            TextButton::new(ButtonKind::Primary, Text::message(StudioUiMessage::Rename)).enabled(!empty, reason);
        (button.show(ui).clicked() && !empty).then(|| RenameAction::Confirm(value.trim().to_owned()))
    }

    fn action(response: ModalResponse, action: Option<RenameAction>) -> Option<RenameAction> {
        if response.closed { Some(RenameAction::Cancel) } else { action }
    }
}
