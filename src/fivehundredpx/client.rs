use hyper::Client as HyperClient;
use hyper::net::HttpsConnector;
use hyper_openssl::OpensslClient;
use super::oauth::*;

use super::super::common::{Platform, Photo};

#[derive(Debug)]
pub struct Client {
    consumer_key: String,
    consumer_secret: String,
}

impl Client {
    pub fn new(key: String, secret: String) -> Client {
        Client {
            consumer_key: key,
            consumer_secret: secret,
        }
    }
}

impl Platform for Client {
    fn upload(&self, _: &Photo) {
        let ssl = OpensslClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = HyperClient::with_connector(connector);

        get_oauth_access_token(&client, &self.consumer_key, &self.consumer_secret)
            .expect("could not get an OAuth access token");
    }
}
