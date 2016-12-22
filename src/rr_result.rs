use std::io;
use std::convert::From;
use std::fmt::{self, Display, Formatter};
use semver::SemVerError;
use toml::DecodeError;
use clap;
use term;

/// The result type used in `rusty-release`.
pub type RrResult<T> = Result<T, RrError>;

/// The error type used in `rusty-release`.
#[derive(Clone, Debug)]
pub enum RrError {
    /// generic error message
    Message(String),

    /// not a real error but clap - the command argument handler -
    /// displays some info like '--help' or '--version'
    ClapDisplaysInfo(String)
}

impl Display for RrError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            RrError::Message(ref msg)          => writeln!(f, "{}", msg),
            RrError::ClapDisplaysInfo(ref msg) => writeln!(f, "{}", msg)
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

impl From<clap::Error> for RrError {
    fn from(err: clap::Error) -> RrError {
        let msg = err.to_string();
        match err.kind {
            clap::ErrorKind::HelpDisplayed | clap::ErrorKind::VersionDisplayed
                => RrError::ClapDisplaysInfo(msg),
            _   => RrError::Message(msg)
        }
    }
}

impl From<DecodeError> for RrError {
    fn from(err: DecodeError) -> RrError {
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
