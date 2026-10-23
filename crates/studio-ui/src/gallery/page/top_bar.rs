use egui::Ui;
use levi_i18n::text::{Text, UserText};

use super::BarRows;
use crate::component::bar::{SceneSwitcherChipView, StudioMode, TopBar, TopBarView};
use crate::gallery::GalleryState;
use crate::tokens::selector::SceneColor;

/// Paper: Editor components, `Top bar` (`G-0`). Edit, Preview and Read only at 1920, and Edit at 1280.
pub struct TopBarPage;

impl TopBarPage {
    /// Paper: the bar artboards are 1920 wide; the narrow row is the 1280 window.
    pub const WIDTH: f32 = 1920.0;
    const SMALL: f32 = 1280.0;

    pub fn show(ui: &mut Ui, _state: &mut GalleryState) {
        BarRows::page(ui, |ui| {
            let rows = [
                ("NORMAL", StudioMode::Edit, true, Self::WIDTH),
                ("PREVIEW", StudioMode::Preview, true, Self::WIDTH),
                ("READ ONLY", StudioMode::Edit, false, Self::WIDTH),
                ("1280", StudioMode::Edit, true, Self::SMALL),
            ];
            for (caption, mode, editable, width) in rows {
                let view = Self::view(mode, editable);
                BarRows::row(ui, caption, width, |ui| {
                    TopBar::new(&view).show(ui);
                });
            }
        });
    }

    fn words(text: &str) -> Text {
        Text::user(UserText::from_model(text.to_owned()))
    }

    fn view(mode: StudioMode, editable: bool) -> TopBarView {
        TopBarView {
            title: Self::words("Nether trip"),
            mode,
            can_undo: editable,
            can_redo: false,
            history_open: false,
            scene: SceneSwitcherChipView {
                scene: Some((Self::words("Arrival"), SceneColor::Lime)),
                position: Some((1, 5)),
                open: false,
            },
        }
    }
}
