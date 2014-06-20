//! Body Parser middleware for Iron
#![crate_id = "bodyparser"]
#![license = "MIT"]

extern crate iron;
extern crate serialize;

use iron::{Request, Response, Middleware, Alloy};
use iron::middleware::{Status, Continue, Unwind};

use serialize::json;
use serialize::json::Json;

#[deriving(Clone)]
struct Parsed(Json);

#[deriving(Clone)]
pub struct BodyParser;

impl BodyParser {
    pub fn new() -> BodyParser {
        BodyParser
    }
}

impl Middleware for BodyParser {
    fn enter(&mut self, req: &mut Request, res: &mut Response, alloy: &mut Alloy) -> Status {
        if !req.body.is_empty() {
            let parse = json::from_str(req.body.as_slice());
            match parse {
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
