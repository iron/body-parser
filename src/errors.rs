use std::error::Error as StdError;
use std::fmt;
use std::str;

use serde_json;

#[derive(Debug)]
pub enum BodyErrorCause {
    Utf8Error(str::Utf8Error),
    IoError(iron::error::HttpError),
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

    fn cause(&self) -> Option<&StdError> {
        use BodyErrorCause::*;

        match self.cause {
            Utf8Error(ref err) => Some(err),
            IoError(ref err) => Some(err),
            JsonError(ref err) => Some(err),
        }
    }
}

impl fmt::Display for BodyError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(formatter)
    }
}
