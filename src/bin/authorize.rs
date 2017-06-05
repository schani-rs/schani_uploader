extern crate schani_uploader;
extern crate dotenv;

use std::env;
use dotenv::dotenv;
use schani_uploader::common::Platform;
use schani_uploader::fivehundredpx::Client;

pub fn main() {
    dotenv().ok();

    let x500px_key = env::var("X500PX_KEY").expect("X500PX_KEY must be set");
    let x500px_secret = env::var("X500PX_SECRET").expect("X500PX_SECRET must be set");

    let platform = Client::new(x500px_key, x500px_secret);

    platform.authorize();
}