#![crate_name = "bodyparser"]

//! Body Parser Plugin for Iron
//!
//! This plugin parses JSON out of an incoming Request.

extern crate iron;
extern crate plugin;
extern crate serialize;
extern crate typemap;

use iron::Request;
use typemap::Key;

use plugin::{Plugin, Phantom};

use serialize::{json, Decodable};
use std::str;

#[deriving(Clone)]
pub struct BodyParser<T: Decodable>;

impl<T: 'static + Decodable> Key for BodyParser<T> {
    type Value = T;
}

impl<T: 'static + Decodable> Plugin<Request> for BodyParser<T> {
    fn eval(req: &mut Request, _: Phantom<BodyParser<T>>) -> Option<T> {
        if !req.body.is_empty() {
            str::from_utf8(req.body.as_slice()).ok()
                .and_then(|body| json::from_str(body).ok())
                .and_then(|json| Decodable::decode(&mut json::Decoder::new(json)).ok())
        } else {
            None
        }
    }
}

