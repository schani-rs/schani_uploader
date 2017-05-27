extern crate hyper;
extern crate hyper_openssl;
extern crate url;
extern crate oauthcli;

use self::hyper::Client as HyperClient;
use self::hyper::net::HttpsConnector;
use self::hyper::method::Method;
use self::hyper_openssl::OpensslClient;
use self::url::form_urlencoded;
use std::io::Read;
use std::fmt;
use std::result::Result;
use std::str::FromStr;
use std::collections::HashMap;

use self::oauthcli::*;
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

    fn get_oauth_request_token(&self,
                               client: &HyperClient)
                               -> Result<HashMap<String, String>, String> {
        let url = url::Url::parse("https://api.500px.com/v1/oauth/request_token").unwrap();
        let oauth_header = OAuthAuthorizationHeaderBuilder::new("POST",
                                                                &url,
                                                                self.consumer_key.to_owned(),
                                                                self.consumer_secret.to_owned(),
                                                                SignatureMethod::HmacSha1)
                .finish()
                .to_string();
        let hdr = OAuthHeader::new(oauth_header);

        let mut resp = client
            .request(Method::Post, url)
            .header(hyper::header::Authorization(hdr))
            .send()
            .unwrap();

        let mut body = vec![];
        resp.read_to_end(&mut body).unwrap();
        let resp_body = String::from_utf8_lossy(&body).into_owned();

        let parsed = form_urlencoded::parse(resp_body.as_bytes());
        let hash_query: HashMap<String, String> = parsed.into_owned().collect();
        assert!(hash_query.contains_key("oauth_token_secret"));
        assert!(hash_query.contains_key("oauth_token"));
        Ok(hash_query)
    }

    fn authorize(&self, client: &HyperClient) -> Result<(), String> {
        let request_token = self.get_oauth_request_token(client);

        let url = url::Url::parse("https://api.500px.com/v1/oauth/authorize").unwrap();
        let mut resp = client.request(Method::Post, url).send().unwrap();

        let mut body = vec![];
        resp.read_to_end(&mut body).unwrap();
        let resp_body = String::from_utf8_lossy(&body).into_owned();

        println!("Auth status: {}", resp.status);
        println!("Auth resp: {}", resp_body);

        Ok(())
    }
}

impl Platform for Client {
    fn upload(&self, _: &Photo) {
        let ssl = OpensslClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = HyperClient::with_connector(connector);

        self.authorize(&client).expect("could not authorize");
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct OAuthHeader {
    val: String,
}

impl OAuthHeader {
    pub fn new(val: String) -> OAuthHeader {
        OAuthHeader { val: val }
    }
}

impl hyper::header::Scheme for OAuthHeader {
    fn scheme() -> Option<&'static str> {
        None
    }

    fn fmt_scheme(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.val)
    }
}

impl FromStr for OAuthHeader {
    type Err = hyper::error::Error;
    fn from_str(_: &str) -> Result<OAuthHeader, hyper::error::Error> {
        Err(hyper::error::Error::Header)
    }
}
