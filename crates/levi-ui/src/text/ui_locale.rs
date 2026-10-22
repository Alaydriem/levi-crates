use std::sync::Arc;

use egui::{Context, Id};
use levi_i18n::localize::Localizer;

use crate::id::LeviUiId;

/// The localizer the widgets of one egui context resolve their `Text` with.
pub struct UiLocale;

impl UiLocale {
    pub fn set(ctx: &Context, localizer: Arc<Localizer>) {
        ctx.data_mut(|data| data.insert_temp(Self::id(), localizer));
    }

    /// The installed localizer, or English when none is installed.
    pub fn get(ctx: &Context) -> Arc<Localizer> {
        ctx.data(|data| data.get_temp::<Arc<Localizer>>(Self::id())).unwrap_or_else(|| Arc::new(Localizer::english()))
    }

    fn id() -> Id {
        Id::unique(LeviUiId::Locale)
    }
}
