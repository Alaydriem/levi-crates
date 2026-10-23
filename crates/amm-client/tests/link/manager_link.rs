use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use amm_abi::types::{AmmEvent, AmmLogLevel, AmmStatus};
use amm_client::link::ManagerLink;
use amm_core::errors::LinkError;
use amm_core::structs::descriptor::Capability;
use amm_core::structs::event::{KeybindPhase, ModEvent};
use amm_core::structs::registry::ModIndex;
use serde::Deserialize;

use crate::support::{DropFlagHandler, FakeTable, IdleHandler};

const DESCRIPTOR: &str = "id = \"alaydriem-refused\"\n";

#[derive(Debug, Deserialize, PartialEq)]
struct Card {
    scale: f32,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Keys {
    take_camera: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestConfig {
    card: Card,
    keys: Keys,
}

#[test]
fn dropping_the_link_unregisters_once() {
    let link = FakeTable::link(IdleHandler);
    let handle = FakeTable::with(|state| state.handle);
    assert!(FakeTable::with(|state| state.unregistered.is_empty()));

    drop(link);

    assert_eq!(FakeTable::with(|state| state.unregistered.clone()), vec![handle]);
}

#[test]
fn register_gives_the_descriptor_text_and_a_refusal_is_the_status() {
    FakeTable::with(|state| state.register_status = AmmStatus::DuplicateId);

    let result = ManagerLink::new(FakeTable::table(), DESCRIPTOR, Path::new(FakeTable::CONFIG_DIR), IdleHandler);

    assert_eq!(result.err(), Some(LinkError::Status(AmmStatus::DuplicateId as i32)));
    assert_eq!(FakeTable::with(|state| state.descriptor.clone()), Some(DESCRIPTOR.to_owned()));
    assert_eq!(FakeTable::with(|state| state.config_dir.clone()), Some(FakeTable::CONFIG_DIR.to_owned()));
    assert!(FakeTable::with(|state| state.unregistered.is_empty()));
}

#[test]
fn events_come_back_as_core_events_in_order() {
    let mut link = FakeTable::link(IdleHandler);
    let sent = vec![
        ModEvent::Granted(Capability::Camera),
        ModEvent::Keybind { action: 2, phase: KeybindPhase::Pressed },
        ModEvent::Revoked { capability: Capability::Input, by: ModIndex(4) },
        ModEvent::Disabled,
    ];
    FakeTable::with(|state| state.events.extend(sent.iter().map(|event| AmmEvent::from(*event))));

    let received: Vec<ModEvent> = link.events().collect();

    assert_eq!(received, sent);
    assert_eq!(link.events().count(), 0);
}

#[test]
fn config_parses_the_hcl_document_into_the_mods_type() {
    let link = FakeTable::link(IdleHandler);
    FakeTable::set_config(
        "# kept by the manager\ncard {\n  scale = 0.75\n}\nkeys {\n  take_camera = \"Ctrl+Shift+C\"\n}\n",
    );

    let config: TestConfig = match link.config() {
        Ok(config) => config,
        Err(error) => panic!("config did not parse: {error}"),
    };

    assert_eq!(
        config,
        TestConfig { card: Card { scale: 0.75 }, keys: Keys { take_camera: "Ctrl+Shift+C".to_owned() } }
    );
}

#[test]
fn config_that_does_not_fit_the_mods_type_is_an_error() {
    let link = FakeTable::link(IdleHandler);
    FakeTable::set_config("card {\n  scale = \"large\"\n}\n");

    let result = link.config::<TestConfig>();

    assert!(matches!(result, Err(LinkError::Json { .. })));
}

#[test]
fn tracing_events_reach_the_table_through_the_bridge() {
    let link = FakeTable::link(IdleHandler);

    tracing::info!(chunk = 12, "from a tracing library");

    let messages: Vec<String> = FakeTable::with(|state| state.logs.iter().map(|call| call.message.clone()).collect());
    assert_eq!(messages, vec!["from a tracing library".to_owned()]);
    drop(link);
}

#[test]
fn logging_after_the_link_is_dropped_does_not_reach_the_table() {
    let link = FakeTable::link(IdleHandler);
    curia::info!("while linked", { n: 1 });
    drop(link);

    curia::info!("after unlink", { n: 2 });

    let messages: Vec<String> = FakeTable::with(|state| state.logs.iter().map(|call| call.message.clone()).collect());
    assert_eq!(messages, vec!["while linked".to_owned()]);
}

#[test]
fn the_link_sink_logs_through_the_links_handle_until_the_link_drops() {
    let link = FakeTable::link(IdleHandler);
    let sink = link.sink();
    let event = curia::LogEvent {
        level: curia::Level::Info,
        target: "demo".to_owned(),
        message: "hello".to_owned(),
        fields: curia::Fields::new(),
        timestamp: chrono::Utc::now(),
        file: None,
        line: None,
    };

    curia::Sink::emit(&sink, &event);
    drop(link);
    curia::Sink::emit(&sink, &event);

    assert_eq!(
        FakeTable::with(|state| state.logs.iter().map(|call| call.message.clone()).collect::<Vec<_>>()),
        vec!["hello"]
    );
}

#[test]
fn an_unregister_that_returns_ok_frees_the_frame_state() {
    let dropped = Arc::new(AtomicBool::new(false));
    let link = FakeTable::link(DropFlagHandler::new(Arc::clone(&dropped)));

    drop(link);

    assert!(dropped.load(Ordering::SeqCst));
}

#[test]
fn a_refused_unregister_keeps_the_frame_state_and_logs_an_error() {
    let dropped = Arc::new(AtomicBool::new(false));
    let link = FakeTable::link(DropFlagHandler::new(Arc::clone(&dropped)));
    FakeTable::with(|state| state.unregister_status = AmmStatus::NotPermitted);

    drop(link);

    assert!(!dropped.load(Ordering::SeqCst));
    let errors =
        FakeTable::with(|state| state.logs.iter().filter(|call| call.level == AmmLogLevel::Error as u8).count());
    assert_eq!(errors, 1);
}

#[test]
fn holder_copies_the_id_and_none_when_no_mod_holds_it() {
    let link = FakeTable::link(IdleHandler);
    let nobody = link.holder(Capability::Camera);
    FakeTable::with(|state| state.holder_id = Some("alaydriem-freecam"));

    let holder = link.holder(Capability::Camera);

    assert_eq!(nobody, None);
    assert_eq!(holder.as_deref(), Some("alaydriem-freecam"));
}

#[test]
fn holder_reads_an_id_longer_than_its_first_buffer() {
    const LONG: &str = "a-mod-id-that-is-longer-than-sixty-four-bytes-so-the-first-buffer-is-too-small";
    let link = FakeTable::link(IdleHandler);
    FakeTable::with(|state| state.holder_id = Some(LONG));

    let holder = link.holder(Capability::Overlay);

    assert_eq!(holder.as_deref(), Some(LONG));
}

#[test]
fn link_reads_the_locale() {
    let link = FakeTable::link(IdleHandler);
    let english = link.locale();
    FakeTable::with(|state| state.locale = "en_XA");

    let pseudo = link.locale();

    assert_eq!(english.as_deref(), Some(""));
    assert_eq!(pseudo.as_deref(), Some("en_XA"));
}

#[test]
fn a_link_without_a_frame_registers_with_no_callbacks_and_unregisters_on_drop() {
    let link = FakeTable::link_without_frame();
    let handle = FakeTable::with(|state| state.handle);
    let callbacks = FakeTable::with(|state| state.callbacks);

    drop(link);

    let Some(callbacks) = callbacks else { panic!("nothing registered") };
    assert!(callbacks.frame.is_none());
    assert!(callbacks.textures_lost.is_none());
    assert!(callbacks.user.is_null());
    assert!(FakeTable::with(|state| state.registered_without_frame()));
    assert_eq!(FakeTable::with(|state| state.descriptor.clone()), Some(FakeTable::DESCRIPTOR.to_owned()));
    assert_eq!(FakeTable::with(|state| state.unregistered.clone()), vec![handle]);
}

#[test]
fn a_link_without_a_frame_gets_keybind_and_settings_events() {
    let mut link = FakeTable::link_without_frame();
    let sent = [
        ModEvent::Enabled,
        ModEvent::Keybind { action: 1, phase: KeybindPhase::Pressed },
        ModEvent::SettingsChanged { paths: 2 },
        ModEvent::Keybind { action: 1, phase: KeybindPhase::Released },
    ];
    sent.iter().for_each(|event| FakeTable::push_event(*event));

    let received: Vec<ModEvent> = link.events().collect();

    assert_eq!(received, sent);
}

#[test]
fn a_link_without_a_frame_forwards_its_logs_until_it_drops() {
    let link = FakeTable::link_without_frame();
    curia::info!("while linked", { n: 1 });
    drop(link);

    curia::info!("after unlink", { n: 2 });

    let messages: Vec<String> = FakeTable::with(|state| state.logs.iter().map(|call| call.message.clone()).collect());
    assert_eq!(messages, vec!["while linked".to_owned()]);
}

#[test]
fn a_link_without_a_frame_and_a_refused_unregister_logs_one_error() {
    let link = FakeTable::link_without_frame();
    FakeTable::with(|state| state.unregister_status = AmmStatus::NotPermitted);

    drop(link);

    let errors =
        FakeTable::with(|state| state.logs.iter().filter(|call| call.level == AmmLogLevel::Error as u8).count());
    assert_eq!(errors, 1);
}

#[test]
fn a_refused_register_without_a_frame_is_the_status() {
    FakeTable::with(|state| state.register_status = AmmStatus::DuplicateId);

    let result = ManagerLink::without_frame(FakeTable::table(), DESCRIPTOR, Path::new(FakeTable::CONFIG_DIR));

    assert_eq!(result.err(), Some(LinkError::Status(AmmStatus::DuplicateId as i32)));
    assert!(FakeTable::with(|state| state.unregistered.is_empty()));
}

#[test]
fn a_link_made_with_a_frame_handler_frees_its_frame_state_on_drop() {
    let dropped = Arc::new(AtomicBool::new(false));
    let link = FakeTable::link(DropFlagHandler::new(Arc::clone(&dropped)));
    assert!(!dropped.load(Ordering::SeqCst));
    assert!(FakeTable::with(|state| !state.registered_without_frame()));

    drop(link);

    assert!(dropped.load(Ordering::SeqCst));
}
