use std::error::Error;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug)]
pub struct UIError {
    pub message: String,
}

impl Display for UIError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}


#[derive(Debug)]
pub enum EncryptionErrorType {
    WrongPassword,
    FileNotFound,
    GenericError,
}

#[derive(Error, Debug)]
pub struct EncryptionError {
    pub message: String,
    pub error_type: EncryptionErrorType,
}

impl EncryptionError {
    pub fn new(msg: String, err_type: EncryptionErrorType) -> Self {
        EncryptionError {
            message: msg,
            error_type: err_type,
        }
    }
}

impl Display for EncryptionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "Encryption Error occured: message: {}",
                self.message
            ).as_str()
        )
    }
}

impl Error for EncryptionError {}
