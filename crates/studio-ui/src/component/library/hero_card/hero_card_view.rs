use levi_i18n::text::Text;

/// What the Library hero card shows.
pub struct HeroCardView {
    pub title: Text,
    /// The facts under the title (length, players, scenes, date), drawn with a dot between them.
    pub meta: Vec<Text>,
    /// The button's words.
    pub open: Text,
    /// The replay cannot be opened: the picture is faint and the button is muted and does nothing.
    pub broken: bool,
}
