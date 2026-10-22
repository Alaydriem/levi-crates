use std::sync::Arc;

use levi_i18n::localize::Localizer;
use levi_i18n::message::Message;
use levi_i18n::pack::LanguagePack;
use levi_ui::LeviUiMessage;

// a pseudo-locale localizer for the levi-ui domain, built from (msgid, translation) pairs
pub struct PackFixture;

impl PackFixture {
    pub fn localizer(entries: &[(&str, &str)]) -> Arc<Localizer> {
        let messages: serde_json::Map<String, serde_json::Value> =
            entries.iter().map(|(id, text)| ((*id).to_owned(), serde_json::Value::from(*text))).collect();
        let json = serde_json::json!({ "v": 1, "locale": "en_XA", "plural": ["one", "other"], "m": messages });
        let pack: LanguagePack =
            serde_json::from_value(json).unwrap_or_else(|error| panic!("fixture pack is invalid: {error}"));
        Arc::new(Localizer::new("en_XA", vec![(LeviUiMessage::DOMAIN, pack)]))
    }
}
