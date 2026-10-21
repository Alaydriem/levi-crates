use amm_core::structs::descriptor::Capability;
use amm_core::structs::toast::ToastKind;
use amm_ui::model::{PanelModel, ToastText};

use crate::support::{FourMods, MINIMAP};

fn toggle_action(mods: &FourMods) -> u16 {
    let registered = mods.manager.registry().get(mods.freecam).unwrap_or_else(|| panic!("freecam is registered"));
    let position = registered.descriptor.keybinds.iter().position(|bind| bind.action == "toggle");
    position.unwrap_or_else(|| panic!("freecam has a toggle keybind")) as u16
}

#[test]
fn blocked_toast_names_capability_holder_mod_and_action() {
    let mods = FourMods::new(MINIMAP);
    let kind = ToastKind::Blocked {
        capability: Capability::Camera,
        holder: mods.dejavu,
        mod_index: mods.freecam,
        action: toggle_action(&mods),
    };

    let text = ToastText::render(&kind, &PanelModel::new(&mods.manager, "1.4.1", 0));

    assert_eq!(text, "Camera held by DejaVu. Freecam Toggle freecam blocked.");
}

#[test]
fn denied_toast_names_capability_holder_and_requester() {
    let mods = FourMods::new(MINIMAP);
    let kind = ToastKind::Denied { capability: Capability::Camera, holder: mods.dejavu, requester: mods.freecam };

    let text = ToastText::render(&kind, &PanelModel::new(&mods.manager, "1.4.1", 0));

    assert_eq!(text, "Camera held by DejaVu. Freecam request denied.");
}
