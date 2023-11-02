use clap::Parser;

use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[clap(long = "lua", help = "Path to lua program to be executed")]
    pub lua: PathBuf,
    #[clap(long = "version", short = 'V', help = "Print version info and exit")]
    pub version: bool,
}
