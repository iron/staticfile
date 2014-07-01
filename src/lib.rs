#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/7853871?s=128", html_favicon_url = "https://avatars0.githubusercontent.com/u/7853871?s=256", html_root_url = "http://ironframework.io/core/staticfile")]
#![crate_id = "staticfile"]
#![deny(missing_doc)]
#![feature(phase)]

//! Static file-serving middleware.

extern crate url;
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

extern crate http;
extern crate iron;
#[phase(plugin, link)]
extern crate log;
extern crate mount;

use http::headers::content_type::MediaType;

use iron::{Request, Response, Middleware, Alloy, Status, Continue, Unwind};
use iron::mixin::{GetUrl, Serve};
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
    fn enter(&mut self, req: &mut Request, res: &mut Response, alloy: &mut Alloy) -> Status {
        match req.url() {
            Some(path) => {
                // Check for requested file
                match res.serve_file(&self.root_path.join(
                    Path::new(
                        // Coerce to relative path.
                        // We include the slash to ensure that you never have a path like ".index.html"
                        // when you meant "./index.html", see http://is.gd/yz9p0B for an example.
                        "./".to_string().append(path.as_slice())))) {
                    Ok(()) => {
                        debug!("Serving static file at {}.",
                            &self.root_path.join("./".to_string().append(path.as_slice())).display());
                        return Unwind
                    },
                    Err(_) => ()
                }

                // Check for index.html
                let index_path = self.root_path.join(
                    Path::new("./".to_string().append(path.as_slice()))
                        .join("./index.html".to_string()));
                if index_path.is_file() {
                    // Avoid serving as a directory
                    if path.len() > 0 {
                        match path.as_slice().char_at_reverse(path.len()) {
                            '/' => {
                                match res.serve_file(&index_path) {
                                    Ok(()) => {
                                        debug!("Serving static file at {}.",
                                            &index_path.display());
                                        return Unwind
                                    },
                                    Err(err) => {
                                        debug!("Failed while trying to serve index.html: {}", err)
                                        return Continue
                                    }
                                }
                            },
                    // 303:
                            _ => ()
                        }
                    }
                    let redirect_path = match alloy.find::<OriginalUrl>() {
                        Some(&OriginalUrl(ref original_url)) => original_url.clone(),
                        None => path.clone()
                    }.append("/");
                    res.headers.extensions.insert("Location".to_string(), redirect_path.clone());
                    let _ = res.serve(::http::status::SeeOther,
                        format!("Redirecting to {}/", redirect_path).as_slice());
                    return Unwind
                }
            },
            None => ()
        }
        Continue
    }
}

impl Middleware for Favicon {
    fn enter(&mut self, req: &mut Request, res: &mut Response, _alloy: &mut Alloy) -> Status {
        match req.url() {
            Some(path) => {
                if regex!("/favicon.ico$").is_match(path.as_slice()) {
                    res.headers.content_type = Some(MediaType {
                        type_: "image".to_string(),
                        subtype: "x-icon".to_string(),
                        parameters: vec![]
                    });
                    res.headers.cache_control = Some(format!("public, max-age={}", self.max_age));
                    let _ = res.try_write_headers();
                    match res.serve_file(&self.favicon_path) {
                        Ok(()) => (),
                        Err(_) => {
                            let _ = res.serve(::http::status::InternalServerError,
                                "Failed to serve favicon.ico.");
                        }
                    }
                    return Unwind;
                }
            },
            None => ()
        }
        Continue
    }
}
