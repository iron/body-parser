extern crate iron;
extern crate bodyparser;
extern crate serialize;

use std::io::net::ip::Ipv4Addr;

use iron::{Iron, Request, Response, IronResult, Plugin, status};
use bodyparser::BodyParser;

#[deriving(Decodable)]
#[deriving(Clone)]
#[deriving(Show)]
struct JsonParams {
    name: String,
    age: Option<i8>,
}

fn log_json(req: &mut Request) -> IronResult<Response> {
    let parsed = req.get::<BodyParser<JsonParams>>();
    match parsed {
        Some(params) => println!("Parsed json:\n{}", params),
        None => println!("Invalid or no json!"),
    }
    Ok(Response::with(status::Ok, ""))
}

// With fn main, you now have a running server at port 3000!
// `curl -i "127.0.0.1:3000/" -H "application/json" -d '{"A":"1","B":"2"}'`
// and check out the printed json in your terminal.
fn main() {
    Iron::new(log_json).listen(Ipv4Addr(127, 0, 0, 1), 3000);
}

