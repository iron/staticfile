extern crate iron;
extern crate hello;

use std::io::net::ip::Ipv4Addr;

use iron::{Iron, ServerT};

use hello::HelloWorld;

fn main() {
    let mut server: ServerT = Iron::new();
    server.smelt(HelloWorld::new());
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
