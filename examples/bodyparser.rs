extern crate iron;
extern crate bodyparser;
extern crate persistent;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use persistent::Read;
use iron::StatusCode;
use iron::prelude::*;

#[derive(Deserialize, Debug, Clone)]
struct MyStructure {
    a: String,
    b: Option<String>,
}

fn log_body(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<bodyparser::Raw>();
    match body {
        Ok(Some(body)) => println!("Read body:\n{}", body),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }

    let json_body = req.get::<bodyparser::Json>();
    match json_body {
        Ok(Some(json_body)) => println!("Parsed body:\n{:?}", json_body),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }

    let struct_body = req.get::<bodyparser::Struct<MyStructure>>();
    match struct_body {
        Ok(Some(struct_body)) => println!("Parsed body:\n{:?}", struct_body),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }

    Ok(Response::with(StatusCode::OK))
}

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

// While the example is running, try the following curl commands and see how they are
// logged by the Rust server process:
//
// `curl -i "localhost:3000/" -H "application/json" -d '{"name":"jason","age":"2"}'`
// `curl -i "localhost:3000/" -H "application/json" -d '{"a":"jason","b":"2"}'`
// `curl -i "localhost:3000/" -H "application/json" -d '{"a":"jason"}'`
fn main() {
    let mut chain = Chain::new(log_body);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    Iron::new(chain).http("localhost:3000");
}
