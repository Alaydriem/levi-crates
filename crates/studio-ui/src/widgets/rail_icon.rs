// what a rail tile shows above its label: a Segoe Fluent glyph or the red recording dot
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RailIcon {
    Glyph(char),
    RecordDot,
}
