use amm_core::services::settings::ColorCodec;
use amm_core::structs::descriptor::{ApplyMode, ColorEncoding, EnumOption, FieldKind, SettingField, SettingsSection};
use amm_core::structs::registry::{ModIndex, RegisteredMod};
use std::sync::Arc;

use egui::{Align, Align2, Color32, Galley, Layout, Painter, Rect, Sense, Ui, UiBuilder, pos2, vec2};
use studio_ui::theme::{Icon, Metrics, StudioPalette, Typeface};
use studio_ui::widgets::{Chip, ChipStyle, Draw, Dropdown, Slider, Toggle};

use crate::model::{PanelAction, PanelIntent, PanelModel};
use crate::views::ValueText;

// The laid-out label and help of a row, and the row height that fits them.
type RowText = (Arc<Galley>, Arc<Galley>, f32);

/// The General tab: every schema section with its non-chord fields, each with its control, its
/// apply mode and the value a game change wrote while the operator's edit was pending. Labels and
/// help wrap inside the text column left of the control, and a row grows to fit them.
pub struct GeneralTab;

impl GeneralTab {
    const INSET: f32 = 18.0;
    const PAD: f32 = 10.0;
    const TEXT_GAP: f32 = 4.0;
    const COLOUR_WIDTH: f32 = 140.0;
    const SLIDER_WIDTH: f32 = 200.0;
    const VALUE_BOX: (f32, f32) = (84.0, Chip::KEY_HEIGHT + 4.0);
    const VALUE_PAD: f32 = 10.0;
    const CONTROL_WIDTH: f32 = 200.0;

    pub fn show(ui: &mut Ui, model: &PanelModel, registered: &RegisteredMod, out: &mut Vec<PanelIntent>) {
        let sections: Vec<&SettingsSection> =
            registered.descriptor.settings.iter().filter(|section| Self::fields(section).next().is_some()).collect();
        if sections.is_empty() {
            Self::empty(ui);
            return;
        }
        for (position, section) in sections.iter().enumerate() {
            if position > 0 {
                ui.add_space(Metrics::GAP_L + 4.0);
            }
            Self::section(ui, model, registered.index, section, out);
        }
    }

    fn fields(section: &SettingsSection) -> impl Iterator<Item = &SettingField> {
        section.fields.iter().filter(|field| !matches!(field.kind, FieldKind::Chord))
    }

    fn empty(ui: &mut Ui) {
        let width = ui.available_width() - 8.0;
        let text = Draw::line_height(ui.painter(), &Typeface::body_strong())
            + Draw::line_height(ui.painter(), &Typeface::help());
        let (frame, _) = ui.allocate_exact_size(vec2(width, text + 2.0 * Metrics::GAP_L), Sense::hover());
        let painter = ui.painter();
        Draw::fill(painter, frame, Metrics::RADIUS_PANEL - 2, StudioPalette::BG_PANEL);
        Draw::outline(painter, frame, Metrics::RADIUS_PANEL - 2, 1.0, StudioPalette::LINE);
        Draw::icon(
            painter,
            pos2(frame.left() + 30.0, frame.center().y),
            Icon::INFO,
            Typeface::ICON_BADGE,
            StudioPalette::TEXT_DIM,
        );
        let x = frame.left() + 56.0;
        Draw::text(
            painter,
            pos2(x, frame.center().y - 2.0),
            Align2::LEFT_BOTTOM,
            "No settings",
            Typeface::body_strong(),
            StudioPalette::TEXT,
        );
        let help = "This mod has only keybinds. Change them on the Keybinds tab.";
        Draw::text(
            painter,
            pos2(x, frame.center().y + 2.0),
            Align2::LEFT_TOP,
            help,
            Typeface::help(),
            StudioPalette::TEXT_DIM,
        );
    }

    fn section(
        ui: &mut Ui,
        model: &PanelModel,
        index: ModIndex,
        section: &SettingsSection,
        out: &mut Vec<PanelIntent>,
    ) {
        let left = ui.cursor().left();
        let caps = Draw::caps(
            ui.painter(),
            pos2(left + 2.0, ui.cursor().top()),
            Align2::LEFT_TOP,
            &section.title,
            StudioPalette::TEXT_FAINT,
        );
        ui.add_space(caps.height() + Metrics::GAP_S);
        let draft = model.draft(index);
        let width = ui.available_width() - 8.0;
        let painter = ui.painter().clone();
        let rows: Vec<(&SettingField, RowText)> = Self::fields(section)
            .map(|field| {
                let noted = draft.is_some_and(|draft| draft.game_change_note(&field.path).is_some());
                (field, Self::measure(&painter, field, width, noted))
            })
            .collect();
        let total = rows.iter().map(|(_, text)| text.2).sum();
        let (frame, _) = ui.allocate_exact_size(vec2(width, total), Sense::hover());
        Draw::fill(&painter, frame, Metrics::RADIUS_PANEL - 2, StudioPalette::BG_PANEL);
        Draw::outline(&painter, frame, Metrics::RADIUS_PANEL - 2, 1.0, StudioPalette::LINE);
        let mut top = frame.top();
        for (position, (field, text)) in rows.into_iter().enumerate() {
            let row = Rect::from_min_size(pos2(frame.left(), top), vec2(width, text.2));
            if position > 0 {
                Draw::hline(&painter, row.left() + 16.0, row.right() - 16.0, row.top(), StudioPalette::LINE);
            }
            top += text.2;
            Self::row(ui, model, index, field, row, text, out);
        }
    }

    // The label and help of a row laid out inside the text column, and the row height that fits them.
    fn measure(painter: &Painter, field: &SettingField, width: f32, noted: bool) -> RowText {
        let text_width = Self::text_width(painter, field, width);
        let label_width = match Self::apply_note(field.apply) {
            Some(note) => text_width - Chip::width(painter, note, ChipStyle::NEUTRAL.with_icon(Icon::INFO)) - 10.0,
            None => text_width,
        };
        let label = painter.layout(field.label.clone(), Typeface::body(), StudioPalette::TEXT, label_width.max(1.0));
        let help = painter.layout(field.help.clone(), Typeface::help(), StudioPalette::TEXT_DIM, text_width.max(1.0));
        let text = Self::PAD + label.size().y + Self::TEXT_GAP + help.size().y + Self::PAD;
        let note = if noted { Self::note_height(painter) } else { 0.0 };
        (label, help, text + note)
    }

    // the line under the help that shows a value the game wrote
    fn note_height(painter: &Painter) -> f32 {
        Draw::line_height(painter, &Typeface::help()) + Self::TEXT_GAP
    }

    // The width left for the label and help: the row less its insets, the control and a gap.
    fn text_width(painter: &Painter, field: &SettingField, width: f32) -> f32 {
        width - 2.0 * Self::INSET - Self::control_width(painter, field) - Metrics::GAP_L
    }

    fn control_width(painter: &Painter, field: &SettingField) -> f32 {
        match field.kind {
            FieldKind::Bool => Toggle::SIZE.0,
            FieldKind::Int { .. } | FieldKind::Float { .. } => {
                Self::SLIDER_WIDTH + Metrics::GAP_S + Self::value_width(painter, field)
            }
            FieldKind::Enum { .. } => Self::CONTROL_WIDTH,
            FieldKind::Color { .. } => Self::COLOUR_WIDTH,
            FieldKind::Chord => 0.0,
        }
    }

    fn row(
        ui: &mut Ui,
        model: &PanelModel,
        index: ModIndex,
        field: &SettingField,
        row: Rect,
        (label, help, _): RowText,
        out: &mut Vec<PanelIntent>,
    ) {
        let painter = ui.painter().clone();
        let draft = model.draft(index);
        if draft.is_some_and(|draft| draft.is_edited(&field.path)) {
            let bar = Rect::from_min_size(pos2(row.left(), row.top() + 12.0), vec2(3.0, row.height() - 24.0));
            Draw::fill(&painter, bar, 2, StudioPalette::ACCENT);
        }
        let x = row.left() + Self::INSET;
        let label_pos = pos2(x, row.top() + Self::PAD);
        let first_line = label.rows.first().map_or(label.size().y, |line| line.rect().height());
        let label_right = label_pos.x + label.size().x;
        let label_bottom = label_pos.y + label.size().y;
        painter.galley(label_pos, label, StudioPalette::TEXT);
        if let Some(note) = Self::apply_note(field.apply) {
            let y = label_pos.y + first_line * 0.5;
            Chip::paint(&painter, label_right + 10.0, y, note, ChipStyle::NEUTRAL.with_icon(Icon::INFO));
        }
        let help_pos = pos2(x, label_bottom + Self::TEXT_GAP);
        let help_bottom = help_pos.y + help.size().y;
        painter.galley(help_pos, help, StudioPalette::TEXT_DIM);
        if let Some(value) = draft.and_then(|draft| draft.game_change_note(&field.path)) {
            let text = format!("Changed in game to {}", ValueText::display(field, value));
            let y = help_bottom + Self::note_height(&painter) * 0.5;
            Draw::icon(&painter, pos2(x + 6.0, y), Icon::INFO, Typeface::ICON_SMALL, StudioPalette::WARNING);
            Draw::text(
                &painter,
                pos2(x + 20.0, y),
                Align2::LEFT_CENTER,
                &text,
                Typeface::help(),
                StudioPalette::WARNING,
            );
        }
        let control_left = row.right() - Self::INSET - Self::control_width(&painter, field);
        let control = Rect::from_min_max(pos2(control_left, row.top()), pos2(row.right() - Self::INSET, row.bottom()));
        let ui = &mut ui.new_child(UiBuilder::new().max_rect(control).layout(Layout::right_to_left(Align::Center)));
        ui.spacing_mut().item_spacing.x = Metrics::GAP_S;
        let value = model.field_value(index, field);
        if let Some(next) = Self::control(ui, field, &value) {
            out.push(PanelIntent::Action(PanelAction::Edit {
                mod_index: index,
                path: field.path.clone(),
                value: next,
            }));
        }
    }

    // Draws the control of a field and returns the new value when the operator changed it.
    fn control(ui: &mut Ui, field: &SettingField, value: &hcl::Value) -> Option<hcl::Value> {
        match &field.kind {
            FieldKind::Bool => {
                let mut on = value.as_bool().unwrap_or(false);
                Toggle::show(ui, &mut on).changed().then(|| hcl::Value::from(on))
            }
            FieldKind::Int { min, max, step } => {
                let mut number = value.as_i64().unwrap_or(*min) as f64;
                Self::value_box(ui, field, &ValueText::display(field, value));
                let range = (*min as f64)..=(*max as f64);
                let changed = Slider::new(&mut number, range, *step as f64).show(ui, Self::SLIDER_WIDTH).changed();
                changed.then(|| hcl::Value::from(number.round() as i64))
            }
            FieldKind::Float { min, max, step, unit } => {
                let mut number = value.as_f64().unwrap_or(*min);
                Self::value_box(ui, field, &ValueText::float(number, *step, unit.as_deref()));
                let changed = Slider::new(&mut number, *min..=*max, *step).show(ui, Self::SLIDER_WIDTH).changed();
                changed.then(|| hcl::Value::from(number))
            }
            FieldKind::Enum { options } => Self::choice(ui, options, value),
            FieldKind::Color { alpha, encoding } => Self::colour(ui, *alpha, *encoding, value),
            FieldKind::Chord => None,
        }
    }

    fn choice(ui: &mut Ui, options: &[EnumOption], value: &hcl::Value) -> Option<hcl::Value> {
        let labels: Vec<&str> = options.iter().map(|option| option.label.as_str()).collect();
        let mut selected = options.iter().position(|option| option.value == *value).unwrap_or(0);
        let changed = Dropdown::new(&mut selected, &labels).show(ui, Self::CONTROL_WIDTH).changed();
        changed.then(|| options.get(selected).map(|option| option.value.clone())).flatten()
    }

    fn colour(ui: &mut Ui, alpha: bool, encoding: ColorEncoding, value: &hcl::Value) -> Option<hcl::Value> {
        let [r, g, b, a] = ColorCodec::to_rgba(value, encoding).unwrap_or([255, 255, 255, 255]);
        let mut colour = Color32::from_rgba_unmultiplied(r, g, b, if alpha { a } else { u8::MAX });
        ui.spacing_mut().interact_size = vec2(44.0, 24.0);
        let changed = ui.color_edit_button_srgba(&mut colour).changed();
        let hex = if alpha { format!("#{r:02X}{g:02X}{b:02X}{a:02X}") } else { format!("#{r:02X}{g:02X}{b:02X}") };
        ui.label(egui::RichText::new(hex).font(Typeface::mono_text()).color(StudioPalette::TEXT_DIM));
        if !changed {
            return None;
        }
        let [r, g, b, a] = colour.to_srgba_unmultiplied();
        Some(ColorCodec::from_rgba([r, g, b, if alpha { a } else { u8::MAX }], encoding))
    }

    // The value box of a slider field: wide enough for the widest text the field can show, and never
    // narrower than the standard box.
    fn value_width(painter: &Painter, field: &SettingField) -> f32 {
        let widest = ValueText::bounds(field).map_or(0.0, |texts| {
            texts
                .iter()
                .map(|text| Draw::galley(painter, text, Typeface::mono_text(), StudioPalette::TEXT).size().x)
                .fold(0.0, f32::max)
        });
        (widest + 2.0 * Self::VALUE_PAD).max(Self::VALUE_BOX.0)
    }

    fn value_box(ui: &mut Ui, field: &SettingField, text: &str) {
        let width = Self::value_width(ui.painter(), field);
        let (rect, _) = ui.allocate_exact_size(vec2(width, Self::VALUE_BOX.1), Sense::hover());
        let painter = ui.painter();
        Draw::fill(painter, rect, Metrics::RADIUS_CONTROL, StudioPalette::BG_FIELD);
        Draw::outline(painter, rect, Metrics::RADIUS_CONTROL, 1.0, StudioPalette::LINE_STRONG);
        Draw::text(painter, rect.center(), Align2::CENTER_CENTER, text, Typeface::mono_text(), StudioPalette::TEXT);
    }

    fn apply_note(apply: ApplyMode) -> Option<&'static str> {
        match apply {
            ApplyMode::Live => None,
            ApplyMode::WorldReload => Some("Applies after world reload"),
            ApplyMode::Restart => Some("Applies after restart"),
        }
    }
}
