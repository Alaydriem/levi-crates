use amm_core::config::Budgets;
use amm_core::services::settings::ManagerConfigFile;

#[test]
fn a_read_clamps_hand_edited_budgets() {
    let text = "budgets {\n  event_queue = 100000000\n  io_queue = 0\n}\n";

    let (config, document) = ManagerConfigFile::read(text).unwrap_or_else(|error| panic!("{error}"));

    assert_eq!(config.budgets.event_queue, Budgets::MAX_EVENT_QUEUE);
    assert_eq!(config.budgets.io_queue, 1);
    assert_eq!(document.text(), text);
}
