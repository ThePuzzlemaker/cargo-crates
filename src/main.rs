#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};
use crates_io_api::{Error as CrateError, SyncClient};

fn main() {
    let matches = App::new("cargo-crates")
        .about("A cargo command to quickly open the crates.io or docs.rs page for the latest version of a crate")
        .version(&crate_version!()[..])
        .bin_name("cargo")
        .subcommand(SubCommand::with_name("crates")
            .about("A cargo command to quickly open the crates.io or docs.rs page for the latest version of a crate")
            .arg(Arg::with_name("CRATE")
                .help("The name of the crate to open the page for")
                .required(true)
                .index(1))
            .arg(Arg::with_name("doc")
                .short("d")
                .long("doc")
                .help("Open the docs.rs page instead of the crates.io page"))
        )
        .settings(&[AppSettings::SubcommandRequired])
        .get_matches();

    let sub_matches = matches.subcommand_matches("crates").unwrap();

    let crate_name = sub_matches.value_of("CRATE").unwrap();
    let open_doc = sub_matches.is_present("doc");

    // Check if the crate name follows crates.io's regulations (excluding reserved names for windows)
    let under_max_length = crate_name.chars().take(65).count() <= 64;
    let first_alphabetic = crate_name
        .chars()
        .next()
        .map(char::is_alphabetic)
        .unwrap_or(false);
    let not_empty = !crate_name.is_empty();
    let all_ascii_alphanum = crate_name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    if !(not_empty && all_ascii_alphanum && first_alphabetic && under_max_length) {
        eprintln!(
            "error: crate name '{}' is not valid for crates.io",
            crate_name
        );
        std::process::exit(-1);
    }

    let exists = match check_crate_exists(crate_name) {
        Ok(e) => e,
        Err(e) => {
            eprintln!(
                "error: failed to lookup information for crate '{}': {}",
                crate_name, e
            );
            std::process::exit(-1);
        }
    };

    if !exists {
        eprintln!("error: the crate '{}' does not exist.", crate_name);
        std::process::exit(-1);
    }

    if open_doc {
        match opener::open(format!("https://docs.rs/crates/{}", crate_name)) {
            Err(e) => {
                eprintln!("error: failed to open link: {}", e);
                std::process::exit(-1);
            }
            Ok(()) => {}
        }
    } else {
        match opener::open(format!("https://crates.io/crates/{}", crate_name)) {
            Err(e) => {
                eprintln!("error: failed to open link: {}", e);
                std::process::exit(-1);
            }
            Ok(()) => {}
        }
    }
}

fn check_crate_exists(name: &str) -> Result<bool, CrateError> {
    let client = SyncClient::new(
        "cargo-crates (github.com/ThePuzzlemaker/cargo-crates)",
        std::time::Duration::from_millis(2000),
    )?;

    match client.get_crate(name) {
        Err(CrateError::NotFound(_)) => Ok(false),
        Err(e) => Err(e),
        _ => Ok(true),
    }
}
