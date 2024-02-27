//! Known workspace paths resolution

use std::path::{Path, PathBuf};

use chrono_humanize::{Accuracy, HumanTime, Tense};
use solana_program_test::{find_file, read_file};

/// Copied from https://stackoverflow.com/a/74942075/5057425
pub fn workspace_root_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}

/// Returns `/path/to/workspace/root/test-fixtures`
pub fn test_fixtures_dir() -> PathBuf {
    workspace_root_dir().join("test-fixtures")
}

/// Loads + logs a compiled .so BPF program from file and returns the file contents.
///
/// Works the same as [`solana_program_test::ProgramTest::add_program`]
///
/// ## Panics
/// If unable to find .so file
pub fn load_program_so(program_name: &str) -> Vec<u8> {
    let so_file = format!("{program_name}.so");
    let program_file = find_file(&so_file)
        .unwrap_or_else(|| panic!("Program file data not available for {program_name}"));
    let so_prog_data = read_file(&program_file);

    // Copied from:
    // https://docs.rs/solana-program-test/latest/src/solana_program_test/lib.rs.html#630-650
    log::info!(
        "\"{}\" SBF program from {}{}",
        program_name,
        program_file.display(),
        std::fs::metadata(&program_file)
            .map(|metadata| {
                metadata
                    .modified()
                    .map(|time| {
                        format!(
                            ", modified {}",
                            HumanTime::from(time).to_text_en(Accuracy::Precise, Tense::Past)
                        )
                    })
                    .ok()
            })
            .ok()
            .flatten()
            .unwrap_or_default()
    );

    so_prog_data
}
