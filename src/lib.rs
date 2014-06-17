#![crate_id = "bodyparser"]
#![license = "MIT"]

extern crate iron;
extern crate serialize;

//use iron::{Iron, Ingot, Furnace, Alloy, Request, Response, ServerT};
use iron::{Ingot, Alloy, Request, Response};
use iron::ingot::{Status, Continue, Unwind};

use serialize::json;
//use serialize::json::{List, Object};

#[deriving(Clone)]
struct Parsed(json::Json);

#[deriving(Clone)]
pub struct BodyParser;

impl BodyParser {
    fn new() -> BodyParser {
        BodyParser
    }
}

impl<Rq: Request, Rs: Response> Ingot<Rq, Rs> for BodyParser {
    fn enter(&mut self, _rq: &mut Rq, _rs: &mut Rs, alloy: &mut Alloy) -> Status {
        alloy.insert::<Parsed>(Parsed(parse_body(_rq.body())));
        Continue
    }
}

fn parse_body(x:&str) -> json::Json {
    let json_object = json::from_str(x.as_slice());
    json_object.clone().unwrap()
}