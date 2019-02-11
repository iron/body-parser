#![crate_name = "bodyparser"]

//! Body Parser Plugin for Iron
//!
//! This plugin parses JSON out of an incoming Request.

extern crate iron;
extern crate plugin;
extern crate persistent;
extern crate serde;
extern crate serde_json;

use serde::Deserialize;
use serde_json::{from_str, from_value};

use iron::mime;
use iron::prelude::*;
use iron::headers;
use iron::typemap::{Key};
use std::any::Any;
use std::marker;

pub use self::errors::{BodyError, BodyErrorCause};

mod errors;

/// This implementation currently ignores the limit parameter, since irons
/// request::get_body_contents() reads the data unlimited.
fn read_body_as_utf8(req: &mut Request, _limit: usize) -> Result<String, errors::BodyError> {
    match req.get_body_contents() {
        Ok(bytes) => {
             match String::from_utf8(bytes.to_vec()) {
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

/// Raw is a plugin to read a request body into UTF-8 String.
/// Doesn't read `multipart/form-data`.
pub struct Raw;

impl Key for Raw {
    type Value = Option<String>;
}

const DEFAULT_BODY_LIMIT: usize = 1024 * 1024 * 100;

impl plugin::Plugin<Request> for Raw {
    type Error = BodyError;

    fn eval(req: &mut Request) -> Result<Option<String>, BodyError> {
        let need_read = req.headers.get(headers::CONTENT_TYPE).map(|header| {
            header.to_str().unwrap().parse::<mime::Mime>().unwrap()
                != "multipart/form-data".parse::<mime::Mime>().unwrap()
        }).unwrap_or(false);

        if need_read {
            let max_length = req
                .get::<persistent::Read<MaxBodyLength>>()
                .ok()
                .map(|x| *x)
                .unwrap_or(DEFAULT_BODY_LIMIT);

            let body = try!(read_body_as_utf8(req, max_length));
            Ok(Some(body))
        } else {
            Ok(None)
        }
    }
}

/// Json is a plugin to parse a request body into JSON.
/// Uses Raw plugin to parse the body with limit.
#[derive(Clone)]
pub struct Json;
impl Key for Json {
    type Value = Option<serde_json::Value>;
}

impl plugin::Plugin<Request> for Json {
    type Error = BodyError;

    fn eval(req: &mut Request) -> Result<Option<serde_json::Value>, BodyError> {
        req.get::<Raw>()
            .and_then(|maybe_body| {
                reverse_option(maybe_body.map(|body| from_str(&body)))
                    .map_err(|err| {
                        BodyError {
                            detail: "Can't parse body to JSON".to_string(),
                            cause: BodyErrorCause::JsonError(err)
                        }
                    })
            })
    }
}

/// Struct is a plugin to parse a request body into a struct.
/// Uses Raw plugin to parse the body with limit.
pub struct Struct<T> where T: for<'a> Deserialize<'a> {
    marker: marker::PhantomData<T>
}
impl<T> Key for Struct<T> where T: for<'a> Deserialize<'a> + Any {
    type Value = Option<T>;
}

impl<T> plugin::Plugin<Request> for Struct<T>
where T: for<'c> Deserialize<'c> + Any {
    type Error = BodyError;

    fn eval(req: &mut Request) -> Result<Option<T>, BodyError> {
        req.get::<Json>()
            .and_then(|maybe_body| {
                reverse_option(maybe_body.map(|body| from_value(body)))
                    .map_err(|err| BodyError {
                        detail: "Can't parse body to the struct".to_string(),
                        cause: BodyErrorCause::JsonError(err)
                    })
            })
    }
}

fn reverse_option<T,E>(value: Option<Result<T, E>>) -> Result<Option<T>, E> {
    match value {
        Some(Ok(val)) => Ok(Some(val)),
        Some(Err(err)) => Err(err),
        None => Ok(None),
    }
}
