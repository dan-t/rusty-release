use std::io;
use std::convert::From;
use std::fmt::{self, Display, Formatter};
use semver::SemVerError;
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

pub fn err_message<M: Into<String>, T>(msg: M) -> RrResult<T> {
    Err(rr_error_message(msg))
}

pub fn rr_error_message<M: Into<String>>(msg: M) -> RrError {
    RrError::Message(msg.into())
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
        rr_error_message(format!("{}", err))
    }
}

impl From<SemVerError> for RrError {
    fn from(err: SemVerError) -> RrError {
        match err {
            SemVerError::ParseError(err) => rr_error_message(err)
        }
    }
}

impl From<term::Error> for RrError {
    fn from(err: term::Error) -> RrError {
        rr_error_message(format!("{}", err))
    }
}

impl From<clap::Error> for RrError {
    fn from(err: clap::Error) -> RrError {
        let msg = format!("{}", err);
        match err.kind {
            clap::ErrorKind::HelpDisplayed | clap::ErrorKind::VersionDisplayed
                => RrError::ClapDisplaysInfo(msg),
            _   => RrError::Message(msg)
        }
    }
}
