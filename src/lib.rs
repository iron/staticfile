#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/7853871?s=128", html_favicon_url = "https://avatars0.githubusercontent.com/u/7853871?s=256", html_root_url = "http://ironframework.io/core/staticfile")]
#![crate_name = "staticfile"]
#![deny(missing_doc)]
#![feature(phase)]

//! Static file-serving middleware.

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

extern crate http;
extern crate iron;
#[phase(plugin, link)]
extern crate log;
extern crate mount;

use http::headers::content_type::MediaType;
use iron::{Request, Response, Middleware, Status, Continue, Unwind, Url};
use mount::OriginalUrl;

/// The static file-serving `Middleware`.
#[deriving(Clone)]
pub struct Static {
    root_path: Path
}

#[deriving(Clone)]
#[doc(hidden)]
struct Favicon {
    max_age: u8,
    favicon_path: Path
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

    /// Create a favicon server from the given filepath.
    ///
    /// This will serve your favicon, as specified by `favicon_path`,
    /// to every request ending in "/favicon.ico" that it sees,
    /// and then unwind the middleware stack for those requests.
    ///
    /// It should be linked first in order to avoid additional processing
    /// for simple favicon requests.
    ///
    /// Unlike normally served static files, favicons are given a max-age,
    /// specified in seconds.
    #[allow(visible_private_types)]
    pub fn favicon(favicon_path: Path, max_age: u8) -> Favicon {
        Favicon {
            max_age: max_age,
            favicon_path: favicon_path
        }
    }
}

impl Middleware for Static {
    fn enter(&mut self, req: &mut Request, res: &mut Response) -> Status {
        // Get the URL path as a slice of Strings.
        let url_path: &[String] = req.url.path.as_slice();

        // Create a file path by combining the Middleware's root path and the URL path.
        let requested_path = self.root_path.join(Path::new("").join_many(url_path));

        // If the requested path matches a real file, serve it.
        if requested_path.is_file() {
            match res.serve_file(&requested_path) {
                Ok(()) => {
                    debug!("Serving static file at {}", requested_path.display());
                    return Unwind;
                },
                Err(e) => {
                    error!("Errored trying to send file at {} with {}",
                          requested_path.display(), e);
                    return Continue;
                }
            }
        }

        // If the requested path is a directory containing an index.html, serve it.
        let index_path = requested_path.join("index.html");
        if index_path.is_file() {
            // If the URL ends in a slash, serve the file directly.
            // As per servo/rust-url/serialize_path, URLs ending in a slash have
            // an empty string stored as the last component of their path.
            // Rust-url even ensures that url.path() is non-empty by
            // appending a forward slash to URLs like http://example.com
            // Just in case a Middleware has mutated the URL's path to violate this property,
            // the empty list case is handled as a redirect.
            match url_path.last().as_ref().map(|s| s.as_slice()) {
                Some("") => {
                    match res.serve_file(&index_path) {
                        Ok(()) => {
                            debug!("Serving static file at {}.", index_path.display());
                            return Unwind;
                        },
                        Err(err) => {
                            debug!("Failed while trying to serve index.html: {}", err);
                            return Continue;
                        }
                    }
                },
                // Otherwise, redirect to the directory equivalent of the URL, ala Apache.
                // Some(_) corresponds to a path without a trailing slash, whilst None
                // corresponds to a path that has been mutated by Middleware into the empty list.
                Some(_) | None => {}
            }

            // Perform an HTTP 301 Redirect.
            let redirect_path = match req.extensions.find::<OriginalUrl>() {
                Some(&OriginalUrl(ref original_url)) => format!("{}/", original_url),
                None => format!("{}/", req.url)
            };
            res.headers.extensions.insert("Location".to_string(), redirect_path.clone());
            let _ = res.serve(::http::status::MovedPermanently,
                                format!("Redirecting to {}/", redirect_path));
            return Unwind;
        }

        // If no file is found, continue to other middleware.
        Continue
    }
}

impl Middleware for Favicon {
    fn enter(&mut self, req: &mut Request, res: &mut Response) -> Status {
        if req.url.path.as_slice() == ["favicon.ico".to_string()] {
            res.headers.content_type = Some(MediaType {
                type_: "image".to_string(),
                subtype: "x-icon".to_string(),
                parameters: vec![]
            });
            res.headers.cache_control = Some(format!("public, max-age={}", self.max_age));
            match res.serve_file(&self.favicon_path) {
                Ok(()) => (),
                Err(_) => {
                    let _ = res.serve(::http::status::InternalServerError,
                                      "Failed to serve favicon.ico.");
                }
            }
            Unwind
        } else {
            Continue
        }
    }
}

