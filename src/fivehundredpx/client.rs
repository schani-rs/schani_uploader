use hyper::Client as HyperClient;
use hyper::net::HttpsConnector;
use hyper_openssl::OpensslClient;
use std::io;
use std::option::Option;
use super::oauth::*;
use super::upload::upload_image;
use super::super::error::UploadError;

use super::super::common::Platform;

#[derive(Debug)]
pub struct Client {
    client: HyperClient,
    consumer_key: String,
    consumer_secret: String,
    token: Option<String>,
    token_secret: Option<String>,
}

impl Client {
    pub fn new(
        key: String,
        secret: String,
        token: Option<String>,
        token_secret: Option<String>,
    ) -> Client {
        let ssl = OpensslClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = HyperClient::with_connector(connector);

        Client {
            client: client,
            consumer_key: key,
            consumer_secret: secret,
            token: token,
            token_secret: token_secret,
        }
    }
}

impl Platform for Client {
    fn authorize(&self) {
        let (request_token, authorize_url) =
            authorize(&self.client, &self.consumer_key, &self.consumer_secret)
                .expect("could not authorize");

        println!(
            "Please visit {} and grant access to your account.",
            authorize_url
        );

        let mut verifier = String::new();
        io::stdin().read_line(&mut verifier).expect(
            "Failed to read line",
        );

        get_oauth_access_token(
            &self.client,
            &self.consumer_key,
            &self.consumer_secret,
            &request_token,
            &verifier,
        ).expect("could not get access token");
    }

    fn upload(&self, mut image_stream: &mut io::Read) -> Result<(), UploadError> {
        let token = try!(self.token.as_ref().ok_or("token required for upload"));
        let token_secret = try!(self.token_secret.as_ref().ok_or(
            "token_secret required for upload",
        ));

        upload_image(
            &self.consumer_key,
            &self.consumer_secret,
            &token,
            &token_secret,
            &mut image_stream,
        )
    }
}
