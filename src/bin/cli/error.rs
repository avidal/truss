use std::error;
use std::io;
use std::fmt;

use toml;

#[derive(Debug)]
pub enum ErrorKind {
    FileNotFound,
    ParseError(toml::Error),
    IoError(io::Error),
}

#[derive(Debug)]
pub struct TrussCliError {
    pub kind: ErrorKind,
    pub detail: Option<String>,
}

impl error::Error for TrussCliError {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::FileNotFound => "configuration file could not be found",
            ErrorKind::ParseError(_) => "a parsing error occurred with the config file",
            ErrorKind::IoError(_) => "an I/O error occurred",
        }
    }
}

impl fmt::Display for TrussCliError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		"beep borp".fmt(f)
	}
}

impl From<io::Error> for TrussCliError {
    fn from(err: io::Error) -> TrussCliError {
        TrussCliError {
            kind: ErrorKind::IoError(err),
            detail: None,
        }
    }
}

impl From<toml::Error> for TrussCliError {
    fn from(err: toml::Error) -> TrussCliError {
        TrussCliError {
            kind: ErrorKind::ParseError(err),
            detail: None,
        }
    }
}
