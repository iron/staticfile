#![crate_id = "hello"]
#![deny(missing_doc)]
#![feature(phase)]

//! A simple middleware to serve "Hello, world!" to all requests.

extern crate iron;
extern crate http;
#[phase(plugin, link)]
extern crate log;

use iron::{Request, Response, Middleware, Alloy};
use iron::middleware::{Status, Continue, Unwind};

use http::status;

/// The hello `Middleware`.
#[deriving(Clone)]
pub struct HelloWorld;

impl HelloWorld {
    /// Create a new instance of the hello `Middleware`.
    pub fn new() -> HelloWorld {
        HelloWorld
    }
}

impl Middleware for HelloWorld {
    /// Serve "Hello, world!"
    ///
    /// In the case of an error, return a status of 500: InternalServerError
    fn enter(&mut self, _req: &mut Request, res: &mut Response, _alloy: &mut Alloy) -> Status {
        match res.write(bytes!("Hello, world!")) {
            Ok(()) => (),
            Err(_) => {
                res.status = status::InternalServerError;
                return Unwind
            }
        }
        Continue
    }

    /// Debug what you did.
    ///
    /// Prints `Served "Hello, World".` if RUST_LOG is set to 4.
    /// This function does not need to be implemented (neither does `enter`).
    /// `Middleware` implements a default function which does nothing.
    /// It is implemented for the sake of example.
    fn exit(&mut self, _req: &mut Request, _res: &mut Response, _alloy: &mut Alloy) -> Status {
        debug!("Served \"Hello, World\".");
        Continue
    }
}
