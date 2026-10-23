use std::collections::{BTreeSet, HashSet};
use std::path::PathBuf;

use levi_i18n::message::Message;
use levi_i18n_tool::pot::PotWriter;
use studio_ui::message::StudioUiMessage;

#[test]
fn every_message_has_text_and_a_unique_key() {
    let mut keys = HashSet::new();
    for message in StudioUiMessage::ALL {
        assert!(!message.msgid().trim().is_empty(), "{message:?}");
        assert!(keys.insert((message.context(), message.msgid())), "{message:?} repeats a key");
    }
}

#[test]
fn index_is_the_position_in_all() {
    for (position, message) in StudioUiMessage::ALL.iter().enumerate() {
        assert_eq!(message.index(), position, "{message:?}");
    }
}

#[test]
fn placeholders_match_between_singular_and_plural() {
    let placeholders = |text: &'static str| -> BTreeSet<&'static str> {
        text.split('{').skip(1).filter_map(|part| part.split_once('}').map(|(name, _)| name)).collect()
    };
    for message in StudioUiMessage::ALL {
        if let Some(plural) = message.plural() {
            assert_eq!(placeholders(message.msgid()), placeholders(plural), "{message:?}");
        }
    }
}

#[test]
fn pot_is_up_to_date() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("i18n").join(format!("{}.pot", StudioUiMessage::DOMAIN.name()));
    let stale = PotWriter::write::<StudioUiMessage>(&path, true).unwrap_or_else(|error| panic!("{error}"));
    assert!(!stale, "{} is out of date: run `mise run i18n:extract`", path.display());
}
