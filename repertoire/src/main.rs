mod opening;

use clap::Parser;
use std::path::PathBuf;
use opening::{Opening, OpeningError};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Load and study an existing opening file
    #[arg(short = 's', long = "study", value_name = "FILENAME")]
    study: Option<String>,

    /// Start a new opening study session
    #[arg(short = 'n', long = "new")]
    new: bool,
}

fn main() {
    let args = Args::parse();

    match (args.study, args.new) {
        (Some(filename), false) => {
            let path = PathBuf::from(filename);
            match Opening::from_file(&path) {
                Ok(opening) => {
                    if let Err(e) = opening.study_loop() {
                        eprintln!("Error during study session: {}", e);
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error loading opening file: {}", e);
                    std::process::exit(1);
                }
            }
        }
        (None, true) => {
            println!("Starting new opening study session");
            // TODO: Implement new study session logic
        }
        (Some(_), true) => {
            eprintln!("Error: Cannot specify both --study and --new options");
            std::process::exit(1);
        }
        (None, false) => {
            eprintln!("Error: Must specify either --study or --new option");
            std::process::exit(1);
        }
    }
}
