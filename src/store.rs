use std::io;

use hyper::Url;
use hyper::client::{Client, Response};

pub fn load_image_file(
    image_id: i32
) -> Result<Box<io::Read>, super::error::UploadError> {
    let client = Client::new();
    let url = try!(Url::parse(&format!(
        "http://store:8000/api/images/{}/file",
        image_id.to_string()
    )));
    let mut resp: Response = try!(client.get(url).send());

    Ok(Box::new(resp))
}
