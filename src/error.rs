use hyper::error::Error as HyperError;
use openssl::error::ErrorStack;
use std::convert::From;
use std::error;
use std::fmt;
use std::io;
use url::ParseError;

#[derive(Debug)]
pub enum UploadError {
    Generic(String),
    HTTP(HyperError),
    IO(io::Error),
    SSL(ErrorStack),
    URLError(ParseError),
}

impl fmt::Display for UploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UploadError::Generic(ref msg) => write!(f, "error uploading an image: {}", msg),
            UploadError::HTTP(ref msg) => write!(f, "error uploading an image: {}", msg),
            UploadError::IO(ref err) => write!(f, "{}", err),
            UploadError::SSL(ref err) => write!(f, "SSL error occurred: {}", err),
            UploadError::URLError(ref err) => write!(f, "URL parsing error: {}", err),
        }
    }
}

impl error::Error for UploadError {
    fn description(&self) -> &str {
        match *self {
            UploadError::Generic(ref msg) => &msg,
            UploadError::HTTP(ref err) => err.description(),
            UploadError::IO(ref err) => err.description(),
            UploadError::SSL(ref err) => err.description(),
            UploadError::URLError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            UploadError::Generic(_) => None,
            UploadError::HTTP(ref err) => err.cause(),
            UploadError::IO(ref err) => err.cause(),
            UploadError::SSL(ref err) => err.cause(),
            UploadError::URLError(ref err) => err.cause(),
        }
    }
}

impl From<&'static str> for UploadError {
    fn from(orig: &str) -> Self {
        UploadError::Generic(orig.to_owned())
    }
}

impl From<ErrorStack> for UploadError {
    fn from(orig: ErrorStack) -> Self {
        UploadError::SSL(orig)
    }
}

impl From<ParseError> for UploadError {
    fn from(orig: ParseError) -> Self {
        UploadError::URLError(orig)
    }
}

impl From<HyperError> for UploadError {
    fn from(orig: HyperError) -> Self {
        UploadError::HTTP(orig)
    }
}

impl From<io::Error> for UploadError {
    fn from(orig: io::Error) -> Self {
        UploadError::IO(orig)
    }
}
