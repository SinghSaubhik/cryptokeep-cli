//! This module is used for encryption services

use std::fs;

use aes_gcm::Nonce;
use anyhow::{Error, Result};
use serde_json::Value;
use sha2::{Sha256, Digest};
use sha2::digest::Update;
use crate::{EncryptionError, EncryptionErrorType, KeyFileData};

pub trait Encryptor {
    fn new(user_password: String) -> Self;
    fn encrypt(&mut self, plaintext: String) -> Result<()>;
    fn decrypt(&mut self, cipher_text: String) -> Result<()>;
}

pub struct EncryptionService {
    key: String,
    nonce: String,
}

fn get_api_key(pass: String) -> Vec<u8> {
    Vec::new()
}

fn match_password(user_password: String) -> Result<KeyFileData> {
    let io = fs::read("test.key")?;
    let file_data_value: Value = serde_json::from_slice(io.as_slice())?;

    let file_data = KeyFileData {
        hashed_key: file_data_value["hashed_key"].to_string(),
        master_key_cipher: file_data_value["master_key_cipher"].to_string(),
    };

    let mut sha = Sha256::new();
    sha.update(user_password.as_bytes());
    let hashed_user_password = sha.finalize();

    if file_data.hashed_key != hashed_user_password {
        return Err(
            Error::new(
                EncryptionError::new(
                    String::from("Incorrect password"),
                    EncryptionErrorType::WrongPassword,
                )
            )
        );
    }

    Ok(KeyFileData { hashed_key: "".to_string(), master_key_cipher: "".to_string() })
}

impl Encryptor for EncryptionService {
    fn new(user_password: String) -> Self {
        Self {
            key: String::from(""),
            nonce: String::from(""),
        }
    }


    fn encrypt(&mut self, plaintext: String) -> Result<()> {
        todo!()
    }

    fn decrypt(&mut self, cipher_text: String) -> Result<()> {
        todo!()
    }
}
