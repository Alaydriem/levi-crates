use amm_ui::views::ManagerSettingsView;
use egui::{Context, LayerId, RawInput, Rect, pos2, vec2};
use studio_ui::theme::{FontSource, StudioTheme};
use studio_ui::widgets::{Chip, Toggle};

const LABEL: &str = "Panel toggle";
const HELP: &str = "Opens and closes this panel from anywhere in the game";

#[test]
fn a_settings_row_keeps_its_label_help_and_control_inside_the_row() {
    let ctx = Context::default();
    StudioTheme::install(&ctx, FontSource::Embedded);

    let mut output = ctx.run_ui(RawInput::default(), |ui| {
        let painter = ui.ctx().layer_painter(LayerId::background());
        let row = Rect::from_min_size(pos2(0.0, 0.0), vec2(760.0, ManagerSettingsView::row_height(&painter)));
        let chord = Chip::chord_size(&painter, &["Ctrl", "Shift", "F10"]);
        let toggle = vec2(Toggle::SIZE.0, Toggle::SIZE.1);

        for control in [chord, toggle] {
            let parts = ManagerSettingsView::parts(&painter, row, LABEL, HELP, control);

            for (name, rect) in [("label", parts.label), ("help", parts.help), ("control", parts.control)] {
                assert!(row.contains_rect(rect), "the {name} {rect:?} leaves the row {row:?}");
            }
            assert!(
                parts.label.bottom() <= parts.help.top(),
                "the label {:?} overlaps the help {:?}",
                parts.label,
                parts.help
            );
            assert!(
                parts.help.right() < parts.control.left(),
                "the help {:?} runs into the control {:?}",
                parts.help,
                parts.control
            );
        }
    });
    // the test has no renderer, so the font atlas upload is dropped on purpose
    output.textures_delta.clear();
}
