use clap::Parser;
use std::path::PathBuf;

/// Program that looks through a given directory and analyzes with the given language which functions use which.
/// Version one requires a filename too, until it is capable of going through the direcotry
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Language
    #[arg(short = 'l', long = "lang")]
    lang: String,

    /// Directory
    #[arg(short = 'd', long = "dir")]
    dir: PathBuf,
}

pub fn parse_input() -> (String, PathBuf) {
    let arg = Args::parse();

    // Check for supported languages is in the match statement of main
    if !arg.dir.exists() {
        eprintln!("The directory does not exist!");
        std::process::exit(1);
    }
    (arg.lang, arg.dir)
}
