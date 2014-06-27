#![crate_id = "staticfile"]
#![deny(missing_doc)]
#![feature(phase)]

//! Static file-serving middleware.

extern crate iron;
#[phase(plugin, link)]
extern crate log;

use iron::{Request, Response, Middleware, Alloy};
use iron::mixin::{GetUrl, Serve};
use iron::middleware::{Status, Continue, Unwind};

/// The static file-serving `Middleware`.
#[deriving(Clone)]
pub struct Static {
    root_path: Path
}

impl Static {
    /// Create a new instance of `Static` with a given root path.
    ///
    /// This will attempt to serve static files from the given root path.
    /// The path may be relative or absolute. If `Path::new("")` is given,
    /// files will be served from the current directory.
    ///
    /// If a static file exists and can be read from, `enter` will serve it to
    /// the `Response` and `Unwind` the middleware stack with a status of `200`.
    ///
    /// In the case of any error, it will `Continue` through the stack.
    /// If a file should have been read but cannot, due to permissions or
    /// read errors, a different `Middleware` should handle it.
    ///
    /// If the path is '/', it will attempt to serve `index.html`.
    pub fn new(root_path: Path) -> Static {
        Static {
            root_path: root_path
        }
    }
}

impl Middleware for Static {
    fn enter(&mut self, req: &mut Request, res: &mut Response, _alloy: &mut Alloy) -> Status {
        match req.url() {
            Some(path) => {
                // Check for requested file
                match res.serve_file(&self.root_path.join(
                    Path::new(
                        // Coerce to relative path, see http://is.gd/yz9p0B
                        "./".to_string().append(path.as_slice())))) {
                    Ok(()) => {
                        debug!("Serving static file at {}.",
                            &self.root_path.join("./".to_string().append(path.as_slice())).display());
                        return Unwind
                    },
                    Err(_) => ()
                }
                // Check for index.html
                // Do not serve index.html as directory
            },
            None => ()
        }
        Continue
    }
}
