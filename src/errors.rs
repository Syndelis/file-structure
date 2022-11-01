use core::fmt;
use std::{
    error::{self}, io,
    path::{PathBuf},
};

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn path_error(path: PathBuf) -> Result<()> {
    Err(Box::new(WrongYamlSubtypeError { path }))
}

pub fn filter_not_found_error(res: io::Result<()>) -> io::Result<()> {
    if let Err(err) = &res {
        if err.kind() == io::ErrorKind::NotFound {
            return Ok(());
        }
    }

    res
}

#[derive(Debug)]
struct WrongYamlSubtypeError {
    path: PathBuf,
}

impl fmt::Display for WrongYamlSubtypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid value for path {}", self.path.display())
    }
}

impl error::Error for WrongYamlSubtypeError {}
