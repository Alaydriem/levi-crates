use amm_core::services::settings::ChordText;
use amm_core::structs::descriptor::{Capability, ClaimMode};
use amm_core::structs::registry::RegisteredMod;
use egui::{Align2, Painter, Rect, ScrollArea, Sense, Ui, UiBuilder, pos2, vec2};
use studio_ui::theme::{Icon, Metrics, StudioPalette, Typeface};
use studio_ui::widgets::{Chip, ChipStyle, Draw, Toggle};

use crate::model::{PanelAction, PanelIntent, PanelModel, PanelPage};
use crate::views::Avatar;

/// The left column: the installed mods, then the Manager group with Conflicts, Keybinds and Manager
/// settings.
pub struct ModListView;

impl ModListView {
    pub const WIDTH: f32 = 340.0;
    const ENTRY: f32 = 46.0;
    const AVATAR: f32 = 34.0;
    // from the top of a row to the top of the mod name
    const NAME_TOP: f32 = 11.0;
    // from the name line to the chip line, and from the chip line to the bottom of the row
    const LINE_GAP: f32 = 4.0;
    const ROW_BOTTOM: f32 = 12.0;
    // the toggle hint at the bottom wraps onto at most this many lines
    const FOOTER_LINES: f32 = 2.0;

    pub fn show(ui: &mut Ui, model: &PanelModel, page: PanelPage, out: &mut Vec<PanelIntent>) {
        let area = ui.max_rect();
        Draw::caps(
            ui.painter(),
            pos2(area.left() + 4.0, area.top()),
            Align2::LEFT_TOP,
            "Installed mods",
            StudioPalette::TEXT_FAINT,
        );
        let caption = Self::caption(ui.painter());
        ui.add_space(caption);
        let row = Self::row_height(ui.painter());
        let list_height = (ui.available_height() - Self::manager_group(ui.painter(), caption)).max(row);
        ScrollArea::vertical().id_salt("amm.mod_list").max_height(list_height).auto_shrink([false, true]).show(
            ui,
            |ui| {
                for registered in model.mods() {
                    Self::row(ui, model, registered, page == PanelPage::Mod(registered.index), out);
                    ui.add_space(4.0);
                }
            },
        );
        ui.add_space(Metrics::GAP_M);
        let painter = ui.painter().clone();
        Draw::hline(&painter, area.left(), area.right(), ui.cursor().top(), StudioPalette::LINE);
        ui.add_space(Metrics::GAP_M);
        Draw::caps(
            &painter,
            pos2(area.left() + 4.0, ui.cursor().top()),
            Align2::LEFT_TOP,
            "Manager",
            StudioPalette::TEXT_FAINT,
        );
        ui.add_space(caption);
        let count = model.clash_report().conflict_count();
        let tint = if count > 0 { StudioPalette::WARNING } else { StudioPalette::TEXT_DIM };
        let label = format!("Conflicts ({count})");
        if Self::entry(ui, Icon::WARNING, tint, &label, page == PanelPage::Conflicts) {
            out.push(PanelIntent::Action(PanelAction::Select(None)));
        }
        ui.add_space(4.0);
        if Self::entry(ui, Icon::KEYBOARD, StudioPalette::TEXT_DIM, "Keybinds", page == PanelPage::Keybinds) {
            out.push(PanelIntent::ShowPage(PanelPage::Keybinds));
        }
        ui.add_space(4.0);
        if Self::entry(
            ui,
            Icon::SETTINGS,
            StudioPalette::TEXT_DIM,
            "Manager settings",
            page == PanelPage::ManagerSettings,
        ) {
            out.push(PanelIntent::ShowPage(PanelPage::ManagerSettings));
        }
        let hint = format!("{} opens and closes this panel", ChordText::format(model.config().toggle_chord));
        let hint = painter.layout(hint, Typeface::help(), StudioPalette::TEXT_FAINT, area.width() - 8.0);
        Draw::place(&painter, hint, pos2(area.left() + 4.0, area.bottom()), Align2::LEFT_BOTTOM);
    }

    // the height of a section caption and the gap under it
    fn caption(painter: &Painter) -> f32 {
        Draw::line_height(painter, &Typeface::section()) + Metrics::GAP_S
    }

    // a mod row: the name line, then the chip line
    fn row_height(painter: &Painter) -> f32 {
        let name = Draw::line_height(painter, &Typeface::body_strong());
        Self::NAME_TOP + name + Self::LINE_GAP + Chip::HEIGHT + Self::ROW_BOTTOM
    }

    // the Manager group below the mods: rule, caption, three entries, footer
    fn manager_group(painter: &Painter, caption: f32) -> f32 {
        let footer = Self::FOOTER_LINES * Draw::line_height(painter, &Typeface::help()) + Metrics::GAP_M;
        2.0 * Metrics::GAP_M + caption + 3.0 * (Self::ENTRY + 4.0) + footer
    }

    pub fn row_fill(painter: &Painter, rect: Rect, selected: bool, hovered: bool) {
        if selected {
            Draw::fill(painter, rect, Metrics::RADIUS_CONTROL + 2, StudioPalette::BG_ACTIVE);
            let bar = Rect::from_min_size(pos2(rect.left(), rect.top() + 14.0), vec2(3.0, rect.height() - 28.0));
            Draw::fill(painter, bar, 2, StudioPalette::ACCENT);
        } else if hovered {
            Draw::fill(painter, rect, Metrics::RADIUS_CONTROL + 2, StudioPalette::BG_HOVER);
        }
    }

    fn row(ui: &mut Ui, model: &PanelModel, registered: &RegisteredMod, selected: bool, out: &mut Vec<PanelIntent>) {
        let index = registered.index;
        let descriptor = &registered.descriptor;
        let height = Self::row_height(ui.painter());
        let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::click());
        let painter = ui.painter().clone();
        Self::row_fill(&painter, rect, selected, response.hovered());
        let avatar = Rect::from_min_size(pos2(rect.left() + 12.0, rect.top() + 12.0), vec2(Self::AVATAR, Self::AVATAR));
        Avatar::paint(
            &painter,
            avatar,
            index,
            &descriptor.display_name,
            Metrics::RADIUS_CONTROL + 2,
            Typeface::BODY_SIZE,
        );
        let x = avatar.right() + Metrics::GAP_M;
        let name_colour = if registered.enabled { StudioPalette::TEXT } else { StudioPalette::TEXT_DIM };
        let name = Draw::text(
            &painter,
            pos2(x, rect.top() + Self::NAME_TOP),
            Align2::LEFT_TOP,
            &descriptor.display_name,
            Typeface::body_strong(),
            name_colour,
        );
        let version = format!("v{}", descriptor.version);
        let ver = Draw::text(
            &painter,
            pos2(name.right() + 8.0, name.center().y + 1.0),
            Align2::LEFT_CENTER,
            &version,
            Typeface::mono_small(),
            StudioPalette::TEXT_FAINT,
        );
        let badge = model.clash_report().badge_count(index);
        if badge > 0 {
            Chip::paint(
                &painter,
                ver.right() + 8.0,
                name.center().y,
                &badge.to_string(),
                ChipStyle::WARNING.with_icon(Icon::WARNING),
            );
        }
        let chip_y = name.bottom() + Self::LINE_GAP + Chip::HEIGHT * 0.5;
        Self::chips(&painter, registered, x, chip_y, rect.right() - 60.0);
        let toggle_rect = Rect::from_center_size(
            pos2(rect.right() - 12.0 - Toggle::SIZE.0 * 0.5, rect.center().y),
            vec2(Toggle::SIZE.0, Toggle::SIZE.1),
        );
        let mut enabled = registered.enabled;
        if Toggle::show(&mut ui.new_child(UiBuilder::new().max_rect(toggle_rect)), &mut enabled).changed() {
            out.push(PanelIntent::Action(PanelAction::SetEnabled(index, enabled)));
        } else if response.clicked() {
            out.push(PanelIntent::Action(PanelAction::Select(Some(index))));
        }
    }

    // The capability chips of a row, a lock on exclusive claims, "+N" when they do not fit.
    fn chips(painter: &Painter, registered: &RegisteredMod, left: f32, y: f32, right: f32) {
        let claims = &registered.descriptor.claims;
        let ordered: Vec<(Capability, ClaimMode)> = Capability::ALL
            .iter()
            .filter_map(|cap| claims.iter().find(|claim| claim.capability == *cap).map(|claim| (*cap, claim.mode)))
            .collect();
        let mut x = left;
        for (position, (cap, mode)) in ordered.iter().enumerate() {
            let label = Self::short(*cap);
            let style = if *mode == ClaimMode::Exclusive {
                ChipStyle::ACCENT.with_icon(Icon::LOCK)
            } else {
                ChipStyle::NEUTRAL
            };
            let remaining = ordered.len() - position;
            let more = format!("+{remaining}");
            let reserve = if remaining > 1 { Chip::width(painter, &more, ChipStyle::NEUTRAL) + 4.0 } else { 0.0 };
            if x + Chip::width(painter, label, style) + reserve > right {
                Chip::paint(painter, x, y, &more, ChipStyle::NEUTRAL);
                return;
            }
            x = Chip::paint(painter, x, y, label, style).right() + 4.0;
        }
    }

    fn short(cap: Capability) -> &'static str {
        match cap {
            Capability::Camera => "Camera",
            Capability::Input => "Input",
            Capability::Hud => "HUD",
            Capability::Overlay => "Overlay",
            Capability::WorldDraw => "World",
        }
    }

    fn entry(ui: &mut Ui, icon: char, tint: egui::Color32, label: &str, selected: bool) -> bool {
        let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), Self::ENTRY), Sense::click());
        let painter = ui.painter();
        Self::row_fill(painter, rect, selected, response.hovered());
        Draw::icon(
            painter,
            pos2(rect.left() + 12.0 + Self::AVATAR * 0.5, rect.center().y),
            icon,
            Typeface::ICON_SIZE,
            tint,
        );
        let x = rect.left() + 12.0 + Self::AVATAR + Metrics::GAP_M;
        Draw::text(
            painter,
            pos2(x, rect.center().y),
            Align2::LEFT_CENTER,
            label,
            Typeface::body_strong(),
            StudioPalette::TEXT,
        );
        response.clicked()
    }
}
