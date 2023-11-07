use clap::Parser;

use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help(true))]
pub struct Args {
    #[clap(help = "Path to lua program to be executed")]
    pub lua: Option<PathBuf>,
    #[clap(long = "version", short = 'V', help = "Print version info and exit")]
    pub version: bool,
}
