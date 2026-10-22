use serde::Deserialize;

/// The crate that a token is generated into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub enum Owner {
    #[serde(rename = "levi-ui")]
    LeviUi,
    #[serde(rename = "studio-ui")]
    StudioUi,
}
