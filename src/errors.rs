use std::io::IoError;
use iron::Error;

/// The error returned when an IoError occurs during file serving.
#[deriving(Show)]
pub struct FileError(pub IoError);

impl Error for FileError {
    fn name(&self) -> &'static str {
        let &FileError(ref error) = self;
        error.desc
    }
}
