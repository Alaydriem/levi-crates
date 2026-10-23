use std::path::Path;

use levi_i18n::message::Message;
use levi_i18n::service::{LocaleSetting, LocalizationService};
use levi_i18n::text::Text;
use levi_ui::LeviUiMessage;
use levi_ui::text::LeviUiPacks;

#[test]
fn every_pack_file_is_listed() {
    let folder = Path::new(env!("CARGO_MANIFEST_DIR")).join("i18n").join("packs");
    let mut on_disk: Vec<String> = std::fs::read_dir(&folder)
        .unwrap_or_else(|error| panic!("{}: {error}", folder.display()))
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let path = entry.path();
            (path.extension()? == "json").then(|| path.file_stem()?.to_str().map(str::to_owned))?
        })
        .collect();
    on_disk.sort();
    assert_eq!(LeviUiPacks::SOURCE.locales(), on_disk);
}

#[test]
fn pseudo_pack_resolves_a_widget_word() {
    let service = LocalizationService::new(vec![(LeviUiMessage::DOMAIN, LeviUiPacks::SOURCE)]);
    let localizer = service.load(&LocaleSetting::Pseudo);
    let close = Text::message(LeviUiMessage::Close);
    let resolved = close.resolve(&localizer);
    assert_ne!(resolved, "Close");
    assert!(resolved.starts_with('['), "{resolved:?} is not the pseudo form");
}
