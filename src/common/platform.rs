use super::super::error::UploadError;

use std::io;
use std::result::Result;

pub trait Platform {
    fn authorize(&self);

    fn upload(&self, user_id: i32, image_stream: &mut io::Read) -> Result<(), UploadError>;
}
