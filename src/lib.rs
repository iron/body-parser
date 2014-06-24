//! Body Parser middleware for Iron
//! 
//! This middleware focuses on parsing incoming data from client requests. Body Parser 
//! has a function `new`, that creates an instance of the middleware and if both invoked 
//! and linked to `server.chain`, it will attach a Parsed<Json> struct to the `Alloy`.
#![crate_id = "bodyparser"]
#![license = "MIT"]

extern crate iron;
extern crate serialize;

use iron::{Request, Response, Middleware, Alloy};
use iron::middleware::{Status, Continue, Unwind};

use serialize::json;
use serialize::json::Json;

/// The Parsed type holds the Json object that is parsed from incoming data.
/// This struct is created with a call to `BodyParser.new` to store on an alloy. 
#[deriving(Clone)]
pub struct Parsed(pub Json);

/// `Middleware` for parsing passed in data and storing the data as JSON.
/// Using `new` and adding this link to the chain is the current implementation.
#[deriving(Clone)]
pub struct BodyParser;

/// Using the `new` function will activate this middleware and parse incoming data
/// and insert it only an `Alloy`. Subsequent middlware will now have access by using
/// the `find` function from `Alloy`.
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
