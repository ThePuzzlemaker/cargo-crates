#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};

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

    if open_doc {
        let url = match crate_name {
            "alloc" | "core" | "proc_macro" | "std" | "test" => format!("https://docs.rs/{}", crate_name),
            _ => format!("https://docs.rs/{}/*/{0}", crate_name)
        };
        
        match opener::open(url) {
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
