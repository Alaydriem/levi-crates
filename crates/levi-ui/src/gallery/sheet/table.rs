use egui::{Align2, Frame, Margin, Rect, Sense, Ui, Vec2, vec2};
use levi_i18n::text::{Args, Text, UserText};

use crate::LeviUiMessage;
use crate::draw::{Corner, Draw, TextFit, TextStyle, TypeSize};
use crate::font::FontFace;
use crate::gallery::{GalleryState, Sample, SheetGrid};
use crate::tokens::{ColorToken, color};
use crate::widget::{Chip, ChipKind, ChipTone, KeyChord, TableColumn, TableHeader, TableRow, TableRowKind};

/// Paper: Mod manager, `Widget / Table header + Table row` (`DK4-0`). A keybind table: the header,
/// a mod's group row, and one row of each kind.
pub struct TableSheet;

impl TableSheet {
    /// Sheet chrome: the container's width and padding.
    const WIDTH: f32 = 980.0;
    const PAD_TOP: i8 = 16;
    const PAD: i8 = 20;
    /// Paper: Table, the columns as shares of the row's inner width (358, 244, 170 and the rest of 904).
    const COLUMNS: [TableColumn; 4] = [
        TableColumn { width_fraction: 0.396 },
        TableColumn { width_fraction: 0.270 },
        TableColumn { width_fraction: 0.188 },
        TableColumn { width_fraction: 0.146 },
    ];
    /// Paper: Table, the group row's height, inset and top padding.
    const GROUP_HEIGHT: f32 = 36.0;
    const GROUP_INSET: f32 = 18.0;
    const GROUP_TOP: f32 = 12.0;

    pub fn show(ui: &mut Ui, _state: &mut GalleryState) {
        SheetGrid::page(ui, "Table header + Table row", |ui| {
            Frame::new()
                .fill(<color::BgPanel as ColorToken>::VALUE)
                .stroke(Draw::border(<color::Line as ColorToken>::VALUE))
                .corner_radius(Corner::Panel.radius(Rect::NOTHING))
                .inner_margin(Margin { left: Self::PAD, right: Self::PAD, top: Self::PAD_TOP, bottom: Self::PAD })
                .show(ui, |ui| {
                    ui.set_width(Self::WIDTH - 2.0 * f32::from(Self::PAD));
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    let heads = ["ACTION", "KEY CHORD", "CAPABILITY", "SCOPE"];
                    TableHeader::new(
                        heads.iter().zip(Self::COLUMNS).map(|(head, c)| (Sample::text(head), c)).collect(),
                    )
                    .show(ui);
                    Self::group(ui, "FREECAM");
                    Self::row(ui, TableRowKind::Plain, ["Toggle freecam", "Ctrl+P", "Camera", "Global"]);
                    Self::row(ui, TableRowKind::Plain, ["Toggle input capture", "Ctrl+E", "Input", "Global"]);
                    Self::row(ui, TableRowKind::Capturing, ["Cycle mode", "", "Camera", "Global"]);
                    let other = Args::new().user("name", UserText::from_model("Light Level".to_owned()));
                    let conflict = TableRowKind::Conflict(Text::format(LeviUiMessage::AlsoBoundBy, other));
                    Self::row(ui, conflict, ["Toggle path markers", "Ctrl+B", "WorldDraw", "Global"]);
                    let disabled = TableRowKind::Disabled(Sample::text("Turn on Freecam to change this key"));
                    Self::row(ui, disabled, ["Fly forward", "W", "Input", "While held"]);
                });
        });
    }

    // a mod's group row: its name in small caps
    fn group(ui: &mut Ui, name: &str) {
        let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), Self::GROUP_HEIGHT), Sense::hover());
        let style = TextStyle::new(TypeSize::Caption, FontFace::Semibold).tracked();
        let dim = <color::TextDim as ColorToken>::VALUE;
        let at = egui::pos2(rect.left() + Self::GROUP_INSET, rect.top() + Self::GROUP_TOP);
        Draw::text(ui.painter(), at, Align2::LEFT_TOP, &Sample::text(name), style, dim, TextFit::Line);
    }

    // one row: the action, the chord ("Ctrl+P", keys split at "+"), the capability chip and the scope
    fn row(ui: &mut Ui, kind: TableRowKind, cells: [&str; 4]) {
        let conflict = matches!(kind, TableRowKind::Conflict(_));
        TableRow::new(&Self::COLUMNS, kind).show(ui, |index, ui| match index {
            0 => {
                let style = TextStyle::new(TypeSize::Body, FontFace::Regular);
                SheetGrid::label(ui, cells[0], style, <color::Text as ColorToken>::VALUE);
            }
            1 => {
                let keys = cells[1].split('+').map(Sample::text).collect();
                KeyChord::new(keys).conflict(conflict).show(ui);
            }
            2 => {
                Chip::new(ChipKind::Display(ChipTone::Outline), Sample::text(cells[2])).show(ui);
            }
            _ => {
                let style = TextStyle::new(TypeSize::Help, FontFace::Regular);
                SheetGrid::label(ui, cells[3], style, <color::TextDim as ColorToken>::VALUE);
            }
        });
    }
}
