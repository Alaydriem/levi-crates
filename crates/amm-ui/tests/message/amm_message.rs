use std::collections::{BTreeSet, HashSet};

use amm_ui::message::AmmMessage;
use amm_ui::text::AmmPacks;
use levi_i18n::message::Message;
use levi_i18n::service::{LocaleSetting, LocalizationService};

#[test]
fn every_message_has_text_and_a_unique_key() {
    let mut keys = HashSet::new();
    for message in AmmMessage::ALL {
        assert!(!message.msgid().trim().is_empty(), "{message:?}");
        assert!(keys.insert((message.context(), message.msgid())), "{message:?} repeats a key");
    }
}

#[test]
fn index_is_the_position_in_all() {
    for (position, message) in AmmMessage::ALL.iter().enumerate() {
        assert_eq!(message.index(), position, "{message:?}");
    }
}

#[test]
fn placeholders_match_between_singular_and_plural() {
    let placeholders = |text: &'static str| -> BTreeSet<&'static str> {
        text.split('{').skip(1).filter_map(|part| part.split_once('}').map(|(name, _)| name)).collect()
    };
    for message in AmmMessage::ALL {
        if let Some(plural) = message.plural() {
            assert_eq!(placeholders(message.msgid()), placeholders(plural), "{message:?}");
        }
    }
}

#[test]
fn no_glyph_separator_in_a_message() {
    for message in AmmMessage::ALL {
        for glyph in ['\u{b7}', '\u{203a}'] {
            assert!(!message.msgid().contains(glyph), "{message:?} holds a separator glyph");
        }
    }
}

#[test]
fn pseudo_pack_covers_every_message() {
    let localizer = LocalizationService::new(vec![(AmmMessage::DOMAIN, AmmPacks::SOURCE)]).load(&LocaleSetting::Pseudo);
    let table = localizer.table::<AmmMessage>();
    for message in AmmMessage::ALL {
        assert_ne!(table.text(*message), message.msgid(), "{message:?} has no pseudo text");
    }
}
