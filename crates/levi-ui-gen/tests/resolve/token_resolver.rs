use levi_ui_gen::error::GenError;
use levi_ui_gen::model::{Owner, OwnershipFile, TokenFile};
use levi_ui_gen::resolve::TokenResolver;
use levi_ui_gen::value::TokenValue;

const TOKENS: &str = r##"{"tokens":[
  {"name":"--color-success","type":"color","value":"#4FD1A5"},
  {"name":"--color-track-camera","type":"color","value":"#FFB547"},
  {"name":"--color-scene-camera","type":"color","value":"var(--color-track-camera)"},
  {"name":"--color-mode-preview","type":"color","value":"var(--color-scene-camera)"},
  {"name":"--color-mode-edit","type":"color","value":"var(--color-success)"}
]}"##;

const OWNERS: &str = r#"{"owners":{
  "--color-success":"levi-ui",
  "--color-track-camera":"studio-ui",
  "--color-scene-camera":"studio-ui",
  "--color-mode-preview":"studio-ui",
  "--color-mode-edit":"studio-ui"
}}"#;

fn tokens(json: &str) -> TokenFile {
    serde_json::from_str(json).unwrap_or_else(|error| panic!("token fixture: {error}"))
}

fn owners(json: &str) -> OwnershipFile {
    serde_json::from_str(json).unwrap_or_else(|error| panic!("ownership fixture: {error}"))
}

#[test]
fn alias_chain_stays_reference() {
    let resolved = TokenResolver::resolve(&tokens(TOKENS), &owners(OWNERS)).unwrap();
    let preview = resolved.iter().find(|token| token.entry.name == "--color-mode-preview").unwrap();
    assert_eq!(preview.value, TokenValue::Alias("--color-scene-camera".into()));
    assert_eq!(preview.owner, Owner::StudioUi);
    let edit = resolved.iter().find(|token| token.entry.name == "--color-mode-edit").unwrap();
    assert_eq!(edit.value, TokenValue::Alias("--color-success".into()));
}

#[test]
fn general_token_cannot_alias_studio_token() {
    let json = r##"{"tokens":[
      {"name":"--color-track-camera","type":"color","value":"#FFB547"},
      {"name":"--color-accent","type":"color","value":"var(--color-track-camera)"}
    ]}"##;
    let own = r#"{"owners":{"--color-track-camera":"studio-ui","--color-accent":"levi-ui"}}"#;
    match TokenResolver::resolve(&tokens(json), &owners(own)) {
        Err(GenError::WrongDirectionAlias { token, target }) => {
            assert_eq!(token, "--color-accent");
            assert_eq!(target, "--color-track-camera");
        }
        other => panic!("expected WrongDirectionAlias, got {other:?}"),
    }
}

#[test]
fn missing_alias_target() {
    let json = r#"{"tokens":[{"name":"--color-accent","type":"color","value":"var(--color-gone)"}]}"#;
    let own = r#"{"owners":{"--color-accent":"levi-ui"}}"#;
    match TokenResolver::resolve(&tokens(json), &owners(own)) {
        Err(GenError::MissingAlias { token, target }) => {
            assert_eq!(token, "--color-accent");
            assert_eq!(target, "--color-gone");
        }
        other => panic!("expected MissingAlias, got {other:?}"),
    }
}

#[test]
fn an_alias_to_another_type_is_missing() {
    let json = r##"{"tokens":[
      {"name":"--spacing-row","type":"spacing","value":"28px"},
      {"name":"--color-accent","type":"color","value":"var(--spacing-row)"}
    ]}"##;
    let own = r#"{"owners":{"--spacing-row":"levi-ui","--color-accent":"levi-ui"}}"#;
    assert!(matches!(TokenResolver::resolve(&tokens(json), &owners(own)), Err(GenError::MissingAlias { .. })));
}

#[test]
fn an_alias_cycle_is_an_error() {
    let json = r#"{"tokens":[
      {"name":"--color-a","type":"color","value":"var(--color-b)"},
      {"name":"--color-b","type":"color","value":"var(--color-a)"}
    ]}"#;
    let own = r#"{"owners":{"--color-a":"levi-ui","--color-b":"levi-ui"}}"#;
    assert!(matches!(TokenResolver::resolve(&tokens(json), &owners(own)), Err(GenError::AliasCycle { .. })));
}

#[test]
fn unowned_tokens_are_listed() {
    let own = r#"{"owners":{"--color-success":"levi-ui","--color-track-camera":"studio-ui","--color-scene-camera":"studio-ui"}}"#;
    match TokenResolver::resolve(&tokens(TOKENS), &owners(own)) {
        Err(GenError::Unowned { tokens }) => assert_eq!(tokens, vec!["--color-mode-edit", "--color-mode-preview"]),
        other => panic!("expected Unowned, got {other:?}"),
    }
}

#[test]
fn a_name_that_does_not_match_its_type_is_rejected() {
    let json = r#"{"tokens":[{"name":"--spacing-row","type":"container","value":"28px"}]}"#;
    let own = r#"{"owners":{"--spacing-row":"levi-ui"}}"#;
    match TokenResolver::resolve(&tokens(json), &owners(own)) {
        Err(GenError::BadName { token }) => assert_eq!(token, "--spacing-row"),
        other => panic!("expected BadName, got {other:?}"),
    }
}

#[test]
fn a_duplicate_name_is_rejected() {
    let json = r##"{"tokens":[
      {"name":"--color-a","type":"color","value":"#000000"},
      {"name":"--color-a","type":"color","value":"#FFFFFF"}
    ]}"##;
    let own = r#"{"owners":{"--color-a":"levi-ui"}}"#;
    assert!(matches!(TokenResolver::resolve(&tokens(json), &owners(own)), Err(GenError::BadName { .. })));
}
