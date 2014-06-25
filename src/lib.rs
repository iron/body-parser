//! Body Parser middleware for Iron
//! 
//! This middleware focuses on parsing incoming data from client requests.
#![crate_id = "bodyparser"]
#![license = "MIT"]

extern crate iron;
extern crate serialize;

use iron::{Request, Response, Middleware, Alloy};
use iron::middleware::{Status, Continue, Unwind};

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
    fn enter(&mut self, req: &mut Request, res: &mut Response, alloy: &mut Alloy) -> Status {
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
