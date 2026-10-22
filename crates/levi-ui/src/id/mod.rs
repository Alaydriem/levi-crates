/// The egui ids levi-ui keeps for itself, or adds under a caller's id. Widgets that keep state take
/// their base id from the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LeviUiId {
    /// The localizer the widgets resolve `Text` with.
    Locale,
    /// A segmented control's selection fill, as it slides.
    SegmentSlide,
    /// A tab row underline's two edges, as they slide.
    UnderlineLeft,
    UnderlineRight,
    /// A field's box, for its hover.
    FieldBox,
    /// A text field's value when it took the focus, restored on Escape.
    FieldStart,
    /// A text field's clear button.
    FieldClear,
    /// The typed text of a drag or colour field, and its text edit.
    InlineBuffer,
    InlineEdit,
    /// Set when typing starts; the edit takes the focus on its first frame.
    InlineFocus,
    /// Why a drag or colour field's last typed value was refused.
    FieldError,
    /// The option the keyboard has reached in an open dropdown list.
    DropdownHighlight,
}
