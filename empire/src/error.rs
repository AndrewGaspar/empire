use std;
use std::ffi::OsString;

use tokio;

#[derive(Debug)]
pub enum Error {
    CommandNotFound(OsString),
    NoSuchPort(String),
    IoError(std::io::Error),
    TokioIoError(tokio::io::Error),
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::CommandNotFound(_) => "empire could not find the requested command",
            &Error::NoSuchPort(_) => "empire could not find a port with the given name",
            &Error::IoError(ref err) => err.description(),
            &Error::TokioIoError(ref err) => err.description(),
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
            &Error::NoSuchPort(ref port_name) => {
                write!(f, "empire could not find the port '{}'", port_name)
            }
            &Error::IoError(ref err) => err.fmt(f),
            _ => {
                use std::error::Error;
                write!(f, "{}", self.description())
            }
            &Error::TokioIoError(ref err) => err.fmt(f),
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

// impl From<tokio::io::Error> for Error {
//     fn from(err: tokio::io::Error) -> Self {
//         Error::TokioIoError(err)
//     }
// }

pub type Result<T> = std::result::Result<T, Error>;
