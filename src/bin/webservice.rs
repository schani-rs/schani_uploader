#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
extern crate schani_uploader;

use dotenv::dotenv;
use rocket::response::status::Custom;
use schani_uploader::common::{Photo, Platform};
use schani_uploader::fivehundredpx::Client;
use std::env;

#[post("/platform/500px/upload/<image_id>")]
fn upload_to_500px(image_id: i32) -> Result<(), Custom<&'static str>> {
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

    // TODO: load photo from store
    let image = format!("/schani_resources/img{}.jpg", image_id);
    let photo = Photo::new(&image, &"test".to_string());

    if let Err(err) = platform.upload(&photo) {
        println!("could not upload photo: {}", err);
    }

    Ok(())
}

fn main() {
    dotenv().ok();

    rocket::ignite()
        .mount("/", routes![upload_to_500px])
        .launch();
}
