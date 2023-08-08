#![allow(unused)]

use anyhow::{Context, Result};

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()>{
    let args = Cli::parse();
    let path = args.path.to_str().expect("Path could not be converted to a string");
    let content = std::fs::read_to_string(path)
    .with_context(|| format!("could not read file '{}'", path))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    return Ok(());
}