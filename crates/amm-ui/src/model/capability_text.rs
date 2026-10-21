use amm_core::structs::descriptor::Capability;

/// The names and one-line meanings of the capabilities, as the panel and the toasts show them.
pub struct CapabilityText;

impl CapabilityText {
    pub fn name(cap: Capability) -> &'static str {
        match cap {
            Capability::Camera => "Camera",
            Capability::Input => "Input",
            Capability::Hud => "Hud",
            Capability::Overlay => "Overlay",
            Capability::WorldDraw => "WorldDraw",
        }
    }

    pub fn blurb(cap: Capability) -> &'static str {
        match cap {
            Capability::Camera => "Moves or replaces the game camera",
            Capability::Input => "Takes keyboard and mouse input",
            Capability::Hud => "Owns what shows on the play screen",
            Capability::Overlay => "Draws interactive panels over the game",
            Capability::WorldDraw => "Draws markers and labels in the world",
        }
    }
}
