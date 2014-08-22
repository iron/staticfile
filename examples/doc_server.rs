extern crate iron;
extern crate staticfile;

// This example will serve the docs (from target/doc).
// To use, `cargo doc && cargo test && target/doc_server`.

use std::io::net::ip::Ipv4Addr;

use iron::{Iron, Server, Chain};
use staticfile::Static;

fn main() {
    let mut server: Server = Iron::new();
    // Serve the docs
    server.chain.link(Static::new(Path::new("target/doc/")));
    // Serve the index.html
    server.chain.link(Static::new(Path::new("target/doc/staticfile/")));
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
