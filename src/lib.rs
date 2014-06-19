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
    fn enter(&mut self, _req: &mut Request, res: &mut Response, alloy: &mut Alloy) -> Status {
        if _req.body.len() != 0 {
            let parse = json::from_str(_req.body.as_slice());
            println!("{}", parse)
            match parse {
                Ok(e) => {
                    println!("e: {}", e)
                    alloy.insert::<Parsed>(Parsed(e));
                },
                Err(e) => {
                    println!("{}\nPlease pass valid JSON", e);
                }
            }
        }
        Continue
    }
}