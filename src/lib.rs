#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/7853871?s=128", html_favicon_url = "https://avatars0.githubusercontent.com/u/7853871?s=256", html_root_url = "http://ironframework.io/core/bodyparser")]
//! Body Parser middleware for Iron
//! 
//! This middleware focuses on parsing incoming data from client requests.
#![crate_id = "bodyparser"]
#![license = "MIT"]

extern crate iron;
extern crate serialize;

use iron::{Request, Response, Middleware, Alloy};
use iron::middleware::{Status, Continue};

use serialize::json;
use serialize::json::Json;

/// The Parsed type holds the Json object that is parsed from incoming data.
#[deriving(Clone)]
pub struct Parsed(pub Json);

#[deriving(Clone)]
pub struct BodyParser;

/// Using `pub fn new() -> BodyParser` will create a new instance of BodyParser,
/// which can then be `link`ed to a `Chain`.
impl BodyParser {
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
