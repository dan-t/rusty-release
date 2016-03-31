use std::io::{self, Write};
use cr_result::CrResult;

mod git;
mod cr_result;

fn main() {
    execute().unwrap_or_else(|err| {
        writeln!(&mut io::stderr(), "{}", err).unwrap();
        std::process::exit(1);
    });
}

fn execute() -> CrResult<()> {
    let _ = try!(git::check_clear_state());
    Ok(())
}
