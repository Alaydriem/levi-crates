use egui::FontFamily;
use levi_ui::font::FontFace;
use studio_ui::theme::{FontLoader, FontSource, Typeface};

#[test]
fn studio_fonts_also_define_the_levi_ui_faces() {
    // a studio context draws levi-ui widgets too, while the studio moves to levi-ui
    for source in [FontSource::BuiltIn, FontSource::Embedded] {
        let defs = FontLoader::definitions(source);
        for face in [FontFace::Regular, FontFace::Semibold, FontFace::Icons] {
            let chain = defs.families.get(&face.family());
            assert!(chain.is_some_and(|c| !c.is_empty()), "{source:?}: {face:?} has no font");
        }
    }
}

#[test]
fn builtin_source_still_defines_every_named_family() {
    let defs = FontLoader::definitions(FontSource::BuiltIn);

    for family in [Typeface::SEMIBOLD, Typeface::MONO, Typeface::ICONS] {
        let chain = defs.families.get(&FontFamily::Name(family.into()));
        assert!(chain.is_some_and(|c| !c.is_empty()), "{family} has no font");
    }
    assert!(!defs.font_data.contains_key(FontLoader::TEXT_KEY));
}

#[test]
fn embedded_source_registers_nordwand_for_ui_and_mono() {
    let defs = FontLoader::definitions(FontSource::Embedded);

    for family in [FontFamily::Proportional, FontFamily::Monospace] {
        let chain = defs.families.get(&family).cloned().unwrap_or_default();
        assert_eq!(chain.first().map(String::as_str), Some(FontLoader::TEXT_KEY));
    }
    let semibold = defs.families.get(&FontFamily::Name(Typeface::SEMIBOLD.into())).cloned().unwrap_or_default();
    assert_eq!(semibold.first().map(String::as_str), Some(FontLoader::SEMIBOLD_KEY));
    assert!(defs.font_data.contains_key(FontLoader::TEXT_KEY));
    assert!(defs.font_data.contains_key(FontLoader::SEMIBOLD_KEY));
}
