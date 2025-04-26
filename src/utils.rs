use color_eyre::Result;
use command_error::CommandExt;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process::Command;

pub fn hash_file(file: &Path) -> Result<String> {
    let mut file = File::open(file)?;
    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    hasher.update(&buffer);

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

pub fn download(url: &str, file: &Path) -> Result<()> {
    Command::new("wget")
        .arg(url)
        .arg("-O")
        .arg(file)
        .status_checked()?;
    Ok(())
}
