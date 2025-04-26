use crate::iso_file::IsoFile;
use crate::iso_source::IsoSource;
use crate::utils::download;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use tempdir::TempDir;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConstantUrl {
    name: String,
    url: String,
    version: Option<String>,
}

impl IsoSource<'_> for ConstantUrl {
    fn latest(&self, temp: &TempDir) -> Result<IsoFile> {
        let file = temp.path().join("download.iso");
        download(&self.url, &file)?;

        Ok(IsoFile::new(&file, self.version.clone())?)
    }

    fn updated(&self, _existing: &IsoFile, _temp: &TempDir) -> Result<Option<IsoFile>> {
        Ok(None)
    }
}
