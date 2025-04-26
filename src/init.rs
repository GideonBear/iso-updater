use crate::installation::Installation;
use color_eyre::Result;

pub fn init() -> Result<()> {
    Installation::init()
}
