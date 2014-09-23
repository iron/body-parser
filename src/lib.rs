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

use serialize::json;
use serialize::json::Json;

#[deriving(Clone)]
pub struct BodyParser;

impl Assoc<Json> for BodyParser {}

impl PluginFor<Request, Json> for BodyParser {
    fn eval(req: &Request, _: Phantom<BodyParser>) -> Option<Json> {
        if !req.body.is_empty() {
            json::from_str(req.body.as_slice()).ok()
        } else {
            None
        }
    }
}

