use std::io;
use std::convert::From;
use std::fmt::{self, Display, Formatter};
use semver::SemVerError;
use clap;
use term;

/// The result type used in `cargo-release`.
pub type CrResult<T> = Result<T, CrErr>;

/// The error type used in `cargo-release`.
#[derive(Clone, Debug)]
pub enum CrErr {
    /// generic error message
    Message(String),

    /// not a real error but clap - the command argument handler -
    /// displays some info like '--help' or '--version'
    ClapDisplaysInfo(String)
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
            CrErr::Message(ref msg)          => writeln!(f, "{}", msg),
            CrErr::ClapDisplaysInfo(ref msg) => writeln!(f, "{}", msg)
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

impl From<clap::Error> for CrErr {
    fn from(err: clap::Error) -> CrErr {
        let msg = format!("{}", err);
        match err.kind {
            clap::ErrorKind::HelpDisplayed | clap::ErrorKind::VersionDisplayed
                => CrErr::ClapDisplaysInfo(msg),
            _   => CrErr::Message(msg)
        }
    }
}
