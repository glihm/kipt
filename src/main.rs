//! Main file runing kipt.
//!
use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::Read;

mod account;
mod args;
mod declare;
mod deploy;
mod error;
mod invoke;
mod lua;
mod utils;

const VERSION_STRING: &str = env!("CARGO_PKG_VERSION");

/// Runs main Kipt program.
fn main() -> Result<()> {
    let args = args::Args::parse();

    if args.version {
        println!("{}", VERSION_STRING);
        return Ok(());
    }

    let program = load_file(&args.lua.to_string_lossy())?;

    Ok(lua::execute(&program)?)
}

/// Loads a file content as `String`.
///
/// # Arguments
///
/// * `file_path` - Path of the file to be loaded.
fn load_file(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    Ok(file_contents)
}
