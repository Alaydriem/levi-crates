use amm_core::config::{ExclusiveResolution, PairKey, SuspensionResolution};
use amm_core::services::settings::ChordText;
use amm_core::structs::clash::{Clash, ClashKind};
use amm_core::structs::descriptor::Capability;
use amm_core::structs::registry::ModIndex;
use egui::{Align, Align2, Layout, Rect, ScrollArea, Sense, Ui, UiBuilder, pos2, vec2};
use studio_ui::theme::{Icon, Metrics, StudioPalette, Typeface};
use studio_ui::widgets::{Chip, ChipStyle, Draw, Dropdown, TextButton};

use crate::model::{CapabilityText, PanelAction, PanelIntent, PanelModel, PanelTab};
use crate::views::PageHeader;

/// The Conflicts page: exclusive and key clashes under a badged "Conflicts" group, suspensions under
/// an unbadged "Interactions" group, one card each with its resolution.
pub struct ConflictsView;

impl ConflictsView {
    const CARD: f32 = 132.0;
    // the centre of the kind chip and title line, from the top of the card
    const TITLE_Y: f32 = 26.0;
    const DETAIL_TOP: f32 = Self::TITLE_Y + Chip::HEIGHT * 0.5 + Metrics::GAP_M;
    // from the bottom of the card to the bottom of the mod chips
    const MODS_BOTTOM: f32 = 12.0;
    // the space under the detail text for the mod chips
    const MODS_BAND: f32 = Metrics::GAP_M + Chip::HEIGHT + Self::MODS_BOTTOM;
    const CONTROL_W: f32 = 250.0;

    pub fn show(ui: &mut Ui, model: &PanelModel, out: &mut Vec<PanelIntent>) {
        let subtitle =
            format!("Found from the capability and keybind declarations of {} installed mods", model.mod_count());
        PageHeader::show(ui, Icon::WARNING, StudioPalette::WARNING, "Conflicts", &subtitle);
        ui.add_space(Metrics::GAP_L);
        let report = model.clash_report();
        ScrollArea::vertical().id_salt("amm.conflicts").auto_shrink([false, false]).show(ui, |ui| {
            Self::group_title(ui, "Conflicts", Some(report.conflict_count()));
            if report.conflicts.is_empty() {
                Self::quiet(ui, "No conflicts. The declarations of the installed mods do not clash.");
            }
            for (position, clash) in report.conflicts.iter().enumerate() {
                Self::card(ui, model, clash, position, out);
                ui.add_space(Metrics::GAP_S + 2.0);
            }
            if report.truncated {
                Self::quiet(ui, "More key clashes exist than the panel lists. Rebind keys to see the rest.");
            }
            ui.add_space(Metrics::GAP_L);
            Self::group_title(ui, "Interactions", None);
            if report.interactions.is_empty() {
                Self::quiet(ui, "No interactions between the installed mods.");
            }
            for (position, clash) in report.interactions.iter().enumerate() {
                Self::card(ui, model, clash, report.conflicts.len() + position, out);
                ui.add_space(Metrics::GAP_S + 2.0);
            }
        });
    }

    fn group_title(ui: &mut Ui, title: &str, badge: Option<usize>) {
        let height = Draw::line_height(ui.painter(), &Typeface::section()).max(Chip::HEIGHT) + 4.0;
        let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
        let painter = ui.painter();
        let caps = Draw::caps(
            painter,
            pos2(rect.left() + 2.0, rect.center().y),
            Align2::LEFT_CENTER,
            title,
            StudioPalette::TEXT_FAINT,
        );
        if let Some(count) = badge.filter(|count| *count > 0) {
            Chip::paint(
                painter,
                caps.right() + 8.0,
                rect.center().y,
                &count.to_string(),
                ChipStyle::WARNING.with_icon(Icon::WARNING),
            );
        }
        ui.add_space(4.0);
    }

    fn quiet(ui: &mut Ui, text: &str) {
        let width = ui.available_width() - 8.0;
        let height = Draw::line_height(ui.painter(), &Typeface::help()) + 2.0 * Metrics::GAP_M;
        let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        let painter = ui.painter();
        Draw::fill(painter, rect, Metrics::RADIUS_PANEL - 2, StudioPalette::BG_PANEL);
        Draw::outline(painter, rect, Metrics::RADIUS_PANEL - 2, 1.0, StudioPalette::LINE);
        Draw::icon(
            painter,
            pos2(rect.left() + 22.0, rect.center().y),
            Icon::CHECK,
            Typeface::ICON_SIZE,
            StudioPalette::SUCCESS,
        );
        Draw::text(
            painter,
            pos2(rect.left() + 42.0, rect.center().y),
            Align2::LEFT_CENTER,
            text,
            Typeface::help(),
            StudioPalette::TEXT_DIM,
        );
        ui.add_space(Metrics::GAP_S);
    }

    fn card(ui: &mut Ui, model: &PanelModel, clash: &Clash, position: usize, out: &mut Vec<PanelIntent>) {
        let width = ui.available_width() - 8.0;
        let wrap = width - Self::CONTROL_W - 60.0;
        let detail =
            ui.painter().layout(Self::detail(model, &clash.kind), Typeface::help(), StudioPalette::TEXT_DIM, wrap);
        let height = (Self::DETAIL_TOP + detail.size().y + Self::MODS_BAND).max(Self::CARD);
        let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        let painter = ui.painter().clone();
        Draw::fill(&painter, rect, Metrics::RADIUS_PANEL - 2, StudioPalette::BG_PANEL);
        Draw::outline(&painter, rect, Metrics::RADIUS_PANEL - 2, 1.0, StudioPalette::LINE);
        let is_conflict = !matches!(clash.kind, ClashKind::Suspension { .. });
        let open = is_conflict && !clash.reviewed;
        let bar_colour = if open { StudioPalette::WARNING } else { StudioPalette::LINE_STRONG };
        Draw::fill(
            &painter,
            Rect::from_min_size(pos2(rect.left(), rect.top() + 12.0), vec2(3.0, rect.height() - 24.0)),
            2,
            bar_colour,
        );
        let x = rect.left() + 20.0;
        let (kind_label, kind_style) = Self::kind(&clash.kind);
        let kind = Chip::paint(&painter, x, rect.top() + Self::TITLE_Y, kind_label, kind_style);
        let title = Draw::text(
            &painter,
            pos2(kind.right() + 10.0, kind.center().y),
            Align2::LEFT_CENTER,
            &Self::title(model, &clash.kind),
            Typeface::body_strong(),
            StudioPalette::TEXT,
        );
        if is_conflict {
            let (label, style) = if clash.reviewed {
                ("Resolved", ChipStyle::SUCCESS.with_icon(Icon::CHECK))
            } else {
                ("Needs review", ChipStyle::WARNING.with_icon(Icon::WARNING))
            };
            Chip::paint(&painter, title.right() + 10.0, kind.center().y, label, style);
        }
        painter.galley(pos2(x, rect.top() + Self::DETAIL_TOP), detail, StudioPalette::TEXT_DIM);
        let mods_y = rect.bottom() - Self::MODS_BOTTOM - Chip::HEIGHT * 0.5;
        Draw::text(&painter, pos2(x, mods_y), Align2::LEFT_CENTER, "Mods", Typeface::note(), StudioPalette::TEXT_FAINT);
        let mut mx = x + 52.0;
        let (a, b) = Self::mods(&clash.kind);
        for index in [a, b] {
            mx = Chip::paint(&painter, mx, mods_y, model.display_name(index), ChipStyle::NEUTRAL).right() + 4.0;
        }
        let control = Rect::from_min_max(
            pos2(rect.right() - 18.0 - Self::CONTROL_W, rect.top() + 18.0),
            pos2(rect.right() - 18.0, rect.bottom() - 14.0),
        );
        let caps = Draw::caps(
            &painter,
            pos2(control.left(), control.top() + 6.0),
            Align2::LEFT_TOP,
            "Resolution",
            StudioPalette::TEXT_FAINT,
        );
        let area = Rect::from_min_max(pos2(control.left(), caps.bottom() + Metrics::GAP_S), control.max);
        let ui = &mut ui.new_child(UiBuilder::new().max_rect(area).layout(Layout::top_down(Align::Min)));
        ui.push_id(("amm.resolution", position), |ui| Self::resolution(ui, model, &clash.kind, out));
    }

    fn resolution(ui: &mut Ui, model: &PanelModel, kind: &ClashKind, out: &mut Vec<PanelIntent>) {
        let book = &model.config().resolutions;
        match *kind {
            ClashKind::Exclusive { capability, a, b } => {
                let Some((a_id, b_id)) = Self::ids(model, a, b) else { return };
                let key = PairKey::new(capability, a_id, b_id);
                let options = [
                    "Deny the second request".to_string(),
                    format!("Prefer {}", model.display_name(a)),
                    format!("Prefer {}", model.display_name(b)),
                ];
                let labels: Vec<&str> = options.iter().map(String::as_str).collect();
                let mut selected = match book.exclusive_for(&key) {
                    Some(ExclusiveResolution::Prefer(id)) if id == a_id => 1,
                    Some(ExclusiveResolution::Prefer(_)) => 2,
                    Some(ExclusiveResolution::DenySecond) | None => 0,
                };
                if Dropdown::new(&mut selected, &labels).show(ui, Self::CONTROL_W).changed() {
                    let resolution = match selected {
                        1 => ExclusiveResolution::Prefer(a_id.to_string()),
                        2 => ExclusiveResolution::Prefer(b_id.to_string()),
                        _ => ExclusiveResolution::DenySecond,
                    };
                    out.push(PanelIntent::Action(PanelAction::SetExclusive(key, resolution)));
                }
            }
            ClashKind::Suspension { capability, exclusive, shared } => {
                let Some((a_id, b_id)) = Self::ids(model, exclusive, shared) else { return };
                let key = PairKey::new(capability, a_id, b_id);
                let options = [format!("Suspend {}", model.display_name(shared)), "Coexist".to_string()];
                let labels: Vec<&str> = options.iter().map(String::as_str).collect();
                let mut selected = usize::from(book.suspension_for(&key) == Some(&SuspensionResolution::Coexist));
                if Dropdown::new(&mut selected, &labels).show(ui, Self::CONTROL_W).changed() {
                    let resolution =
                        if selected == 1 { SuspensionResolution::Coexist } else { SuspensionResolution::Suspend };
                    out.push(PanelIntent::Action(PanelAction::SetSuspension(key, resolution)));
                }
            }
            ClashKind::Key { a, .. } => {
                ui.label(
                    egui::RichText::new("Rebind one of the keys").font(Typeface::help()).color(StudioPalette::TEXT_DIM),
                );
                if TextButton::secondary(ui, "Open keybinds").clicked() {
                    out.push(PanelIntent::Action(PanelAction::Select(Some(a.mod_index))));
                    out.push(PanelIntent::Action(PanelAction::SelectTab(PanelTab::Keybinds)));
                }
            }
        }
    }

    fn ids<'a>(model: &PanelModel<'a>, a: ModIndex, b: ModIndex) -> Option<(&'a str, &'a str)> {
        Some((model.registered(a)?.descriptor.id.as_str(), model.registered(b)?.descriptor.id.as_str()))
    }

    fn mods(kind: &ClashKind) -> (ModIndex, ModIndex) {
        match *kind {
            ClashKind::Exclusive { a, b, .. } => (a, b),
            ClashKind::Suspension { exclusive, shared, .. } => (exclusive, shared),
            ClashKind::Key { a, b, .. } => (a.mod_index, b.mod_index),
        }
    }

    fn kind(kind: &ClashKind) -> (&'static str, ChipStyle) {
        match kind {
            ClashKind::Exclusive { .. } => ("Exclusive", ChipStyle::WARNING.with_icon(Icon::LOCK)),
            ClashKind::Suspension { .. } => ("Suspension", ChipStyle::ACCENT.with_icon(Icon::LOCK)),
            ClashKind::Key { .. } => ("Keybind", ChipStyle::DANGER.with_icon(Icon::KEYBOARD)),
        }
    }

    fn title(model: &PanelModel, kind: &ClashKind) -> String {
        match *kind {
            ClashKind::Exclusive { capability, a, b } => {
                format!("{}: {} and {}", CapabilityText::name(capability), model.display_name(a), model.display_name(b))
            }
            ClashKind::Suspension { capability, exclusive, shared } => format!(
                "{}: {} and {}",
                CapabilityText::name(capability),
                model.display_name(exclusive),
                model.display_name(shared)
            ),
            ClashKind::Key { chord, .. } => format!("{} is bound twice", ChordText::format(chord)),
        }
    }

    fn detail(model: &PanelModel, kind: &ClashKind) -> String {
        match *kind {
            ClashKind::Exclusive { capability, .. } => format!(
                "Both mods claim {} as exclusive. Only one of them can hold it at a time.",
                CapabilityText::name(capability)
            ),
            ClashKind::Suspension { capability, exclusive, shared } => {
                Self::suspension_detail(model, capability, exclusive, shared)
            }
            ClashKind::Key { a, b, .. } => {
                let label = |bind: amm_core::structs::clash::BindRef| {
                    model.keybind(bind.mod_index, bind.action).map_or("a keybind", |decl| decl.label.as_str())
                };
                format!(
                    "{} in {} and {} in {} use the same chord. Both can be live at the same time.",
                    label(a),
                    model.display_name(a.mod_index),
                    label(b),
                    model.display_name(b.mod_index)
                )
            }
        }
    }

    fn suspension_detail(model: &PanelModel, capability: Capability, exclusive: ModIndex, shared: ModIndex) -> String {
        let name = CapabilityText::name(capability);
        format!(
            "{} claims {name} as exclusive. While it holds {name}, {} stops its shared use and its {name} keys are blocked.",
            model.display_name(exclusive),
            model.display_name(shared)
        )
    }
}
