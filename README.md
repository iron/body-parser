body-parser [![Build Status](https://secure.travis-ci.org/iron/body-parser.png?branch=master)](https://travis-ci.org/iron/body-parser)
====

> Body parsing plugins for the [Iron](https://github.com/iron/iron) web framework.

## Example

```rust
extern crate iron;
extern crate bodyparser;
extern crate persistent;

use persistent::Read;
use iron::status;
use iron::prelude::*;

#[derive(Debug, Clone, RustcDecodable)]
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
        Ok(Some(json_body)) => println!("Parsed body:\n{}", json_body),
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

// `curl -i "localhost:3000/" -H "application/json" -d '{"name":"jason","age":"2"}'`
// and check out the printed json in your terminal.
fn main() {
    let mut chain = Chain::new(log_body);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    Iron::new(chain).http("localhost:3000").unwrap();
}
```

## Overview

body-parser is a part of Iron's [core bundle](https://github.com/iron/core). It contains:

* **Raw** - performs body parsing to string with limiting.
* **Json** - parses body into Json.
* **Struct** - parses body into a struct using Decode.

## Installation

If you're using a `Cargo.toml` to manage dependencies, just add body-parser to the toml:

```toml
[dependencies.bodyparser]

git = "https://github.com/iron/body-parser.git"
```

Otherwise, `cargo build`, and the rlib will be in your `target` directory.

## [Documentation](http://ironframework.io/doc/bodyparser/index.html)

Along with the [online documentation](http://ironframework.io/doc/bodyparser/index.html),
you can build a local copy with `make doc`.

## [Examples](/examples)

## Get Help

One of us ([@reem](https://github.com/reem/), [@zzmp](https://github.com/zzmp/),
[@theptrk](https://github.com/theptrk/), [@mcreinhard](https://github.com/mcreinhard))
is usually on `#iron` on the mozilla irc. Come say hi and ask any questions you might have.
We are also usually on `#rust` and `#rust-webdev`.
