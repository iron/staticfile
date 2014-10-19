use iron::{Request, Response, Url, Handler, Error, IronResult};
use iron::status;
use mount::OriginalUrl;
use super::errors::FileError;
use super::requested_path::RequestedPath;

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
        let requested_path = RequestedPath::new(&self.root_path, req);

        // If the URL ends in a slash, serve the file directly.
        // Otherwise, redirect to the directory equivalent of the URL, ala Apache.
        if requested_path.should_redirect(req) {
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

        match requested_path.get_file() {
            Some(file) =>
                match Response::from_file(&file) {
                    Ok(response) => {
                        debug!("Serving static file at {}", file.display());
                        return Ok(response);
                    },
                    Err(err) => {
                        return Err(FileError(err).erase());
                    }
                },

            None =>
                // If no file is found, return a 404 response.
                return Ok(Response::with(status::NotFound, "File not found")),
        }
    }
}
