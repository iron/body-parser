extern crate iron;
extern crate bodyparser;

use std::io::net::ip::Ipv4Addr;

use iron::{Iron, ServerT, Request, Response, Alloy, Chain};

use bodyparser::{BodyParser, Parsed};

// Here we create a function to log the json we are storing in Alloy.
// Alloy is where your middleware can store data and we access it through
// the `find` API exposed by Alloy.
fn log_json(_: &mut Request, _: &mut Response, alloy: &mut Alloy) {
    let json = alloy.find::<Parsed>();
    match allow.find::<Parsed>() {
        Some(&Parsed(ref parsed)) => println!("Parsed Json:\n{}", parsed),
        None => ()
    }
}

// With fn main, you now have a running server at port 3000!
// `curl -i "127.0.0.1:3000/" -H "application/json" -d '{"A":"1","B":"2"}'`
// and check out the printed json in your terminal.
fn main() {
    let mut server: ServerT = Iron::new();
    server.chain.link(BodyParser::new());
    server.chain.link(log_json);
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
