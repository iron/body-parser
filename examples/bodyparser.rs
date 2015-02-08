extern crate iron;
extern crate bodyparser;
extern crate plugin;

use plugin::Pluggable;

use iron::status;

fn log_body(req: &mut iron::Request) -> iron::IronResult<iron::Response> {
    let parsed = req.get::<bodyparser::BodyReader>();
    match parsed {
        Ok(Some(body)) => println!("Parsed body:\n{}", body),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }
    Ok(iron::Response::with(status::Ok))
}

// `curl -i "localhost:3000/" -H "application/json" -d '{"name":"jason","age":"2"}'`
// and check out the printed json in your terminal.
fn main() {
    let chain = iron::Chain::new(log_body);
    iron::Iron::new(chain).listen("localhost:3000").unwrap();
}

