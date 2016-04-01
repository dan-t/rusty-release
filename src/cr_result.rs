use std::io;
use std::convert::From;
use std::fmt::{self, Display, Formatter};
use semver::SemVerError;
use term;

/// The result type used in `cargo-release`.
pub type CrResult<T> = Result<T, CrErr>;

/// The error type used in `cargo-release`.
#[derive(Clone, Debug)]
pub enum CrErr {
    /// generic error message
    Message(String)
}

pub fn err_message<M: Into<String>, T>(msg: M) -> CrResult<T> {
    Err(cr_err_message(msg))
}

pub fn cr_err_message<M: Into<String>>(msg: M) -> CrErr {
    CrErr::Message(msg.into())
}

impl Display for CrErr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            CrErr::Message(ref msg) => writeln!(f, "{}", msg),
        }
    }
}

impl From<io::Error> for CrErr {
    fn from(err: io::Error) -> CrErr {
        cr_err_message(format!("{}", err))
    }
}

impl From<SemVerError> for CrErr {
    fn from(err: SemVerError) -> CrErr {
        match err {
            SemVerError::ParseError(err) => cr_err_message(err)
        }
    }
}

impl From<term::Error> for CrErr {
    fn from(err: term::Error) -> CrErr {
        cr_err_message(format!("{}", err))
    }
}
