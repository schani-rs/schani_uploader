use super::Photo;
use super::super::error::UploadError;
use std::result::Result;

pub trait Platform {
    fn authorize(&self);

    fn upload(&self, photo: &Photo) -> Result<(), UploadError>;
}
