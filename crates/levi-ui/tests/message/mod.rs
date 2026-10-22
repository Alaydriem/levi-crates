use std::collections::{BTreeSet, HashSet};

use levi_i18n::message::Message;
use levi_ui::LeviUiMessage;

#[test]
fn every_message_has_text_and_a_unique_key() {
    let mut keys = HashSet::new();
    for message in LeviUiMessage::ALL {
        assert!(!message.msgid().trim().is_empty(), "{message:?}");
        assert!(keys.insert((message.context(), message.msgid())), "{message:?} repeats a key");
    }
}

#[test]
fn index_is_the_position_in_all() {
    for (position, message) in LeviUiMessage::ALL.iter().enumerate() {
        assert_eq!(message.index(), position, "{message:?}");
    }
}

#[test]
fn placeholders_match_between_singular_and_plural() {
    let placeholders = |text: &'static str| -> BTreeSet<&'static str> {
        text.split('{').skip(1).filter_map(|part| part.split_once('}').map(|(name, _)| name)).collect()
    };
    for message in LeviUiMessage::ALL {
        if let Some(plural) = message.plural() {
            assert_eq!(placeholders(message.msgid()), placeholders(plural), "{message:?}");
        }
    }
}
