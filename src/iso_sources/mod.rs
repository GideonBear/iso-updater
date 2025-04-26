mod constant_url;
mod linux_mint;

use crate::iso_file::IsoFile;
use crate::iso_source::IsoSource;
use crate::iso_sources::constant_url::ConstantUrl;
use crate::iso_sources::linux_mint::LinuxMint;
use serde::{Deserialize, Serialize};
use tempdir::TempDir;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum IsoSourceEnum {
    // TODO: https://distrowatch.com/dwres.php?resource=bittorrent
    // TODO: Windows from massgrave.dev
    ConstantUrl(ConstantUrl),
    LinuxMint(LinuxMint),
}

impl IsoSourceEnum {
    /// Returns the latest version of the IsoFile.
    pub fn latest(&self, temp: &TempDir) -> color_eyre::Result<IsoFile> {
        match self {
            IsoSourceEnum::ConstantUrl(constant_url) => constant_url.latest(temp),
            IsoSourceEnum::LinuxMint(linux_mint) => linux_mint.latest(temp),
        }
    }

    /// Returns a new IsoFile if there is an update available, None otherwise.
    pub fn updated(
        &self,
        existing: &IsoFile,
        temp: &TempDir,
    ) -> color_eyre::Result<Option<IsoFile>> {
        match self {
            IsoSourceEnum::ConstantUrl(constant_url) => constant_url.updated(existing, temp),
            IsoSourceEnum::LinuxMint(linux_mint) => linux_mint.updated(existing, temp),
        }
    }
}
