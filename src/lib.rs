#![crate_name = "bodyparser"]
#![license = "MIT"]
#![feature(default_type_params)]

//! Body Parser Plugin for Iron
//!
//! This plugin parses JSON out of an incoming Request.

extern crate iron;
extern crate serialize;
extern crate plugin;

use iron::Request;
use iron::typemap::Assoc;

use plugin::{PluginFor, Phantom};

use serialize::json;
use serialize::json::Json;

use std::str;

#[deriving(Clone)]
pub struct BodyParser;

impl Assoc<Json> for BodyParser {}

impl PluginFor<Request, Json> for BodyParser {
    fn eval(req: &mut Request, _: Phantom<BodyParser>) -> Option<Json> {
        if !req.body.is_empty() {
            str::from_utf8(req.body.as_slice())
                .and_then(|body| json::from_str(body).ok())
        } else {
            None
        }
    }
}

