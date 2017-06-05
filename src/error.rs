use std::error;
use std::fmt;

#[derive(Debug)]
pub enum UploadError {}

impl fmt::Display for UploadError {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match *self {
        }
    }
}

impl error::Error for UploadError {
    fn description(&self) -> &str {
        match *self {
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
        }
    }
}
