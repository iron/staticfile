#![crate_id = "staticfile"]
#![deny(missing_doc)]
#![feature(phase)]

//! Static file-serving middleware.

extern crate iron;
extern crate http;
#[phase(plugin, link)]
extern crate log;

use std::path::BytesContainer;
use std::str::from_utf8;

use iron::{Request, Response, ServeFile, Middleware, Alloy};
use iron::middleware::{Status, Continue, Unwind};

use http::server::request::AbsolutePath;

/// The static file-serving `Middleware`.
#[deriving(Clone)]
pub struct Static {
    root_path: Path
}

impl Static {
    /// Create a new instance of `Static`.
    pub fn new(root_path: Path) -> Static {
        Static {
            root_path: root_path
        }
    }
}

impl Middleware for Static {
    /// Serve static files.
    ///
    /// If a static file exists and can be read from, `enter` will serve it
    /// to the `Response` and `Unwind` the middleware stack.
    ///
    /// In the case of any error, it will `Continue` through the stack.
    ///
    /// If the path is '/', it will attempt to serve `index.html`.
    fn enter(&mut self, req: &mut Request, res: &mut Response, _alloy: &mut Alloy) -> Status {
        match req.request_uri {
            AbsolutePath(ref path) => {
                debug!("Serving static file at {}{}.", from_utf8(self.root_path.container_as_bytes()).unwrap(), path);
                let mut relative_path = path.clone();
                if relative_path.eq(&"/".to_string()) {
                    relative_path = "index.html".to_string();
                } else {
                    let _ = relative_path.as_slice().slice_from(1u);
                }
                match res.serve_file(&self.root_path.join(Path::new(relative_path.to_string()))) {
                    Ok(()) => { Unwind },
                    Err(_) => { Continue }
                }
            },
            _ => {
                Continue
            }
        }
    }
}
