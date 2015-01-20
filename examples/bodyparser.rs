extern crate iron;
extern crate bodyparser;
extern crate serialize;

use iron::prelude::*;
use iron::status;
use bodyparser::BodyParser;

#[derive(Decodable, Clone, Show)]
struct JsonParams {
    name: String,
    age: Option<i8>,
}

fn log_json(req: &mut Request) -> IronResult<Response> {
    let parsed = req.get::<BodyParser<JsonParams>>();
    match parsed {
        Some(params) => println!("Parsed json:\n{:?}", params),
        None => println!("Invalid or no json!"),
    }
    Ok(Response::with(status::Ok))
}

// `curl -i "localhost:3000/" -H "application/json" -d '{"name":"jason","age":"2"}'`
// and check out the printed json in your terminal.
fn main() {
    Iron::new(log_json).listen("localhost:3000").unwrap();
}

