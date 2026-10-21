use amm_core::structs::registry::{ModIndex, RegisteredMod};
use egui::{Align, Align2, Layout, Rect, RichText, ScrollArea, Sense, Ui, UiBuilder, pos2, vec2};
use studio_ui::theme::{Icon, Metrics, StudioPalette, Typeface};
use studio_ui::widgets::{Chip, ChipStyle, Draw, Toggle};

use crate::model::{PanelAction, PanelController, PanelIntent, PanelModel, PanelTab};
use crate::views::{Avatar, CapabilitiesTab, GeneralTab, KeybindsTab, PageHeader};

/// The right pane for one mod: its header, the tabs and the tab content.
pub struct ModDetailView;

impl ModDetailView {
    const TAB_HEIGHT: f32 = 42.0;
    const TABS: [(PanelTab, &'static str, char); 3] = [
        (PanelTab::General, "General", Icon::SETTINGS),
        (PanelTab::Keybinds, "Keybinds", Icon::KEYBOARD),
        (PanelTab::Capabilities, "Capabilities", Icon::LOCK),
    ];

    pub fn show(
        ui: &mut Ui,
        model: &PanelModel,
        controller: &PanelController,
        index: ModIndex,
        out: &mut Vec<PanelIntent>,
    ) {
        let Some(registered) = model.registered(index) else {
            return;
        };
        Self::header(ui, model, registered, out);
        ui.add_space(Metrics::GAP_L);
        let tab = controller.tab();
        Self::tabs(ui, tab, out);
        ui.add_space(Metrics::GAP_L);
        ScrollArea::vertical().id_salt(("amm.pane", index.0, tab as u8)).auto_shrink([false, false]).show(ui, |ui| {
            match tab {
                PanelTab::General => GeneralTab::show(ui, model, registered, out),
                PanelTab::Keybinds => KeybindsTab::show(ui, model, controller, registered, out),
                PanelTab::Capabilities => CapabilitiesTab::show(ui, model, registered),
            }
        });
    }

    fn header(ui: &mut Ui, model: &PanelModel, registered: &RegisteredMod, out: &mut Vec<PanelIntent>) {
        let index = registered.index;
        let descriptor = &registered.descriptor;
        let height = PageHeader::height(ui.painter());
        let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
        let painter = ui.painter().clone();
        let avatar = Rect::from_min_size(rect.min, vec2(height, height));
        Avatar::paint(&painter, avatar, index, &descriptor.display_name, Metrics::RADIUS_PANEL, Typeface::HEADING_SIZE);
        let x = avatar.right() + Metrics::GAP_M + 2.0;
        Draw::text(
            &painter,
            pos2(x, rect.top() + 1.0),
            Align2::LEFT_TOP,
            &descriptor.display_name,
            Typeface::heading(),
            StudioPalette::TEXT,
        );
        let version = format!("v{}", descriptor.version);
        let author = format!("by {}", descriptor.author);
        let dot = "  \u{b7}  ";
        let meta = Draw::runs(
            &painter,
            &[
                (&descriptor.id, Typeface::mono_text(), StudioPalette::TEXT_DIM),
                (dot, Typeface::help(), StudioPalette::TEXT_FAINT),
                (&version, Typeface::mono_text(), StudioPalette::TEXT_DIM),
                (dot, Typeface::help(), StudioPalette::TEXT_FAINT),
                (&author, Typeface::help(), StudioPalette::TEXT_DIM),
            ],
        );
        Draw::place(&painter, meta, pos2(x, rect.bottom() - 2.0), Align2::LEFT_BOTTOM);
        let badge = model.clash_report().badge_count(index);
        let right = Rect::from_min_max(pos2(rect.center().x, rect.top()), rect.max);
        ui.scope_builder(UiBuilder::new().max_rect(right).layout(Layout::right_to_left(Align::Center)), |ui| {
            ui.spacing_mut().item_spacing.x = Metrics::GAP_S;
            let mut enabled = registered.enabled;
            if Toggle::show(ui, &mut enabled).changed() {
                out.push(PanelIntent::Action(PanelAction::SetEnabled(index, enabled)));
            }
            let (state, colour) = if registered.enabled {
                ("Enabled", StudioPalette::TEXT)
            } else {
                ("Disabled", StudioPalette::TEXT_DIM)
            };
            ui.label(RichText::new(state).font(Typeface::body()).color(colour));
            if badge > 0 {
                ui.add_space(Metrics::GAP_L);
                let label = format!("{badge} conflict{}  \u{203a}", if badge == 1 { "" } else { "s" });
                let style = ChipStyle::WARNING.with_icon(Icon::WARNING);
                let width = Chip::width(ui.painter(), &label, style);
                let (chip, response) = ui.allocate_exact_size(vec2(width, Chip::HEIGHT + 4.0), Sense::click());
                Chip::paint(ui.painter(), chip.left(), chip.center().y, &label, style);
                if response.clicked() {
                    out.push(PanelIntent::Action(PanelAction::Select(None)));
                }
            }
        });
    }

    fn tabs(ui: &mut Ui, current: PanelTab, out: &mut Vec<PanelIntent>) {
        let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), Self::TAB_HEIGHT), Sense::hover());
        Draw::hline(ui.painter(), rect.left(), rect.right(), rect.bottom() - 1.0, StudioPalette::LINE_STRONG);
        let mut x = rect.left();
        for (value, label, icon) in Self::TABS {
            let galley = Draw::galley(ui.painter(), label, Typeface::label_strong(), StudioPalette::TEXT);
            let width = galley.size().x + 54.0;
            let cell = Rect::from_min_size(pos2(x, rect.top()), vec2(width, rect.height()));
            let response = ui.interact(cell, ui.scope_id().with(("amm.tab", label)), Sense::click());
            let active = current == value;
            let colour = if active || response.hovered() { StudioPalette::TEXT } else { StudioPalette::TEXT_DIM };
            let painter = ui.painter();
            if response.hovered() && !active {
                let hover = cell.shrink2(vec2(0.0, 4.0)).translate(vec2(0.0, -2.0));
                Draw::fill(painter, hover, Metrics::RADIUS_CONTROL, StudioPalette::BG_HOVER);
            }
            let icon_colour = if active { StudioPalette::ACCENT_HOVER } else { colour };
            Draw::icon(
                painter,
                pos2(cell.left() + 21.0, cell.center().y - 1.0),
                icon,
                Typeface::ICON_SIZE,
                icon_colour,
            );
            Draw::text(
                painter,
                pos2(cell.left() + 36.0, cell.center().y - 1.0),
                Align2::LEFT_CENTER,
                label,
                Typeface::label_strong(),
                colour,
            );
            if active {
                let bar = Rect::from_min_max(
                    pos2(cell.left() + 8.0, rect.bottom() - 3.0),
                    pos2(cell.right() - 8.0, rect.bottom()),
                );
                Draw::fill(painter, bar, 2, StudioPalette::ACCENT);
            }
            if response.clicked() && !active {
                out.push(PanelIntent::Action(PanelAction::SelectTab(value)));
            }
            x += width + 4.0;
        }
    }
}
