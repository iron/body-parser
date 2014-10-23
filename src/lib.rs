#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/7853871?s=128", html_favicon_url = "https://avatars0.githubusercontent.com/u/7853871?s=256", html_root_url = "http://ironframework.io/core/bodyparser")]
#![feature(default_type_params)]
#![license = "MIT"]

//! Body Parser middleware for Iron
//!
//! This middleware parses incoming JSON data from client requests. On an empy
//! request, or on malformed data, the chain is not unwound, but rather
//! nothing is inserted into the `Alloy`. Middleware further down the chain
//! must take care to handle this robustly.

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

