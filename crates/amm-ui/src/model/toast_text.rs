use amm_core::structs::toast::ToastKind;

use crate::model::{CapabilityText, PanelModel};

/// The text of a toast, built only while the toast is drawn.
pub struct ToastText;

impl ToastText {
    const SAVE_FAILED: &'static str = "Save failed, try again";

    pub fn render(kind: &ToastKind, names: &PanelModel) -> String {
        match *kind {
            ToastKind::Denied { capability, holder, requester } => format!(
                "{} held by {}. {} request denied.",
                CapabilityText::name(capability),
                names.display_name(holder),
                names.display_name(requester),
            ),
            ToastKind::Blocked { capability, holder, mod_index, action } => format!(
                "{} held by {}. {} {} blocked.",
                CapabilityText::name(capability),
                names.display_name(holder),
                names.display_name(mod_index),
                names.keybind(mod_index, action).map_or("keybind", |bind| bind.label.as_str()),
            ),
            ToastKind::FrameDisabled(index) => {
                format!("{} stopped drawing. It failed too many frames in a row.", names.display_name(index))
            }
            ToastKind::SaveFailed => Self::SAVE_FAILED.to_string(),
        }
    }
}
