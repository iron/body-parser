body-parser [![Build Status](https://secure.travis-ci.org/iron/body-parser.png?branch=master)](https://travis-ci.org/iron/body-parser)
====

> JSON body parsing middleware for the [Iron](https://github.com/iron/iron) web framework.

## Example

```rust
extern crate iron;
extern crate bodyparser;
use iron::{Iron, ServerT, Chain, Request, Response, Alloy};
use bodyparser::{BodyParser, Parsed};

fn main() {
    let mut server: ServerT = Iron::new();
    server.chain.link(Bodyparser::new()); // Add middleware to the server's stack
    server.listen(::std::io::net::ip::Ipv4Addr(127, 0, 0, 1), 3000);
}
```

## Overview

body-parser is a part of Iron's [core bundle](https://github.com/iron/core).

- The middleware performs JSON parsing using native functionality bundled in the standard
  library. 
- Iron functionality is packed in middleware, so link BodyParser to the chain
  just like all other middleware.

## Installation

If you're using a `Cargo` to manage dependencies, just add body-parser to the toml:

```toml
[dependencies.bodyparser]

git = "https://github.com/iron/body-parser.git"
```

Otherwise, `cargo build`, and the rlib will be in your `target` directory.

## [Documentation](http://docs.ironframework.io/core/bodyparser)

Along with the [online documentation](http://docs.ironframework.io/core/bodyparser),
you can build a local copy with `make doc`.

## [Examples](/examples)

## Get Help

One of us ([@reem](https://github.com/reem/), [@zzmp](https://github.com/zzmp/),
[@theptrk](https://github.com/theptrk/), [@mcreinhard](https://github.com/mcreinhard))
is usually on `#iron` on the mozilla irc. Come say hi and ask any questions you might have.
We are also usually on `#rust` and `#rust-webdev`.
