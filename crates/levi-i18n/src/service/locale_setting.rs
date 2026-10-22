/// The language the user picked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocaleSetting {
    English,
    /// The `en_XA` pseudo-locale: accented, bracketed and padded text that shows unmarked
    /// strings and layouts that break with longer languages.
    Pseudo,
    /// A locale id such as `ru` or `pt-BR`; it is matched against the shipped packs.
    Locale(String),
}

impl LocaleSetting {
    pub const PSEUDO_LOCALE: &'static str = "en_XA";

    /// The locale id to look for, or `None` for English.
    pub fn requested(&self) -> Option<&str> {
        match self {
            Self::English => None,
            Self::Pseudo => Some(Self::PSEUDO_LOCALE),
            Self::Locale(locale) => Some(locale),
        }
    }
}
