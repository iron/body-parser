body-parser [![Build Status](https://secure.travis-ci.org/iron/body-parser.png?branch=master)](https://travis-ci.org/iron/body-parser)
====

> Body parsing middleware for the [Iron](https://github.com/iron/iron) web framework.

## Example

```rust
extern crate iron::prelude::*;
extern crate bodyparser;

use iron::status;

fn log_body(req: &mut Request) -> IronResult<Response> {
    let parsed = req.extensions.get::<bodyparser::BodyKey>();
    match parsed {
        Some(body) => println!("Parsed body:\n{}", body),
        None => println!("Invalid or no body!"),
    }
    Ok(Response::with(status::Ok))
}

// `curl -i "localhost:3000/" -H "application/json" -d '{"name":"jason","age":"2"}'`
// and check out the printed json in your terminal.
fn main() {
    let mut chain = Chain::new(log_body);
    chain.link_before(bodyparser::BodyReader::new(1024 * 1024 * 10));
    iron::Iron::new(chain).listen("localhost:3000").unwrap();
}
```

## Overview

body-parser is a part of Iron's [core bundle](https://github.com/iron/core).

- Perform body parsing to string with limiting.

## Installation

If you're using a `Cargo.toml` to manage dependencies, just add body-parser to the toml:

```toml
[dependencies.bodyparser]

git = "https://github.com/iron/body-parser.git"
```

Otherwise, `cargo build`, and the rlib will be in your `target` directory.

## [Documentation](http://docs.ironframework.io/bodyparser)

Along with the [online documentation](http://docs.ironframework.io/bodyparser),
you can build a local copy with `make doc`.

## [Examples](/examples)

## Get Help

One of us ([@reem](https://github.com/reem/), [@zzmp](https://github.com/zzmp/),
[@theptrk](https://github.com/theptrk/), [@mcreinhard](https://github.com/mcreinhard))
is usually on `#iron` on the mozilla irc. Come say hi and ask any questions you might have.
We are also usually on `#rust` and `#rust-webdev`.
