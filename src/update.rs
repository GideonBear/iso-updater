use crate::installation::Installation;
use crate::iso_file::InPlaceIsoFile;
use color_eyre::Result;
use tempdir::TempDir;

pub fn update() -> Result<()> {
    let temp = TempDir::new("iso_updater")?;

    Installation::get_and_use(|data| {
        // Update files
        data.sources
            .iter()
            .try_for_each(|(id, source)| match data.files.get(id) {
                Some(file) => match source.updated(&file.iso_file, &temp)? {
                    Some(file) => todo!(),
                    None => {
                        println!("No update found for {id}");
                        Ok(())
                    }
                },
                None => {
                    todo!()
                }
            })?;

        // Update usb's if necessary
        todo!();

        Ok(())
    })
}
