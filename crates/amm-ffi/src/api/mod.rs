pub mod capture;
pub mod events;
pub mod input_query;
pub mod leases;
pub mod log;
pub mod register;
pub mod settings;
pub mod table;

pub use capture::{amm_held_by_other, amm_request_screenshot, amm_texture_budget};
pub use events::amm_poll_events;
pub use input_query::amm_key_down;
pub use leases::{amm_acquire, amm_holder, amm_release, amm_set_cursor_mode};
pub use log::amm_log;
pub use register::{amm_config_document, amm_register, amm_unregister};
pub use settings::amm_set_setting;
pub use table::amm_api_for;
