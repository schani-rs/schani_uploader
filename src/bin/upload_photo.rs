extern crate schani_uploader;
extern crate dotenv;

use std::env;
use dotenv::dotenv;
use schani_uploader::common::{Photo, Platform};
use schani_uploader::fivehundredpx::Client;

pub fn main() {
    dotenv().ok();

    let x500px_key = env::var("X500PX_KEY").expect("X500PX_KEY must be set");
    let x500px_secret = env::var("X500PX_SECRET").expect("X500PX_SECRET must be set");

    // TODO: load from userinfo preferences
    let x500px_token = env::var("X500PX_TOKEN").expect("X500PX_TOKEN must be set");
    let x500px_token_secret = env::var("X500PX_TOKEN_SECRET")
        .expect("X500PX_TOKEN_SECRET must be set");

    let platform = Client::new(x500px_key,
                               x500px_secret,
                               Some(x500px_token),
                               Some(x500px_token_secret));
    let photo = Photo::new(&"resources/img1.jpg".to_string(), &"test".to_string());

    if let Err(err) = platform.upload(&photo)  {
        println!("could not upload photo: {}", err);
    }
}