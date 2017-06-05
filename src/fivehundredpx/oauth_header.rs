use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use hyper::header::Scheme;
use hyper::error::Error;

#[derive(Clone, PartialEq, Debug)]
pub struct OAuthHeader {
    val: String,
}

impl OAuthHeader {
    pub fn new(val: String) -> OAuthHeader {
        OAuthHeader { val: val }
    }
}

impl Scheme for OAuthHeader {
    fn scheme() -> Option<&'static str> {
        None
    }

    fn fmt_scheme(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.val)
    }
}

impl FromStr for OAuthHeader {
    type Err = Error;
    fn from_str(_: &str) -> Result<OAuthHeader, Error> {
        Err(Error::Header)
    }
}