use crate::data::Data;
use color_eyre::Result;
use color_eyre::eyre::{OptionExt, eyre};
use dirs::home_dir;
use std::path::{Path, PathBuf};

pub struct Installation {
    directory: PathBuf,
    data: Data,
}

impl Installation {
    pub fn get_and_use<F>(f: F) -> Result<()>
    where
        F: FnOnce(&mut Data) -> Result<()>,
    {
        let mut installation = Installation::get()?;

        f(&mut installation.data)?;

        installation.write()?;

        Ok(())
    }

    pub fn init() -> Result<()> {
        let directory = Self::get_directory()?;

        let data = Data::init();
        let installation = Self { directory, data };
        installation.write()?;

        Ok(())
    }

    fn get_directory() -> Result<PathBuf> {
        Ok(home_dir()
            .ok_or_eyre("Could not get home directory")?
            .join(".isos"))
    }

    fn get() -> Result<Self> {
        let directory = Self::get_directory()?;

        if !directory.exists() {
            return Err(eyre!(
                "~/.isos does not exist. Run `iso-updater --init` first!"
            ));
        }

        let data = Data::from_file(&Self::data_path(&directory))?;

        Ok(Self { directory, data })
    }

    fn write(self) -> Result<()> {
        self.data.to_file(&Self::data_path(&self.directory))
    }

    fn data_path(directory: &Path) -> PathBuf {
        directory.join("data.json")
    }
}
