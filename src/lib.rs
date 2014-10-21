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

use serialize::{json, Decodable};
use serialize::json::{Decoder, DecoderError};
use std::str;

#[deriving(Clone)]
pub struct BodyParser<T: Decodable<Decoder, DecoderError>>;

impl<T: 'static + Decodable<Decoder, DecoderError>> Assoc<T> for BodyParser<T> {}

impl<T: Decodable<Decoder, DecoderError>> PluginFor<Request, T> for BodyParser<T> {
    fn eval(req: &mut Request, _: Phantom<BodyParser<T>>) -> Option<T> {
        if !req.body.is_empty() {
            str::from_utf8(req.body.as_slice())
                .and_then(|body| json::from_str(body).ok())
                .and_then(|json| Decodable::decode(&mut json::Decoder::new(json)).ok())
        } else {
            None
        }
    }
}

