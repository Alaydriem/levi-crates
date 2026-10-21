use amm_core::structs::descriptor::{Capability, ClaimMode};
use amm_core::structs::registry::RegisteredMod;
use egui::{Align2, Color32, Sense, Ui, pos2, vec2};
use studio_ui::theme::{Icon, Metrics, StudioPalette, Typeface};
use studio_ui::widgets::{Chip, ChipStyle, Draw};

use crate::model::{CapabilityText, LeaseState, PanelModel};
use crate::views::TableHeader;

/// The Capabilities tab: each capability with the mod's claim, its reason and the current lease state.
pub struct CapabilitiesTab;

impl CapabilitiesTab {
    const PAD: f32 = 10.0;
    const LINE_GAP: f32 = 4.0;
    const COLS: [f32; 3] = [0.0, 0.46, 0.78];

    pub fn show(ui: &mut Ui, model: &PanelModel, registered: &RegisteredMod) {
        let width = ui.available_width() - 8.0;
        let intro = format!(
            "What {} declares it may use. An exclusive claim goes to one mod at a time.",
            registered.descriptor.display_name
        );
        ui.label(egui::RichText::new(intro).font(Typeface::help()).color(StudioPalette::TEXT_DIM));
        ui.add_space(Metrics::GAP_M);
        TableHeader::show(ui, width, &["Capability", "Claim", "Lease"], &Self::COLS);
        let name_height = Draw::line_height(ui.painter(), &Typeface::label_strong()).max(Chip::HEIGHT);
        // the name and the claim chip share the first line; the blurb and the reason start on the second
        let first_line = Self::PAD + name_height * 0.5;
        let second_line = Self::PAD + name_height + Self::LINE_GAP;
        for (position, cap) in Capability::ALL.iter().enumerate() {
            let claim = registered.descriptor.claims.iter().find(|claim| claim.capability == *cap);
            let left = ui.cursor().left();
            let claim_x = TableHeader::column(left, width, Self::COLS[1]);
            let lease_x = TableHeader::column(left, width, Self::COLS[2]);
            let reason = claim.map(|claim| {
                ui.painter().layout(
                    claim.reason.clone(),
                    Typeface::note(),
                    StudioPalette::TEXT_DIM,
                    lease_x - claim_x - 12.0,
                )
            });
            let blurb_colour = if claim.is_some() { StudioPalette::TEXT_DIM } else { StudioPalette::TEXT_FAINT };
            let blurb_width = claim_x - left - TableHeader::INSET - Metrics::GAP_M;
            let blurb = ui.painter().layout(
                CapabilityText::blurb(*cap).to_string(),
                Typeface::note(),
                blurb_colour,
                blurb_width,
            );
            let reason_height = reason.as_ref().map_or(0.0, |galley| galley.size().y);
            let height = second_line + reason_height.max(blurb.size().y) + Self::PAD;
            let (row, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
            let painter = ui.painter();
            if position > 0 {
                Draw::hline(painter, row.left() + 8.0, row.right() - 8.0, row.top(), StudioPalette::LINE);
            }
            let declared = claim.is_some();
            let x0 = row.left() + TableHeader::INSET;
            let first = row.top() + first_line;
            let second = row.top() + second_line;
            let name_colour = if declared { StudioPalette::TEXT } else { StudioPalette::TEXT_FAINT };
            Draw::text(
                painter,
                pos2(x0, first),
                Align2::LEFT_CENTER,
                CapabilityText::name(*cap),
                Typeface::label_strong(),
                name_colour,
            );
            painter.galley(pos2(x0, second), blurb, blurb_colour);
            let (Some(claim), Some(reason)) = (claim, reason) else {
                Draw::text(
                    painter,
                    pos2(claim_x, row.center().y),
                    Align2::LEFT_CENTER,
                    "Not declared",
                    Typeface::help(),
                    StudioPalette::TEXT_FAINT,
                );
                Draw::text(
                    painter,
                    pos2(lease_x + 4.0, row.center().y),
                    Align2::LEFT_CENTER,
                    "\u{2014}",
                    Typeface::body(),
                    StudioPalette::TEXT_FAINT,
                );
                continue;
            };
            match claim.mode {
                ClaimMode::Exclusive => {
                    Chip::paint(painter, claim_x, first, "Exclusive", ChipStyle::ACCENT.with_icon(Icon::LOCK));
                }
                ClaimMode::Shared => {
                    Chip::paint(painter, claim_x, first, "Shared", ChipStyle::NEUTRAL);
                }
            }
            painter.galley(pos2(claim_x, second), reason, StudioPalette::TEXT_DIM);
            let (label, style) = Self::lease(model, model.lease_state(registered.index, *cap));
            Chip::paint(painter, lease_x, row.center().y, &label, style);
        }
    }

    fn lease(model: &PanelModel, state: LeaseState) -> (String, ChipStyle) {
        match state {
            LeaseState::Held => ("Held".to_string(), ChipStyle::SUCCESS.with_icon(Icon::CHECK)),
            LeaseState::Suspended { by } => {
                (format!("Suspended by {}", model.display_name(by)), ChipStyle::WARNING.with_icon(Icon::BLOCKED))
            }
            LeaseState::Blocked { by } => {
                (format!("Blocked by {}", model.display_name(by)), ChipStyle::DANGER.with_icon(Icon::BLOCKED))
            }
            LeaseState::Idle | LeaseState::NotClaimed => {
                ("Idle".to_string(), ChipStyle { fill: Color32::TRANSPARENT, ..ChipStyle::NEUTRAL })
            }
        }
    }
}
