use crate::init::init;
use crate::update::update;
use clap::Parser;
use color_eyre::Result;

#[derive(Parser, Debug)]
#[command(name = "iso-updater", author, long_version = crate::VERSION)]
#[command(about = "Update your ISO files and synchronize them with your USB drive")]
#[command(propagate_version = true)]
struct Cli {
    /// Initialize your ~/.isos directory
    #[arg(long)]
    init: bool,
}

pub fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.init { init() } else { update() }
}
