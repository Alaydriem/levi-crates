use egui::{Rect, Sense, Ui, UiBuilder, pos2, vec2};
use studio_ui::theme::{Metrics, StudioPalette};
use studio_ui::widgets::Draw;

use crate::model::{PanelController, PanelIntent, PanelModel, PanelPage};
use crate::views::{
    ConflictsView, CornerRowView, HeaderView, KeybindsPage, ManagerSettingsView, ModDetailView, ModListView, ToastsView,
};

/// One pass of the manager's own UI over the whole screen: the panel when it is open, the toasts,
/// and the menu button while the panel is closed.
pub struct PanelLayout;

impl PanelLayout {
    const PANEL_MAX: (f32, f32) = (1180.0, 720.0);
    const SCREEN_MARGIN: f32 = 32.0;

    /// Returns the rect of the corner row when it drew one, in points.
    pub fn show(
        ui: &mut Ui,
        model: &PanelModel,
        controller: &PanelController,
        out: &mut Vec<PanelIntent>,
    ) -> Option<Rect> {
        let screen = ui.max_rect();
        let mut panel = None;
        if model.panel_open() {
            Draw::fill(ui.painter(), screen, 0, StudioPalette::DIM);
            ui.allocate_rect(screen, Sense::click());
            let rect = Self::panel_rect(screen);
            Self::panel(ui, rect, model, controller, out);
            panel = Some(rect);
        }
        let corner = if model.menu_button_visible() { CornerRowView::show(ui, screen, model, out) } else { None };
        ToastsView::show(ui.ctx(), screen, panel, corner, model);
        corner
    }

    /// The panel's rect on a screen: centred, inside a 32 point margin, at most 1180 by 720.
    pub fn panel_rect(screen: Rect) -> Rect {
        let size = vec2(
            (screen.width() - 2.0 * Self::SCREEN_MARGIN).min(Self::PANEL_MAX.0),
            (screen.height() - 2.0 * Self::SCREEN_MARGIN).min(Self::PANEL_MAX.1),
        );
        Rect::from_center_size(screen.center(), size.max(vec2(0.0, 0.0)))
    }

    /// The page shown: the operator's choice while it is valid, else the first mod, else Conflicts.
    pub fn page(model: &PanelModel, controller: &PanelController) -> PanelPage {
        match controller.page() {
            Some(PanelPage::Mod(index)) if model.registered(index).is_some() => PanelPage::Mod(index),
            Some(PanelPage::Mod(_)) | None => model.first_mod().map_or(PanelPage::Conflicts, PanelPage::Mod),
            Some(page) => page,
        }
    }

    fn panel(ui: &mut Ui, rect: Rect, model: &PanelModel, controller: &PanelController, out: &mut Vec<PanelIntent>) {
        Draw::card(ui.painter(), rect);
        let inner = rect.shrink(Metrics::GAP_L + 4.0);
        let header = Rect::from_min_size(inner.min, vec2(inner.width(), HeaderView::height(ui.painter())));
        HeaderView::show(ui, header, model, out);
        let line_y = header.bottom() + Metrics::GAP_M;
        Draw::hline(ui.painter(), rect.left(), rect.right(), line_y, StudioPalette::LINE_STRONG);
        let body_top = line_y + 1.0 + Metrics::GAP_L;
        let list =
            Rect::from_min_max(pos2(inner.left(), body_top), pos2(inner.left() + ModListView::WIDTH, inner.bottom()));
        let divider_x = list.right() + Metrics::GAP_L;
        let divider = Rect::from_min_max(pos2(divider_x, line_y + 1.0), pos2(divider_x + 1.0, rect.bottom() - 1.0));
        Draw::fill(ui.painter(), divider, 0, StudioPalette::LINE_STRONG);
        let pane = Rect::from_min_max(pos2(divider_x + Metrics::GAP_L + 8.0, body_top), inner.max);
        let page = Self::page(model, controller);
        ui.scope_builder(UiBuilder::new().max_rect(list), |ui| ModListView::show(ui, model, page, out));
        ui.scope_builder(UiBuilder::new().max_rect(pane), |ui| match page {
            PanelPage::Mod(index) => ModDetailView::show(ui, model, controller, index, out),
            PanelPage::Conflicts => ConflictsView::show(ui, model, out),
            PanelPage::Keybinds => KeybindsPage::show(ui, model, controller, out),
            PanelPage::ManagerSettings => ManagerSettingsView::show(ui, model, out),
        });
    }
}
