use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::str;

use serde_json;

#[derive(Debug)]
pub enum BodyErrorCause {
    Utf8Error(str::Utf8Error),
    IoError(io::Error),
    JsonError(serde_json::Error),
}

#[derive(Debug)]
pub struct BodyError {
    pub detail: String,
    pub cause: BodyErrorCause
}

impl StdError for BodyError {
    fn description(&self) -> &str {
        &self.detail[..]
    }
}

impl fmt::Display for BodyError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(formatter)
    }
}
