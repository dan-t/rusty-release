use std::io;
use std::convert::From;
use std::fmt::{self, Display, Formatter};
use semver::SemVerError;
use term;
use toml;

/// The result type used in `rusty-release`.
pub type RrResult<T> = Result<T, RrError>;

/// The error type used in `rusty-release`.
#[derive(Clone, Debug)]
pub enum RrError {
    /// generic error message
    Message(String)
}

impl Display for RrError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            RrError::Message(ref msg) => writeln!(f, "{}", msg),
        }
    }
}

impl From<io::Error> for RrError {
    fn from(err: io::Error) -> RrError {
        RrError::Message(err.to_string())
    }
}

impl From<SemVerError> for RrError {
    fn from(err: SemVerError) -> RrError {
        match err {
            SemVerError::ParseError(err) => RrError::Message(err)
        }
    }
}

impl From<term::Error> for RrError {
    fn from(err: term::Error) -> RrError {
        RrError::Message(err.to_string())
    }
}

impl From<toml::de::Error> for RrError {
    fn from(err: toml::de::Error) -> RrError {
        RrError::Message(err.to_string())
    }
}

impl From<String> for RrError {
    fn from(s: String) -> RrError {
        RrError::Message(s)
    }
}

impl<'a> From<&'a str> for RrError {
    fn from(s: &str) -> RrError {
        RrError::Message(s.to_owned())
    }
}
