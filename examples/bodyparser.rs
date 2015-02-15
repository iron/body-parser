extern crate iron;
extern crate bodyparser;
extern crate persistent;

use persistent::Read;
use iron::status;
use iron::prelude::*;

fn log_body(req: &mut Request) -> IronResult<Response> {
    let parsed = req.get::<bodyparser::BodyReader>();
    match parsed {
        Ok(Some(body)) => println!("Readed body:\n{}", body),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }
    Ok(Response::with(status::Ok))
}

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

// `curl -i "localhost:3000/" -H "application/json" -d '{"name":"jason","age":"2"}'`
// and check out the printed json in your terminal.
fn main() {
    let mut chain = Chain::new(log_body);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    Iron::new(chain).listen("localhost:3000").unwrap();
}

