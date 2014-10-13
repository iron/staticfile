#![crate_name = "static_file"]
#![deny(missing_doc)]
#![feature(phase)]

//! Static file-serving handler.

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

extern crate http;
extern crate iron;
#[phase(plugin, link)]
extern crate log;
extern crate mount;

use iron::{Request, Response, Url, Handler, Error, IronResult};
use iron::status;
use mount::OriginalUrl;
use std::io::IoError;
use std::io::fs::PathExtensions;

/// The static file-serving `Handler`.
///
/// This handler serves files from a single filesystem path, which may be absolute or relative.
/// Incoming requests are mapped onto the filesystem by appending their URL path to the handler's
/// root path. If the filesystem path corresponds to a regular file, the handler will attempt to
/// serve it. Otherwise, if the path corresponds to a directory containing an `index.html`,
/// the handler will attempt to serve that instead.
///
/// ## Errors
///
/// If the path doesn't match any real object in the filesystem, the handler will return
/// a Response with `status::NotFound`. If an IO error occurs whilst attempting to serve
/// a file, `FileError(IoError)` will be returned.
#[deriving(Clone)]
pub struct Static {
    root_path: Path
}

/// The error returned when an IoError occurs during file serving.
#[deriving(Show)]
pub struct FileError(IoError);

impl Error for FileError {
    fn name(&self) -> &'static str {
        let &FileError(ref error) = self;
        error.desc
    }
}

impl Static {
    /// Create a new instance of `Static` with a given root path.
    ///
    /// If `Path::new("")` is given, files will be served from the current directory.
    pub fn new(root_path: Path) -> Static {
        Static {
            root_path: root_path
        }
    }
}

impl Handler for Static {
    fn call(&self, req: &mut Request) -> IronResult<Response> {
        // Get the URL path as a slice of Strings.
        let url_path: &[String] = req.url.path.as_slice();

        // Create a file path by combining the handler's root path and the URL path.
        let requested_path = self.root_path.join(Path::new("").join_many(url_path));

        // If the requested path matches a real file, serve it.
        if requested_path.is_file() {
            match Response::from_file(&requested_path) {
                Ok(response) => {
                    debug!("Serving static file at {}", requested_path.display());
                    return Ok(response);
                },
                Err(err) => {
                    return Err(FileError(err).erase());
                }
            }
        }

        // If the requested path is a directory containing an index.html, serve it.
        let index_path = requested_path.join("index.html");
        if index_path.is_file() {
            // If the URL ends in a slash, serve the file directly.
            // As per servo/rust-url/serialize_path, URLs ending in a slash have
            // an empty string stored as the last component of their path.
            // Rust-url even ensures that url.path is non-empty by
            // appending a forward slash to URLs like http://example.com
            // Some middleware may mutate the URL's path to violate this property,
            // so the empty list case is handled as a redirect.
            match url_path.last().as_ref().map(|s| s.as_slice()) {
                Some("") => {
                    match Response::from_file(&index_path) {
                        Ok(response) => {
                            debug!("Serving static file at {}.", index_path.display());
                            return Ok(response);
                        },
                        Err(err) => {
                            return Err(FileError(err).erase());
                        }
                    }
                },
                // Otherwise, redirect to the directory equivalent of the URL, ala Apache.
                // Some(_) corresponds to a path without a trailing slash, whilst None
                // corresponds to a path that has been mutated by Middleware into the empty list.
                Some(_) | None => {}
            }

            // Perform an HTTP 301 Redirect.
            let redirect_path = match req.extensions.find::<OriginalUrl, Url>() {
                Some(original_url) => format!("{}/", original_url),
                None => format!("{}/", req.url)
            };
            let mut res = Response::with(status::MovedPermanently,
                            format!("Redirecting to {}", redirect_path));
            res.headers.extensions.insert("Location".to_string(), redirect_path);
            return Ok(res);
        }

        // If no file is found, return a 404 response.
        Ok(Response::status(status::NotFound))
    }
}
