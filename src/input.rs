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
    #[arg(short = 'd', long = "dir", default_value = ".")]
    dir: PathBuf,

    /// Graph engine
    #[arg(
        short = 'e',
        long = "engine",
        default_value = "neato",
        help = "Which eninge should be used for the graph generation:\n- neato f<300(small)\n- fdp f<2000 (medium)\n- sfdp f>2000 (large)\n- twopi (centered approach, radial generation)\n"
    )]
    engine: String,
}

pub fn parse_input() -> (String, PathBuf, String) {
    let mut arg = Args::parse();

    // Check for supported languages is in the match statement of main
    if !arg.dir.exists() {
        eprintln!("The directory does not exist!");
        std::process::exit(1);
    }

    match arg.engine.as_str() {
        "neato" | "sfdp" | "twopi" | "fdp" => (),
        _ => arg.engine = String::from("neato"),
    };
    (arg.lang, arg.dir, arg.engine)
}
