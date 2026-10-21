use amm_core::services::settings::ColorCodec;
use amm_core::structs::descriptor::{FieldKind, SettingField};

/// A settings value as the panel shows it, in a value box or in a sentence such as "Changed in game to
/// 2.50". A float shows the decimals its step implies and its unit; an int shows a whole number.
pub struct ValueText;

impl ValueText {
    const MAX_DECIMALS: usize = 6;

    pub fn display(field: &SettingField, value: &hcl::Value) -> String {
        match &field.kind {
            FieldKind::Bool => match value.as_bool() {
                Some(true) => "on".to_string(),
                Some(false) => "off".to_string(),
                None => value.to_string(),
            },
            FieldKind::Float { step, unit, .. } => match value.as_f64() {
                Some(number) => Self::float(number, *step, unit.as_deref()),
                None => value.to_string(),
            },
            FieldKind::Int { .. } => match value.as_i64() {
                Some(number) => number.to_string(),
                None => value.to_string(),
            },
            FieldKind::Enum { options } => options
                .iter()
                .find(|option| option.value == *value)
                .map_or_else(|| value.to_string(), |o| o.label.clone()),
            FieldKind::Color { encoding, .. } => match ColorCodec::to_rgba(value, *encoding) {
                Some([r, g, b, a]) => format!("#{r:02X}{g:02X}{b:02X}{a:02X}"),
                None => value.to_string(),
            },
            FieldKind::Chord => match value.as_str() {
                Some(text) => text.to_string(),
                None => value.to_string(),
            },
        }
    }

    /// A float with as many decimals as `step` has, then the unit. A one-character unit such as `x`
    /// or `%` joins the number; a longer unit such as `blocks/s` follows a space.
    pub fn float(number: f64, step: f64, unit: Option<&str>) -> String {
        let decimals = Self::decimals(step);
        match unit {
            Some(unit) if unit.chars().count() > 1 => format!("{number:.decimals$} {unit}"),
            Some(unit) => format!("{number:.decimals$}{unit}"),
            None => format!("{number:.decimals$}"),
        }
    }

    /// The texts of a slider field at its min and at its max, which between them hold its widest
    /// text. `None` for a field with no value box.
    pub fn bounds(field: &SettingField) -> Option<[String; 2]> {
        match &field.kind {
            FieldKind::Int { min, max, .. } => Some([min.to_string(), max.to_string()]),
            FieldKind::Float { min, max, step, unit } => {
                Some([Self::float(*min, *step, unit.as_deref()), Self::float(*max, *step, unit.as_deref())])
            }
            _ => None,
        }
    }

    /// The decimals of a step: 0.05 has 2, 0.5 has 1, 1 and 16 have none.
    pub fn decimals(step: f64) -> usize {
        let mut scaled = step.abs();
        for decimals in 0..Self::MAX_DECIMALS {
            if (scaled - scaled.round()).abs() < 1e-9 * scaled.max(1.0) {
                return decimals;
            }
            scaled *= 10.0;
        }
        Self::MAX_DECIMALS
    }
}
