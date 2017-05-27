
use std::path::Path;

#[derive(Debug)]
pub struct Photo {}

impl Photo {
    pub fn load<P: AsRef<Path>>(_: P) -> Result<Photo, &'static str> {
        Ok(Photo {})
    }
}
