//! Main file runing kipt.
//!
use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::Read;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

mod account;
mod args;
mod call;
mod declare;
mod deploy;
mod error;
mod invoke;
mod logger;
mod lua;
mod transaction;

const VERSION_STRING: &str = env!("CARGO_PKG_VERSION");

/// Runs main Kipt program.
fn main() -> Result<()> {
    init_tracing();

    let args = args::Args::parse();

    if args.version {
        println!("{}", VERSION_STRING);
        return Ok(());
    }

    if let Some(lua) = &args.lua {
        let program = load_file(&lua.to_string_lossy())?;
        Ok(lua::execute(&program)?)
    } else {
        // Help will be printed out by Args.
        Ok(())
    }
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

/// Initializes tracing.
fn init_tracing() {
    tracing_log::LogTracer::init().expect("Setting log tracer failed.");
    let env_filter = EnvFilter::from_default_env();
    let fmt_layer = fmt::layer();

    let subscriber = Registry::default().with(env_filter).with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed.");
}
