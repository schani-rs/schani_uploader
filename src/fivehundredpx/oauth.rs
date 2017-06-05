use hyper::Client;
use hyper::method::Method::Post;
use hyper::header::Authorization;
use oauthcli::*;
use std::borrow::Cow;
use url::Url;
use url::form_urlencoded;
use std::collections::HashMap;
use std::io::Read;
use super::oauth_header::OAuthHeader;

#[derive(Debug)]
pub struct OAuthRequestToken<'a> {
    value: Cow<'a, str>,
    secret: Cow<'a, str>,
}

fn get_oauth_request_token<'a>(client: &Client,
                               consumer_key: &'a String,
                               consumer_secret: &'a String)
                               -> Result<OAuthRequestToken<'a>, String> {
    let url = Url::parse("https://api.500px.com/v1/oauth/request_token").unwrap();
    let oauth_header = OAuthAuthorizationHeaderBuilder::new("POST",
                                                            &url,
                                                            consumer_key.to_string(),
                                                            consumer_secret.to_string(),
                                                            SignatureMethod::HmacSha1)
            .finish()
            .to_string();
    let hdr = OAuthHeader::new(oauth_header);

    let mut resp = client
        .request(Post, url)
        .header(Authorization(hdr))
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

pub fn authorize<'a>(client: &Client,
                     consumer_key: &'a String,
                     consumer_secret: &'a String)
                     -> Result<(OAuthRequestToken<'a>, String), String> {
    let request_token = get_oauth_request_token(client, consumer_key, consumer_secret)?;

    let mut authorize_url = Url::parse("https://api.500px.com/v1/oauth/authorize").unwrap();
    authorize_url
        .query_pairs_mut()
        .append_pair("oauth_token", &request_token.value);

    Ok((request_token, authorize_url.to_string()))
}

pub fn get_oauth_access_token<'a>(client: &Client,
                                  consumer_key: &'a String,
                                  consumer_secret: &'a String,
                                  request_token: &OAuthRequestToken,
                                  verifier: &'a String)
                                  -> Result<(), String> {

    println!("Got verifier {}", verifier);
    let url = Url::parse("https://api.500px.com/v1/oauth/access_token").unwrap();
    let oauth_header = OAuthAuthorizationHeaderBuilder::new("POST",
                                                            &url,
                                                            consumer_key.to_string(),
                                                            consumer_secret.to_string(),
                                                            SignatureMethod::HmacSha1)
            .token(request_token.value.to_owned(),
                   request_token.secret.to_owned())
            .verifier(verifier.trim())
            .finish()
            .to_string();
    let hdr = OAuthHeader::new(oauth_header);

    let mut resp = client
        .request(Post, url)
        .header(Authorization(hdr))
        .send()
        .unwrap();

    let mut body = vec![];
    resp.read_to_end(&mut body).unwrap();
    let resp_body = String::from_utf8_lossy(&body).into_owned();

    println!("Status: {}", resp.status);
    println!("Body: {}", resp_body);

    Ok(())
}