#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
extern crate schani_uploader;

use std::env;

use dotenv::dotenv;
use rocket::http::Status;
use rocket::response::status::Custom;
use schani_uploader::common::Platform;
use schani_uploader::fivehundredpx::Client;
use schani_uploader::store;

#[post("/platform/500px/upload/<user_id>/<image_id>")]
fn upload_to_500px(user_id: i32, image_id: i32) -> Result<(), Custom<&'static str>> {
    let x500px_key = env::var("X500PX_KEY").expect("X500PX_KEY must be set");
    let x500px_secret = env::var("X500PX_SECRET").expect("X500PX_SECRET must be set");

    // TODO: load from userinfo preferences
    let x500px_token = env::var("X500PX_TOKEN").map_err(|_| {
        Custom(Status::InternalServerError, "X500PX_TOKEN must be set")
    })?;
    let x500px_token_secret = env::var("X500PX_TOKEN_SECRET").map_err(|_| {
        Custom(
            Status::InternalServerError,
            "X500PX_TOKEN_SECRET must be set",
        )
    })?;

    let platform = Client::new(
        x500px_key,
        x500px_secret,
        Some(x500px_token),
        Some(x500px_token_secret),
    );

    let mut image_stream = try!(store::load_image_file(image_id).map_err(|_| {
        Custom(
            Status::InternalServerError,
            "could not load file from store",
        )
    }));

    if let Err(err) = platform.upload(user_id, &mut image_stream) {
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
