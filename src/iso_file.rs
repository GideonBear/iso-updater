use crate::utils::hash_file;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IsoFile {
    /// The sha256 hash of the file
    pub hash: String,
    /// The version, if any. This may be the same for multiple different files.
    pub version: Option<String>,
}

impl IsoFile {
    pub fn new(file: &Path, version: Option<String>) -> Result<Self> {
        let hash = hash_file(&file)?;
        Ok(Self { hash, version })
    }
}

impl PartialEq<Self> for IsoFile {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Eq for IsoFile {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InPlaceIsoFile {
    pub iso_file: IsoFile,
    /// The name of the file (can be nested inside folders)
    filename: PathBuf,
}

impl InPlaceIsoFile {
    pub fn put(
        iso_file: IsoFile,
        file: PathBuf,
        target_directory: &Path,
        target_filename: PathBuf,
    ) -> Result<Self> {
        let target_file = target_directory.join(&target_filename);
        fs::rename(&file, &target_file)?;

        Ok(Self {
            iso_file,
            filename: target_filename,
        })
    }
}
