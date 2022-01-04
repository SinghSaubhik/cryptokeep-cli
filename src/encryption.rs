//! This module is used for encryption services

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use anyhow::{Result};
use base64::{encode, decode};
use rand::{RngCore, thread_rng};

pub trait Encryptor {
    fn encrypt(&self, plaintext: &str) -> Result<String>;
    fn decrypt(&self, cipher_text: &str) -> Result<String>;
}

pub struct EncryptionProvider {
    key: String,
}

impl EncryptionProvider {
    pub fn new(key: String) -> EncryptionProvider {
        EncryptionProvider { key }
    }

    fn get_cipher(&self) -> Aes256Gcm {
        let key = Key::from_slice(self.key.as_bytes());
        let cipher = Aes256Gcm::new(key);
        cipher
    }
}

impl Encryptor for EncryptionProvider {
    fn encrypt(&self, plaintext: &str) -> Result<String> {
        let cipher: Aes256Gcm = self.get_cipher();
        let mut random_bytes = [0u8; 12];
        thread_rng().fill_bytes(&mut random_bytes);

        println!("{}", &random_bytes.len());

        let nonce = Nonce::from_slice(&random_bytes);

        let encrypted = cipher.encrypt(nonce, plaintext.as_ref())
            .expect("Error while encrypting");

        let result = format!("{}:{}", encode(nonce), encode(encrypted));
        Ok(result)
    }

    fn decrypt(&self, cipher_text: &str) -> Result<String> {
        let split_data: Vec<&str> = cipher_text.split(":").collect();
        let nonce_vec = decode(&split_data[0])?;
        let nonce = Nonce::from_slice(nonce_vec.as_slice());
        let decoded_cipher = decode(&split_data[1])?;

        // let key = Key::from_slice(self.key.as_bytes());
        // let cipher = Aes256Gcm::new(key);
        let cipher: Aes256Gcm = self.get_cipher();
        let decrypted_data = cipher.decrypt(
            nonce,
            decoded_cipher.as_slice(),
        ).expect("Unable to decrypt");

        Ok(String::from_utf8(decrypted_data)?)
    }
}
