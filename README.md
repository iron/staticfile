static-file [![Build Status](https://secure.travis-ci.org/iron/static-file.png?branch=master)](https://travis-ci.org/iron/static-file)
====

> Static file-serving middleware for the [Iron](https://github.com/iron/iron) web framework.

## Example

```rust
fn main() {
    let mut server: Server = Iron::new();
    // Serve a favicon
    server.chain.link(Static::favicon(Path::new("path/to/favicon")));
    // Serve the docs
    server.chain.link(Static::new(Path::new("doc/")));
    // Serve the index.html
    server.chain.link(Static::new(Path::new("doc/staticfile/")));
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
```

## Overview

static-file is a part of Iron's [core bundle](https://github.com/iron/core).

- Serve static files from a given path, if available.
- Serve all requests to favicon.ico with a given file.

## Installation

If you're using a `Cargo.toml` to manage dependencies, just add static-file to the toml:

```toml
[dependencies.staticfile]

git = "https://github.com/iron/static-file.git"
```

Otherwise, `cargo build`, and the rlib will be in your `target` directory.

## [Documentation](http://docs.ironframework.io/staticfile)

Along with the [online documentation](http://docs.ironframework.io/staticfile),
you can build a local copy with `make doc`.

## [Examples](/examples)

## Get Help

One of us ([@reem](https://github.com/reem/), [@zzmp](https://github.com/zzmp/),
[@theptrk](https://github.com/theptrk/), [@mcreinhard](https://github.com/mcreinhard))
is usually on `#iron` on the mozilla irc. Come say hi and ask any questions you might have.
We are also usually on `#rust` and `#rust-webdev`.
