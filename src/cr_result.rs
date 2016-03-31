use std::io;
use std::convert::From;
use std::fmt::{self, Display, Formatter};

/// The result type used in `cargo-release`.
pub type CrResult<T> = Result<T, CrErr>;

/// The error type used in `cargo-release`.
#[derive(Clone)]
pub enum CrErr {
    /// generic error message
    Message(String)
}

impl Display for CrErr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            &CrErr::Message(ref msg) => writeln!(f, "{}", msg),
        }
    }
}

impl From<io::Error> for CrErr {
    fn from(err: io::Error) -> CrErr {
        CrErr::Message(format!("{}", err))
    }
}
