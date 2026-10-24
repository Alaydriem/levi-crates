use egui::{Ui, vec2};
use levi_ui::gallery::{Sample, SheetGrid};

use crate::component::library::{HeroCard, HeroCardView};
use crate::gallery::GalleryState;

/// Paper: Editor components, `Editor / Hero card` (`G64-0`). A card that opens and a broken one. The studio bundles
/// no pictures, so the card shows the app background in the picture's place.
pub struct HeroCardPage;

impl HeroCardPage {
    /// Sheet chrome: the card size.
    const SIZE: egui::Vec2 = vec2(900.0, 506.0);

    pub fn show(ui: &mut Ui, _state: &mut GalleryState) {
        SheetGrid::page(ui, "Hero card", |ui| {
            let cards = [
                ("READY", false, "Nether trip", ["02:14:08", "6 players", "5 scenes", "Today, 14:02"]),
                ("BROKEN", true, "Old realm backup", ["00:00:00", "0 players", "0 scenes", "Sep 30"]),
            ];
            for (caption, broken, title, meta) in cards {
                let view = HeroCardView {
                    title: Sample::text(title),
                    meta: meta.iter().map(|fact| Sample::text(fact)).collect(),
                    open: Sample::text("Open in editor"),
                    broken,
                };
                SheetGrid::section(ui, caption, |ui| {
                    HeroCard::new(&view).show(ui, Self::SIZE);
                });
            }
        });
    }
}
