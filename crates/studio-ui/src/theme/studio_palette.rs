//! Old studio-ui theme. Kept until DejaVu (Plan 08) moves to levi-ui; its frame handler installs it.
use egui::Color32;

// the Studio Dark colour tokens
pub struct StudioPalette;

impl StudioPalette {
    pub const BG_APP: Color32 = Color32::from_rgb(0x0B, 0x0C, 0x0F);
    pub const BG_PANEL: Color32 = Color32::from_rgb(0x12, 0x14, 0x19);
    pub const BG_RAISED: Color32 = Color32::from_rgb(0x19, 0x1C, 0x23);
    pub const BG_FIELD: Color32 = Color32::from_rgb(0x20, 0x24, 0x2D);
    pub const BG_HOVER: Color32 = Color32::from_rgb(0x28, 0x2D, 0x38);
    pub const BG_ACTIVE: Color32 = Color32::from_rgb(0x30, 0x36, 0x43);
    // dark translucent fill of panels drawn over the game picture
    pub const BG_OVERLAY: Color32 = Color32::from_rgba_unmultiplied_const(8, 9, 12, 168);
    // the translucent card behind map panels
    pub const CARD: Color32 = Color32::from_rgba_unmultiplied_const(0x19, 0x1C, 0x23, 240);
    // the dark pill behind text over the map
    pub const PILL: Color32 = Color32::from_rgba_unmultiplied_const(11, 12, 15, 190);

    pub const LINE: Color32 = Color32::from_rgb(0x25, 0x2A, 0x33);
    pub const LINE_STRONG: Color32 = Color32::from_rgb(0x34, 0x3A, 0x46);
    // hairline around panels drawn over the game picture
    pub const LINE_OVERLAY: Color32 = Color32::from_rgba_unmultiplied_const(255, 255, 255, 18);

    // primary text; dim and faint text keep at least 4.5:1 (WCAG AA) on BG_PANEL, BG_RAISED, BG_FIELD, BG_HOVER and BG_ACTIVE
    pub const TEXT: Color32 = Color32::from_rgb(0xE6, 0xE8, 0xEC);
    pub const TEXT_DIM: Color32 = Color32::from_rgb(0xC0, 0xC5, 0xCD);
    pub const TEXT_FAINT: Color32 = Color32::from_rgb(0x9A, 0xA1, 0xAF);

    // conflict and denial states
    pub const WARNING: Color32 = Color32::from_rgb(0xF5, 0xB9, 0x4F);
    pub const WARNING_SOFT: Color32 = Color32::from_rgba_unmultiplied_const(0xF5, 0xB9, 0x4F, 36);
    pub const DANGER: Color32 = Color32::from_rgb(0xFF, 0x6B, 0x6B);
    pub const DANGER_SOFT: Color32 = Color32::from_rgba_unmultiplied_const(0xFF, 0x6B, 0x6B, 34);

    pub const ACCENT: Color32 = Color32::from_rgb(0x8B, 0x7C, 0xFF);
    pub const ACCENT_HOVER: Color32 = Color32::from_rgb(0x9C, 0x8F, 0xFF);
    pub const ACCENT_PRESS: Color32 = Color32::from_rgb(0x7A, 0x6A, 0xF2);
    pub const ACCENT_SOFT: Color32 = Color32::from_rgba_unmultiplied_const(0x8B, 0x7C, 0xFF, 46);

    pub const SUCCESS: Color32 = Color32::from_rgb(0x4F, 0xD1, 0xA5);

    pub const AXIS_X: Color32 = Color32::from_rgb(0xFF, 0x6B, 0x6B);
    pub const AXIS_Y: Color32 = Color32::from_rgb(0x4F, 0xD1, 0xA5);
    pub const AXIS_Z: Color32 = Color32::from_rgb(0x6C, 0xB8, 0xFF);

    // the recording dot and the live timer
    pub const RECORD: Color32 = Color32::from_rgb(0xFF, 0x4D, 0x5E);
    pub const RECORD_HOVER: Color32 = Color32::from_rgb(0xFF, 0x64, 0x73);

    pub const WHITE: Color32 = Color32::WHITE;
    pub const SHADOW: Color32 = Color32::from_rgba_unmultiplied_const(0, 0, 0, 150);
    // the map behind an open dialog
    pub const DIM: Color32 = Color32::from_rgba_unmultiplied_const(0, 0, 0, 115);

    // `color` at `alpha` (0..1), keeping its hue
    pub fn alpha(color: Color32, alpha: f32) -> Color32 {
        let [r, g, b, a] = color.to_srgba_unmultiplied();
        let scaled = (f32::from(a) * alpha.clamp(0.0, 1.0)).round() as u8;
        Color32::from_rgba_unmultiplied(r, g, b, scaled)
    }

    // gamma-space blend, `t` = 0 gives `from`
    pub fn mix(from: Color32, to: Color32, t: f32) -> Color32 {
        from.lerp_to_gamma(to, t.clamp(0.0, 1.0))
    }
}
