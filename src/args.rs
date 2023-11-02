use clap::Parser;

use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[clap(long = "lua", help = "Lua file to be loaded")]
    lua: PathBuf,
}
