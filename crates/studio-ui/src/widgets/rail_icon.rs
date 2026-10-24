use levi_ui::icon::Icon;

// what a rail tile shows above its label: a levi-ui icon or the red recording dot
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RailIcon {
    Glyph(Icon),
    RecordDot,
}
