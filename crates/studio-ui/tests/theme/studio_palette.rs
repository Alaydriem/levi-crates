use egui::Color32;
use studio_ui::theme::StudioPalette;

fn channel_distance(a: Color32, b: Color32) -> u8 {
    let [ar, ag, ab, aa] = a.to_array();
    let [br, bg, bb, ba] = b.to_array();
    [ar.abs_diff(br), ag.abs_diff(bg), ab.abs_diff(bb), aa.abs_diff(ba)].into_iter().max().unwrap_or(0)
}

#[test]
fn mix_interpolates_each_channel() {
    let midpoint = StudioPalette::mix(Color32::BLACK, Color32::WHITE, 0.5);

    assert!(channel_distance(midpoint, Color32::from_rgb(128, 128, 128)) <= 1);
    assert_eq!(StudioPalette::mix(Color32::BLACK, Color32::WHITE, 0.0), Color32::BLACK);
    assert_eq!(StudioPalette::mix(Color32::BLACK, Color32::WHITE, 1.0), Color32::WHITE);
}

#[test]
fn mix_clamps_t_to_the_unit_range() {
    assert_eq!(StudioPalette::mix(Color32::BLACK, Color32::WHITE, -3.0), Color32::BLACK);
    assert_eq!(StudioPalette::mix(Color32::BLACK, Color32::WHITE, 7.0), Color32::WHITE);
}

#[test]
fn alpha_scales_existing_opacity_and_clamps() {
    let half = StudioPalette::alpha(Color32::from_rgb(200, 100, 50), 0.5);

    assert_eq!(half.a(), 128);
    assert_eq!(StudioPalette::alpha(StudioPalette::CARD, 2.0).a(), StudioPalette::CARD.a());
    assert_eq!(StudioPalette::alpha(Color32::WHITE, -1.0).a(), 0);
}

// WCAG 2 contrast maths over opaque sRGB colours
struct Wcag;

impl Wcag {
    fn luminance(color: Color32) -> f64 {
        let linear = |channel: u8| {
            let c = f64::from(channel) / 255.0;
            if c <= 0.040_45 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
        };
        0.2126 * linear(color.r()) + 0.7152 * linear(color.g()) + 0.0722 * linear(color.b())
    }

    fn contrast(a: Color32, b: Color32) -> f64 {
        let (la, lb) = (Self::luminance(a), Self::luminance(b));
        (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
    }
}

#[test]
fn dim_and_faint_text_meet_wcag_aa_on_every_panel_background() {
    let backgrounds = [
        ("BG_PANEL", StudioPalette::BG_PANEL),
        ("BG_RAISED", StudioPalette::BG_RAISED),
        ("BG_FIELD", StudioPalette::BG_FIELD),
        ("BG_HOVER", StudioPalette::BG_HOVER),
        ("BG_ACTIVE", StudioPalette::BG_ACTIVE),
    ];
    for (text_name, text) in [("TEXT_DIM", StudioPalette::TEXT_DIM), ("TEXT_FAINT", StudioPalette::TEXT_FAINT)] {
        for (bg_name, bg) in backgrounds {
            let ratio = Wcag::contrast(text, bg);
            assert!(ratio >= 4.5, "{text_name} on {bg_name} is {ratio:.2}:1, below 4.5:1");
        }
    }
}

#[test]
fn text_greys_keep_a_clear_step_between_levels() {
    let field = StudioPalette::BG_FIELD;
    let (text, dim, faint) = (
        Wcag::contrast(StudioPalette::TEXT, field),
        Wcag::contrast(StudioPalette::TEXT_DIM, field),
        Wcag::contrast(StudioPalette::TEXT_FAINT, field),
    );

    assert!(text > dim * 1.4, "TEXT {text:.2} is too close to TEXT_DIM {dim:.2}");
    assert!(dim > faint * 1.4, "TEXT_DIM {dim:.2} is too close to TEXT_FAINT {faint:.2}");
}

#[test]
fn record_colour_has_text_contrast_on_card() {
    let card = StudioPalette::mix(StudioPalette::BG_APP, StudioPalette::BG_RAISED, 0.94);

    assert!(Wcag::contrast(StudioPalette::RECORD, card) >= 3.0);
    assert!(Wcag::contrast(StudioPalette::RECORD_HOVER, card) >= 3.0);
}
