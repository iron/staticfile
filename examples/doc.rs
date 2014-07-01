extern crate iron;
extern crate staticfile;

// This example will serve the docs (from doc).
// To use, `make examples doc && ./examples/doc`.

use std::io::net::ip::Ipv4Addr;

use iron::{Iron, Server, Chain};
use staticfile::Static;

fn main() {
    let mut server: Server = Iron::new();
    // Serve the docs
    server.chain.link(Static::new(Path::new("doc/")));
    // Serve the index.html
    server.chain.link(Static::new(Path::new("doc/staticfile/")));
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
