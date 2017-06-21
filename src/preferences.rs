use std::error::{self, Error};
use std::fmt;

use hyper;
use serde_json;
use serde_urlencoded;

#[derive(Debug)]
pub struct PreferencesError {}

impl error::Error for PreferencesError {
    fn description(&self) -> &str {
        "could not save/retrieve user preferences"
    }
}

impl fmt::Display for PreferencesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (*self).description())
    }
}

pub trait PreferenceStore {
    fn save_preference(
        &self,
        user_id: i32,
        key: String,
        value: String,
    ) -> Result<(), PreferencesError>;
    fn get_preference(&self, user_id: i32, key: String)
        -> Result<String, PreferencesError>;
}

pub struct PreferencesStoreBuilder {}

impl PreferencesStoreBuilder {
    pub fn build() -> PreferencesStoreImpl {
        PreferencesStoreImpl {}
    }
}

#[derive(Debug)]
pub struct PreferencesStoreImpl {}

impl PreferenceStore for PreferencesStoreImpl {
    fn save_preference(
        &self,
        user_id: i32,
        key: String,
        value: String,
    ) -> Result<(), PreferencesError> {
        let client = hyper::client::Client::new();

        let body = try!(
            serde_urlencoded::to_string(
                [("key", key.to_string()), ("value", value.to_owned())],
            ).map_err(|_| PreferencesError {})
        );
        let req = client
            .put(&format!("http://userinfo:8000/user/{}/setting", user_id))
            .header(hyper::header::ContentType::form_url_encoded())
            .body(&body);
        try!(req.send().map_err(|_| PreferencesError {}));

        Ok(())
    }

    fn get_preference(
        &self,
        user_id: i32,
        key: String,
    ) -> Result<String, PreferencesError> {
        let client = hyper::client::Client::new();

        let req = client.get(&format!(
            "http://userinfo:8000/user/{}/setting/{}",
            user_id,
            key
        ));
        let resp: hyper::client::Response = try!(req.send().map_err(|_| PreferencesError {}));

        println!("status: {}", resp.status);
        // TODO: implement different status for "not found" and "error" for userinfo service
        // and distinguish here accordingly for cleaner error handling/propagation
        if resp.status == hyper::Ok {
            let val: serde_json::Value = try!(serde_json::from_reader(resp).map_err(
                |_| PreferencesError {},
            ));
            return Ok(val["value"].as_str().unwrap().to_string());
        }
        Err(PreferencesError {})
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn get_nonexistent_preference() {
        let store = PreferencesStoreBuilder::build();

        assert!(
            store
                .get_preference(1, "i do not exist".to_string())
                .is_err()
        );
    }

    #[test]
    #[ignore]
    fn create_and_get_preference() {
        let store = PreferencesStoreBuilder::build();

        assert!(
            store
                .save_preference(1, "test".to_string(), "hello".to_string())
                .is_ok()
        );

        let result = store.get_preference(1, "test".to_string());
        assert!(result.is_ok());
    }
}
