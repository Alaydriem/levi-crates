use egui::{Align2, CentralPanel, Color32, Context, Frame, Margin, Rect, Ui, pos2, vec2};
use studio_ui::theme::{FontSource, Icon, Metrics, StudioPalette, StudioTheme, Typeface};
use studio_ui::widgets::{CardFrame, Chip, ChipStyle, Draw, Dropdown, IconButton, Slider, TextButton, Toggle};

use crate::screenshot::ScreenshotJob;

// every Studio widget, once on the opaque panel and once on the translucent card over a busy backdrop
pub struct GalleryApp {
    toggle: bool,
    slider: f64,
    choice: usize,
    job: Option<ScreenshotJob>,
}

impl GalleryApp {
    const OPTIONS: [&'static str; 3] = ["Medium", "High", "Ultra"];

    pub fn new(ctx: &Context, job: Option<ScreenshotJob>) -> Self {
        StudioTheme::install(ctx, FontSource::Embedded);
        Self { toggle: true, slider: 2.25, choice: 1, job }
    }

    fn widgets(&mut self, ui: &mut Ui) {
        Draw::caps(ui.painter(), ui.cursor().min, Align2::LEFT_TOP, "Controls", StudioPalette::TEXT_DIM);
        ui.add_space(20.0);
        ui.horizontal(|ui| {
            Toggle::show(ui, &mut self.toggle);
            ui.label(if self.toggle { "Enabled" } else { "Disabled" });
        });
        ui.add_space(Metrics::GAP_S);
        ui.horizontal(|ui| {
            Slider::new(&mut self.slider, 0.5..=8.0, 0.25).show(ui, 200.0);
            ui.label(format!("{:.2}", self.slider));
        });
        ui.add_space(Metrics::GAP_S);
        Dropdown::new(&mut self.choice, &Self::OPTIONS).show(ui, 200.0);
        ui.add_space(Metrics::GAP_M);
        ui.horizontal(|ui| {
            TextButton::primary(ui, "Apply");
            TextButton::secondary(ui, "Cancel");
        });
        ui.add_space(Metrics::GAP_M);
        ui.horizontal(|ui| {
            for icon in [Icon::ADD, Icon::COPY, Icon::EDIT, Icon::DELETE, Icon::SETTINGS, Icon::KEYBOARD] {
                IconButton::show(ui, icon, 16.0);
            }
        });
        ui.add_space(Metrics::GAP_M);
        self.chips(ui);
        ui.add_space(Metrics::GAP_M);
        ui.label(egui::RichText::new("Body text 15 pt").font(Typeface::body()));
        ui.label(egui::RichText::new("Help text").font(Typeface::help()).color(StudioPalette::TEXT_DIM));
        ui.label(egui::RichText::new("Mono text ctrl+shift+m").font(Typeface::mono_text()));
        ui.label(egui::RichText::new("Semibold text").font(Typeface::body_strong()));
    }

    fn chips(&self, ui: &mut Ui) {
        let (rect, _) = ui.allocate_exact_size(vec2(360.0, 56.0), egui::Sense::hover());
        let painter = ui.painter();
        let styles = [
            ("neutral", ChipStyle::NEUTRAL),
            ("accent", ChipStyle::ACCENT.with_icon(Icon::CHECK)),
            ("warning", ChipStyle::WARNING.with_icon(Icon::WARNING)),
            ("danger", ChipStyle::DANGER.with_icon(Icon::BLOCKED)),
            ("ok", ChipStyle::SUCCESS),
        ];
        let mut x = rect.left();
        for (label, style) in styles {
            x = Chip::paint(painter, x, rect.top() + 10.0, label, style).right() + 6.0;
        }
        Chip::chord(painter, rect.left(), rect.top() + 42.0, &["Ctrl", "Shift", "M"], false);
        Chip::chord(painter, rect.left() + 170.0, rect.top() + 42.0, &["Alt", "F4"], true);
    }

    fn backdrop(ui: &Ui, rect: Rect) {
        let painter = ui.painter();
        Draw::fill(painter, rect, 0, Color32::from_rgb(0x3E, 0x6B, 0x4A));
        for step in 0..8 {
            let t = step as f32 / 8.0;
            let band = Rect::from_min_size(pos2(rect.left(), rect.top() + rect.height() * t), vec2(rect.width(), 40.0));
            Draw::fill(
                painter,
                band,
                0,
                StudioPalette::mix(Color32::from_rgb(0x5B, 0x8F, 0x3A), Color32::from_rgb(0xB8, 0x9B, 0x5E), t),
            );
        }
    }
}

impl eframe::App for GalleryApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        if let Some(job) = self.job.as_mut() {
            job.tick(ui.ctx());
        }
        let half = ui.available_width() / 2.0;
        CentralPanel::default().frame(Frame::new().fill(StudioPalette::BG_PANEL)).show(ui, |ui| {
            let full = ui.max_rect();
            let left = Rect::from_min_max(full.min, pos2(full.left() + half, full.bottom()));
            let right = Rect::from_min_max(pos2(full.left() + half, full.top()), full.max);
            Self::backdrop(ui, right);
            ui.scope_builder(egui::UiBuilder::new().max_rect(left.shrink(24.0)), |ui| {
                Frame::new().inner_margin(Margin::same(0)).show(ui, |ui| self.widgets(ui));
            });
            ui.scope_builder(egui::UiBuilder::new().max_rect(right.shrink(24.0)), |ui| {
                CardFrame::card().show(ui, |ui| self.widgets(ui));
            });
        });
    }
}
