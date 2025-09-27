// Warnings are translated to denys in CI
#![warn(clippy::panic)]
#![warn(clippy::missing_panics_doc)] // Catches other panics (unwrap, expect)

mod cli;
mod data;
mod init;
mod installation;
mod iso_file;
mod iso_source;
mod iso_sources;
mod test;
mod update;
mod utils;

use color_eyre::Result;

fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}
