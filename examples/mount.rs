extern crate iron;
extern crate staticfile;
extern crate mount;

// This example will serve the docs (from doc), but
// mounting it from a different path.
//
// To use, `make examples doc && ./examples/mount`, then
// point your browser to 127.0.0.1:3000/doc/

use std::io::net::ip::Ipv4Addr;

use iron::{Iron, Server, Chain};
use staticfile::Static;
use mount::Mount;


fn main() {
    let mut server: Server = Iron::new();

    // Serve the stylesheet at /main.css
    server.chain.link(Mount::new("/main.css", Static::new(Path::new("target/doc/main.css"))));
    // Serve the docs at /doc/
    server.chain.link(Mount::new("/doc/", Static::new(Path::new("target/doc/staticfile/"))));
    // Serve the source code at /src/
    server.chain.link(Mount::new("/src/", Static::new(Path::new("target/doc/src/staticfile/src/lib.rs.html"))));

    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
