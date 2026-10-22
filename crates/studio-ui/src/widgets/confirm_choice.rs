// what the person picked in a confirm popover this frame
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfirmChoice {
    None,
    Keep,
    Confirm,
}
