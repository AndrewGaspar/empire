use std;
use std::ffi::OsString;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    CommandNotFound(OsString),
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::CommandNotFound(_) => "empire could not find the requested command",
            &Error::IoError(ref err) => err.description(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            &Error::CommandNotFound(ref command) => write!(
                f,
                "empire could not find the command '{}'",
                command.to_str().unwrap_or("Error converting command name")
            ),
            &Error::IoError(ref err) => err.fmt(f),
            _ => {
                use std::error::Error;
                write!(f, "{}", self.description())
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
