use egui::Id;
use levi_ui::id::LeviUiId;

/// An id enum from another crate, with a derived `Hash`.
#[derive(Debug, Clone, Copy, Hash)]
enum OtherId {
    First,
    Second,
}

#[test]
fn a_levi_ui_id_never_equals_the_same_numbered_variant_of_another_enum() {
    assert_ne!(Id::unique(LeviUiId::Locale), Id::unique(OtherId::First));
    assert_ne!(Id::unique(LeviUiId::SegmentSlide), Id::unique(OtherId::Second));
}

#[test]
fn levi_ui_id_variants_keep_their_own_ids() {
    assert_ne!(Id::unique(LeviUiId::Locale), Id::unique(LeviUiId::SegmentSlide));
}
