use std::process;

use clap::Parser;
use commands::run;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = args::Cli::parse();

    if let Err(error) = run(&args) {
        eprintln!("An error occurred: {}", error);
        process::exit(1);
    }

    Ok(())
}