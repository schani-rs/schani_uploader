use hyper::client::Request;
use hyper::header::Authorization;
use hyper::method::Method::Post;
use hyper::net::HttpsConnector;
use hyper::net::Streaming;
use hyper;
use hyper_openssl::OpensslClient;
use multipart::client::Multipart;
use oauthcli::*;
use url::Url;
use std::io::Read;
use super::oauth_header::OAuthHeader;
use super::super::common::Photo;
use super::super::error::UploadError;

pub fn upload_image(consumer_key: &String,
                    consumer_secret: &String,
                    oauth_token: &String,
                    oauth_token_secret: &String,
                    photo: &Photo)
                    -> Result<(), UploadError> {
    let ssl = try!(OpensslClient::new());
    let connector = HttpsConnector::new(ssl);

    let url = try!(Url::parse("https://api.500px.com/v1/photos/upload"));
    let oauth_header = OAuthAuthorizationHeaderBuilder::new("POST",
                                                            &url,
                                                            consumer_key.to_string(),
                                                            consumer_secret.to_string(),
                                                            SignatureMethod::HmacSha1)
            .token(oauth_token.to_string(), oauth_token_secret.to_string())
            .finish()
            .to_string();
    let hdr = OAuthHeader::new(oauth_header);

    let mut request = try!(Request::with_connector(Post, url, &connector));
    request.headers_mut().set(Authorization(hdr));

    let mut multipart = try!(Multipart::from_request(request));
    try!(write_body(&mut multipart, photo));

    let mut resp = try!(multipart.send());

    let mut body = vec![];
    try!(resp.read_to_end(&mut body));
    let resp_body = String::from_utf8_lossy(&body).into_owned();

    println!("Status: {}", resp.status);
    println!("Body: {}", resp_body);
    Ok(())
}

fn write_body(multipart: &mut Multipart<Request<Streaming>>, photo: &Photo) -> hyper::Result<()> {
    try!(multipart.write_file("file", photo.get_path()));
    try!(multipart.write_text("name", "Schani test"));
    multipart.write_text("privacy", "1").map(|_| ())
}
