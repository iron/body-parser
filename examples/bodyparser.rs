extern crate iron;
extern crate bodyparser;
extern crate persistent;
extern crate serde;

use persistent::Read;
use iron::status;
use iron::prelude::*;
use serde::{Deserialize, Deserializer};
use serde::de::{MapVisitor, Visitor};

#[derive(Debug, Clone)]
struct MyStructure {
    a: String,
    b: Option<String>,
}

// In your programs, you can automatically derive from `Deserialize`
// using serde_macros or serde_codegen + syntex. It's implemented manually here
// to avoid adding a dependency on any of those libraries to the bodyparser
// crate.
//
// See https://github.com/serde-rs/serde for details.
impl Deserialize for MyStructure {
    fn deserialize<D>(deserializer: &mut D) -> Result<MyStructure, D::Error>
        where D: Deserializer
    {
        static FIELDS: &'static [&'static str] = &["a", "b"];

        deserializer.visit_struct("MyStructure", FIELDS, MyStructureVisitor)
    }
}

// Same as above. In your own code you won't need to implement this manually.
struct MyStructureVisitor;

// Same as above. In your own code you won't need to implement this manually.
impl Visitor for MyStructureVisitor {
    type Value = MyStructure;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<MyStructure, V::Error>
        where V:  MapVisitor
    {
        let mut a = None;
        let mut b = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(MyStructureField::A) => { a = Some(try!(visitor.visit_value())); }
                Some(MyStructureField::B) => { b = Some(try!(visitor.visit_value())); }
                None => { break; }
            }
        }

        let a = match a {
            Some(a) => a,
            None => try!(visitor.missing_field("a")),
        };

        let b = match b {
            Some(b) => b,
            None => try!(visitor.missing_field("b")),
        };

        try!(visitor.end());

        Ok(MyStructure{
            a: a,
            b: b,
        })
    }
}

// Same as above. In your own code you won't need to implement this manually.
enum MyStructureField {
    A,
    B,
}

// Same as above. In your own code you won't need to implement this manually.
impl Deserialize for MyStructureField {
    fn deserialize<D>(deserializer: &mut D) -> Result<MyStructureField, D::Error>
        where D: Deserializer
    {
        struct MyStructureFieldVisitor;

        impl Visitor for MyStructureFieldVisitor {
            type Value = MyStructureField;

            fn visit_str<E>(&mut self, value: &str) -> Result<MyStructureField, E>
                where E: serde::de::Error
            {
                match value {
                    "a" => Ok(MyStructureField::A),
                    "b" => Ok(MyStructureField::B),
                    _ => Err(serde::de::Error::syntax("expected a or b")),
                }
            }
        }

        deserializer.visit(MyStructureFieldVisitor)
    }
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

    Ok(Response::with(status::Ok))
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
    Iron::new(chain).http("localhost:3000").unwrap();
}
