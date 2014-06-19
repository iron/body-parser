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
        println!("Welcome to Body Parser")
        if _req.body.len() != 0 {
            println!("its not 0")
            alloy.insert::<Parsed>(Parsed(parse_body(_req.body.clone())));
        }
        Continue
    }
}

fn parse_body(x:String) -> Json {
    let json_object = json::from_str(x.clone().as_slice());
    let obj = json_object.clone().unwrap();
    obj
}