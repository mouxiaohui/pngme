use std::process;

use args::PngMeArgs;
use clap::Parser;
use commands::run_cmd;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    if let Err(err) = run_cmd(PngMeArgs::parse()) {
        eprintln!("Error: {}", err);
    };

    Ok(())
}
