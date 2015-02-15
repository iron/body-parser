#![crate_name = "bodyparser"]
#![feature(core)]
#![feature(io)]

//! Body Parser Plugin for Iron
//!
//! This plugin parses JSON out of an incoming Request.

extern crate iron;
extern crate "rustc-serialize" as rustc_serialize;
extern crate plugin;
extern crate persistent;

use iron::mime;
use iron::prelude::*;
use iron::headers;
use iron::typemap::{Key};
use std::old_io::ByRefReader;
use persistent::Read;

pub use self::errors::{BodyError};
pub use self::limit_reader::{LimitReader};

mod errors;
mod limit_reader;

fn read_body_as_utf8(req: &mut Request, limit: usize) -> Result<String, errors::BodyError> {
    match LimitReader::new(req.body.by_ref(), limit).read_to_end() {
        Ok(bytes) => {
             match String::from_utf8(bytes) {
                Ok(e) => Ok(e),
                Err(err) => Err(errors::BodyError {
                    detail: "Invalid UTF-8 sequence".to_string(),
                    cause: errors::BodyErrorCause::Utf8Error(err.utf8_error())
                })
            }
        },
        Err(err) => Err(errors::BodyError {
            detail: "Can't read request body".to_string(),
            cause: errors::BodyErrorCause::IoError(err)
        })
    }
}

/// Use this key to modify the default body limit.
pub struct MaxBodyLength;
impl Key for MaxBodyLength {
    type Value = usize;
}

/// BodyReader is a plugin to read a request body into UTF-8 String.
/// Doesn't read `multipart/form-data`.
pub struct BodyReader;

impl Key for BodyReader {
    type Value = Option<String>;
}

const DEFAULT_BODY_LIMIT: usize = 1024 * 1024 * 100;

impl<'a> plugin::Plugin<Request<'a>> for BodyReader {
    type Error = BodyError;

    fn eval(req: &mut Request) -> Result<Option<String>, BodyError> {
        let need_read = req.headers.get::<headers::ContentType>().map(|header| {
            match **header {
                mime::Mime(mime::TopLevel::Multipart, mime::SubLevel::FormData, _) => false,
                _ => true
            }
        }).unwrap_or(false);

        if need_read {
            let max_length = req.get::<Read<MaxBodyLength>>()
                .ok().cloned().unwrap_or(DEFAULT_BODY_LIMIT);
            let body = try!(read_body_as_utf8(req, max_length));
            Ok(Some(body))
        } else {
            Ok(None)
        }
    }
}