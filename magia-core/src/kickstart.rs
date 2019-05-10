use std::io::Result;
use std::fs;

///
/// Holds the data for the kickstart
///
pub struct Kickstart {
    /// Data for the kickstart loaded into memory
    pub data: Box<[u8]>,
    /// Kickstart version
    _version: u32
}

impl Kickstart {
    pub fn load(filename: &str) -> Result<Kickstart> {
        let data = fs::read(filename)?;

        Ok(Kickstart {
            data: data.into_boxed_slice(),
            _version: 0,
        })
    }
}
