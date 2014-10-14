body-parser [![Build Status](https://secure.travis-ci.org/iron/body-parser.png?branch=master)](https://travis-ci.org/iron/body-parser)
====

> JSON body parsing middleware for the [Iron](https://github.com/iron/iron) web framework.

## Example

```rust
extern crate iron;
extern crate bodyparser;

use std::io::net::ip::Ipv4Addr;

use iron::{Iron, Request, Response, IronResult, Plugin, status};
use bodyparser::BodyParser;

fn log_json(req: &mut Request) -> IronResult<Response> {
    req.get::<BodyParser>().map(|parsed| println!("Parsed Json:\n{}", parsed));
    Ok(Response::with(status::Ok, ""))
}

// With fn main, you now have a running server at port 3000!
// `curl -i "127.0.0.1:3000/" -H "application/json" -d '{"A":"1","B":"2"}'`
// and check out the printed json in your terminal.
fn main() {
    Iron::new(log_json).listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
```

## Overview

body-parser is a part of Iron's [core bundle](https://github.com/iron/core).

- Perform JSON parsing using native functionality bundled in the standard
  library. 

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
