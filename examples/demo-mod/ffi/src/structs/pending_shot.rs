/// The one screenshot the demo waits on: the sequence the manager gave, the number in its file name, and whether
/// the demo took the Hud lease only for this request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PendingShot {
    pub sequence: u32,
    pub number: u32,
    pub temporary_hud: bool,
}
