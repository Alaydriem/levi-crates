use std::path::Path;
use std::sync::Arc;

use egui::{FontData, FontDefinitions, FontFamily};

use crate::theme::{FontSource, Typeface};

// Nordwand Mono is embedded; Segoe Fluent Icons is read from the Windows font folder and never shipped.
// A missing icon file falls back to egui's built-in fonts
pub struct FontLoader;

impl FontLoader {
    pub const FONT_DIR: &'static str = r"C:\Windows\Fonts";
    pub const TEXT_KEY: &'static str = "nordwand-mono";
    pub const SEMIBOLD_KEY: &'static str = "nordwand-mono-semibold";
    pub const ICONS_KEY: &'static str = "segoe-fluent-icons";
    const ICONS_FILE: &'static str = "SegoeIcons.ttf";
    const TEXT: &'static [u8] = include_bytes!("../../assets/fonts/NordwandMono-Medium.ttf");
    const SEMIBOLD: &'static [u8] = include_bytes!("../../assets/fonts/NordwandMono-SemiBold.ttf");

    pub fn definitions(source: FontSource) -> FontDefinitions {
        let mut defs = FontDefinitions::default();
        let builtin_proportional = defs.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
        let builtin_mono = defs.families.get(&FontFamily::Monospace).cloned().unwrap_or_default();
        let embedded = source == FontSource::Embedded;
        let text = Self::embed(&mut defs, embedded, Self::TEXT_KEY, Self::TEXT);
        let semibold = Self::embed(&mut defs, embedded, Self::SEMIBOLD_KEY, Self::SEMIBOLD);
        let icons = Self::load_icons(&mut defs, embedded);

        let ui_chain = Self::chain(&[text.clone(), icons.clone()], &builtin_proportional);
        let semibold_chain = Self::chain(&[semibold, text.clone(), icons.clone()], &builtin_proportional);
        let mono_chain = Self::chain(&[text, icons.clone()], &builtin_mono);
        let icon_chain = Self::chain(&[icons], &builtin_proportional);

        defs.families.insert(FontFamily::Proportional, ui_chain);
        defs.families.insert(FontFamily::Monospace, mono_chain.clone());
        defs.families.insert(FontFamily::Name(Typeface::SEMIBOLD.into()), semibold_chain);
        defs.families.insert(FontFamily::Name(Typeface::MONO.into()), mono_chain);
        defs.families.insert(FontFamily::Name(Typeface::ICONS.into()), icon_chain);
        defs
    }

    fn embed(defs: &mut FontDefinitions, enabled: bool, key: &str, bytes: &'static [u8]) -> Option<String> {
        if !enabled {
            return None;
        }
        defs.font_data.insert(key.to_owned(), Arc::new(FontData::from_static(bytes)));
        Some(key.to_owned())
    }

    fn load_icons(defs: &mut FontDefinitions, enabled: bool) -> Option<String> {
        if !enabled {
            return None;
        }
        let path = Path::new(Self::FONT_DIR).join(Self::ICONS_FILE);
        match std::fs::read(&path) {
            Ok(bytes) => {
                defs.font_data.insert(Self::ICONS_KEY.to_owned(), Arc::new(FontData::from_owned(bytes)));
                Some(Self::ICONS_KEY.to_owned())
            }
            Err(_) => {
                curia::warn!("icon font not found; using the built-in font", {
                    file: Self::ICONS_FILE,
                    directory: Self::FONT_DIR,
                });
                None
            }
        }
    }

    fn chain(preferred: &[Option<String>], fallback: &[String]) -> Vec<String> {
        preferred.iter().flatten().cloned().chain(fallback.iter().cloned()).collect()
    }
}
