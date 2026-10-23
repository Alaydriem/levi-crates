//! Old studio-ui theme. Kept until DejaVu (Plan 08) moves to levi-ui; its frame handler installs it.
use std::sync::LazyLock;

use egui::{FontFamily, FontId};

// font ids of the Studio Dark type scale; the named families are built once, so a font id costs a refcount
pub struct Typeface;

static SEMIBOLD_FAMILY: LazyLock<FontFamily> = LazyLock::new(|| FontFamily::Name(Typeface::SEMIBOLD.into()));
static MONO_FAMILY: LazyLock<FontFamily> = LazyLock::new(|| FontFamily::Name(Typeface::MONO.into()));
static ICON_FAMILY: LazyLock<FontFamily> = LazyLock::new(|| FontFamily::Name(Typeface::ICONS.into()));

impl Typeface {
    pub const SEMIBOLD: &'static str = "ui-semibold";
    pub const MONO: &'static str = "ui-mono";
    pub const ICONS: &'static str = "ui-icons";

    // chip text, small version numbers
    pub const CAPTION_SIZE: f32 = 12.5;
    // UPPERCASE section labels, drawn semibold with letter spacing
    pub const SECTION_SIZE: f32 = 12.5;
    // secondary detail under a name: timestamps, blurbs, claim reasons, "Esc cancels"
    pub const NOTE_SIZE: f32 = 13.5;
    // help under a setting, page subtitles, hints and other secondary sentences
    pub const HELP_SIZE: f32 = 14.0;
    // key caps, values and ids
    pub const MONO_SIZE: f32 = 14.0;
    // tab labels, table names, toast titles and intros: one step under body
    pub const LABEL_SIZE: f32 = 15.5;
    // setting labels, list names, buttons and card titles
    pub const BODY_SIZE: f32 = 16.5;
    // the panel title
    pub const TITLE_SIZE: f32 = 21.0;
    // the title of a page or a mod
    pub const HEADING_SIZE: f32 = 23.0;

    // aliases of BODY_SIZE and HELP_SIZE for existing mods
    pub const UI_SIZE: f32 = Self::BODY_SIZE;
    pub const SMALL_SIZE: f32 = Self::HELP_SIZE;

    // icons inside a chip or next to one line of help
    pub const ICON_SMALL: f32 = 11.5;
    // icons beside body text: tabs, list entries, banners, close
    pub const ICON_SIZE: f32 = 15.5;
    // icons in a 36 point badge or an empty-state card
    pub const ICON_BADGE: f32 = 20.0;
    // icons in a 48 point page badge
    pub const ICON_HERO: f32 = 24.0;

    pub fn ui(size: f32) -> FontId {
        FontId::new(size, FontFamily::Proportional)
    }

    pub fn semibold(size: f32) -> FontId {
        FontId::new(size, SEMIBOLD_FAMILY.clone())
    }

    pub fn mono(size: f32) -> FontId {
        FontId::new(size, MONO_FAMILY.clone())
    }

    pub fn icon(size: f32) -> FontId {
        FontId::new(size, ICON_FAMILY.clone())
    }

    pub fn caption() -> FontId {
        Self::ui(Self::CAPTION_SIZE)
    }

    pub fn section() -> FontId {
        Self::semibold(Self::SECTION_SIZE)
    }

    pub fn note() -> FontId {
        Self::ui(Self::NOTE_SIZE)
    }

    pub fn help() -> FontId {
        Self::ui(Self::HELP_SIZE)
    }

    pub fn help_strong() -> FontId {
        Self::semibold(Self::HELP_SIZE)
    }

    pub fn label() -> FontId {
        Self::ui(Self::LABEL_SIZE)
    }

    pub fn label_strong() -> FontId {
        Self::semibold(Self::LABEL_SIZE)
    }

    pub fn body() -> FontId {
        Self::ui(Self::BODY_SIZE)
    }

    pub fn body_strong() -> FontId {
        Self::semibold(Self::BODY_SIZE)
    }

    pub fn title() -> FontId {
        Self::semibold(Self::TITLE_SIZE)
    }

    pub fn heading() -> FontId {
        Self::semibold(Self::HEADING_SIZE)
    }

    pub fn mono_text() -> FontId {
        Self::mono(Self::MONO_SIZE)
    }

    pub fn mono_small() -> FontId {
        Self::mono(Self::CAPTION_SIZE)
    }
}
