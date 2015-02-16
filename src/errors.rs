use std::error::Error as StdError;
use std::fmt;
use std::old_io;
use std::str;
use rustc_serialize::json;

#[derive(Debug, Clone)]
pub enum BodyErrorCause {
    Utf8Error(str::Utf8Error),
    IoError(old_io::IoError),
    ParserError(json::ParserError),
    DecoderError(json::DecoderError)
}

#[derive(Debug, Clone)]
pub struct BodyError {
    pub detail: String,
    pub cause: BodyErrorCause
}

impl StdError for BodyError {
    fn description(&self) -> &str {
        &self.detail[]
    }
}

impl fmt::Display for BodyError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(formatter)
    }
}