extern crate hyper;
extern crate hyper_openssl;
extern crate url;
extern crate oauthcli;

use self::hyper::Client as HyperClient;
use self::hyper::net::HttpsConnector;
use self::hyper::method::Method;
use self::hyper_openssl::OpensslClient;
use self::url::form_urlencoded;
use std::borrow::Cow;
use std::io;
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

#[derive(Debug)]
struct OAuthRequestToken<'a> {
    value: Cow<'a, str>,
    secret: Cow<'a, str>,
}

impl Client {
    pub fn new(key: String, secret: String) -> Client {
        Client {
            consumer_key: key,
            consumer_secret: secret,
        }
    }

    fn get_oauth_request_token(&self, client: &HyperClient) -> Result<OAuthRequestToken, String> {
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
        let mut hash_query: HashMap<String, String> = parsed.into_owned().collect();
        assert!(hash_query.contains_key("oauth_token_secret"));
        assert!(hash_query.contains_key("oauth_token"));
        let req_token: String = hash_query.remove("oauth_token").expect("");
        let req_token_secret: String = hash_query.remove("oauth_token_secret").expect("");
        Ok(OAuthRequestToken {
               value: req_token.into(),
               secret: req_token_secret.into(),
           })
    }

    fn authorize(&self, client: &HyperClient) -> Result<(OAuthRequestToken, String), String> {
        let request_token = self.get_oauth_request_token(client)?;

        let mut authorize_url = url::Url::parse("https://api.500px.com/v1/oauth/authorize")
            .unwrap();
        authorize_url
            .query_pairs_mut()
            .append_pair("oauth_token", &request_token.value);
        println!("Please visit {} and grant access to your account.",
                 authorize_url);

        let mut verifier = String::new();

        io::stdin()
            .read_line(&mut verifier)
            .expect("Failed to read line");


        Ok((request_token, verifier))
    }

    fn get_oauth_access_token(&self, client: &HyperClient) -> Result<(), String> {
        let (request_token, verifier) = self.authorize(client)?;

        println!("Got verifier {}", verifier);

        let url = url::Url::parse("https://api.500px.com/v1/oauth/access_token").unwrap();
        let oauth_header = OAuthAuthorizationHeaderBuilder::new("POST",
                                                                &url,
                                                                self.consumer_key.to_owned(),
                                                                self.consumer_secret.to_owned(),
                                                                SignatureMethod::HmacSha1)
                .token(request_token.value, request_token.secret)
                .verifier(verifier.trim())
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

        println!("Status: {}", resp.status);
        println!("Body: {}", resp_body);

        Ok(())
    }
}

impl Platform for Client {
    fn upload(&self, _: &Photo) {
        let ssl = OpensslClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = HyperClient::with_connector(connector);

        self.get_oauth_access_token(&client)
            .expect("could not get an OAuth access token");
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
