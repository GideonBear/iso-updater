use crate::iso_file::IsoFile;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tempdir::TempDir;

pub trait IsoSource<'a>: Debug + Clone + Deserialize<'a> + Serialize {
    /// Returns the latest version of the IsoFile.
    fn latest(&self, temp: &TempDir) -> Result<IsoFile>;

    /// Returns a new IsoFile if there is an update available, None otherwise.
    fn updated(&self, existing: &IsoFile, temp: &TempDir) -> Result<Option<IsoFile>>;
}
