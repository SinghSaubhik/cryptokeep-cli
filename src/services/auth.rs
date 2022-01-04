// This modules contains authentication related stuff

use anyhow::Result;
use base64::encode;
use sha2::{Sha512, Digest};
use crate::{get_component_path, KeyFileData};
use crate::constants::PASSWORD_FILE_NAME;

pub trait Authenticator {
    fn authenticate(&self) -> Result<bool>;
}

pub struct Auth {}

impl Auth {
    pub fn new() -> Self {
        Self {}
    }

    pub fn match_user_password(user_pass: &str) -> Result<bool> {
        let path = get_component_path(PASSWORD_FILE_NAME)
            .expect("Unable to get the password component");

        let mut hash = Sha512::new();
        hash.update(user_pass.as_bytes());
        let d = hash.finalize();
        let _final = encode(d);

        let saved_pass_hash = std::fs::read_to_string(path).unwrap();

        if saved_pass_hash == _final {
            return Ok(true);
        }

        Ok(false)
    }

    fn get_user_data(&self) -> KeyFileData {
        KeyFileData { hashed_key: "".to_string(), master_key_cipher: "".to_string() }
    }
}

impl Authenticator for Auth {
    fn authenticate(&self) -> Result<bool> { todo!() }
}