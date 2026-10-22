//! Writes `i18n/levi-ui.pot` from `LeviUiMessage`. With `--check`, writes nothing and fails when
//! the file on disk is out of date.

use std::path::PathBuf;
use std::process::ExitCode;

use levi_i18n::message::Message;
use levi_i18n_tool::error::ToolError;
use levi_i18n_tool::pot::PotWriter;
use levi_ui::LeviUiMessage;

struct I18nExtract;

impl I18nExtract {
    fn run() -> ExitCode {
        let check = std::env::args().any(|arg| arg == "--check");
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("i18n")
            .join(format!("{}.pot", LeviUiMessage::DOMAIN.name()));
        match PotWriter::write::<LeviUiMessage>(&path, check) {
            Ok(true) if check => {
                eprintln!("{}", ToolError::StalePot { path });
                ExitCode::FAILURE
            }
            Ok(_) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("{error}");
                ExitCode::FAILURE
            }
        }
    }
}

fn main() -> ExitCode {
    I18nExtract::run()
}
