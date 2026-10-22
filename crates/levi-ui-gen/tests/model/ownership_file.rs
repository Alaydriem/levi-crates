use levi_ui_gen::model::{Owner, OwnershipFile};

const OWNERSHIP: &str = r#"{
  "owners": { "--color-accent": "levi-ui", "--color-scene-1": "studio-ui" },
  "selectors": [
    { "crate": "studio-ui", "name": "SceneColor", "variants": [["Lime", "--color-scene-1"]] }
  ]
}"#;

#[test]
fn reads_owners_and_selectors() {
    let file: OwnershipFile = serde_json::from_str(OWNERSHIP).unwrap();
    assert_eq!(file.owners["--color-accent"], Owner::LeviUi);
    assert_eq!(file.owners["--color-scene-1"], Owner::StudioUi);
    assert_eq!(file.selectors.len(), 1);
    assert_eq!(file.selectors[0].crate_owner, Owner::StudioUi);
    assert_eq!(file.selectors[0].name, "SceneColor");
    assert_eq!(file.selectors[0].variants, vec![("Lime".to_owned(), "--color-scene-1".to_owned())]);
}
