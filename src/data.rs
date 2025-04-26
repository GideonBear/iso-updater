use crate::iso_file::InPlaceIsoFile;
use crate::iso_sources::IsoSourceEnum;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Data {
    pub sources: HashMap<String, IsoSourceEnum>,
    pub files: HashMap<String, InPlaceIsoFile>,
    // TODO: support non-ventoy USB drives?
    pub usb: HashMap<String, InPlaceIsoFile>,
}

impl Data {
    pub fn init() -> Self {
        Self {
            sources: HashMap::new(),
            files: HashMap::new(),
            usb: HashMap::new(),
        }
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let data = ron::de::from_reader(reader)?;
        Ok(data)
    }

    pub fn to_file(&self, path: &Path) -> Result<()> {
        let file = File::create(path)?;
        let string = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?;
        let mut writer = BufWriter::new(file);
        writer.write_all(string.as_bytes())?;
        Ok(())
    }
}
