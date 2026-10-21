use egui::{ComboBox, Response, RichText, Ui};

use crate::theme::{StudioPalette, Typeface};

// a single-choice dropdown over a fixed list of labels, in the Studio colours
pub struct Dropdown<'a> {
    selected: &'a mut usize,
    options: &'a [&'a str],
}

impl<'a> Dropdown<'a> {
    pub fn new(selected: &'a mut usize, options: &'a [&'a str]) -> Self {
        Self { selected, options }
    }

    pub fn show(self, ui: &mut Ui, width: f32) -> Response {
        let current = self.options.get(*self.selected).copied().unwrap_or("");
        let id = ui.next_auto_id();
        let text = RichText::new(current).font(Typeface::body()).color(StudioPalette::TEXT);
        let mut changed = false;
        let selected = self.selected;
        let mut outer = ComboBox::from_id_salt(id).width(width).selected_text(text).show_ui(ui, |ui| {
            for (index, label) in self.options.iter().enumerate() {
                if ui.selectable_label(*selected == index, *label).clicked() {
                    *selected = index;
                    changed = true;
                }
            }
        });
        if changed {
            outer.response.mark_changed();
        }
        outer.response
    }
}
