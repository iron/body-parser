#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/7853871?s=128", html_favicon_url = "https://avatars0.githubusercontent.com/u/7853871?s=256", html_root_url = "http://ironframework.io/core/bodyparser")]
//! Body Parser middleware for Iron
//!
//! This middleware parses incoming JSON data from client requests. On an empy
//! request, or on malformed data, the chain is not unwound, but rather
//! nothing is inserted into the `Alloy`. Middleware further down the chain
//! must take care to handle this robustly.
#![crate_id = "bodyparser"]
#![license = "MIT"]

extern crate iron;
extern crate serialize;

use iron::{Request, Response, Middleware, Alloy};
use iron::middleware::{Status, Continue};

use serialize::json;
use serialize::json::Json;

/// The Parsed type holds the Json object that was parsed from incoming data.
#[deriving(Clone)]
pub struct Parsed(pub Json);

#[deriving(Clone)]
pub struct BodyParser;

impl BodyParser {
    /// Create a new `BodyParse`, for linking into a `Chain`
    pub fn new() -> BodyParser {
        BodyParser
    }
}

impl Middleware for BodyParser {
    fn enter(&mut self, req: &mut Request, _ : &mut Response, alloy: &mut Alloy) -> Status {
        if !req.body.is_empty() {
            match json::from_str(req.body.as_slice()) {
                Ok(parsed) => {
                    alloy.insert::<Parsed>(Parsed(parsed));
                },
                Err(_) => {
                    return Continue;
                }
            }
        }
        Continue
    }
}
