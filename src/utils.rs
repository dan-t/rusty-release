use std::process::Output;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use cr_result::{CrResult, err_message};

pub fn check_output(out: &Output) -> CrResult<()> {
    if out.status.success() {
        return Ok(());
    }

    err_message(String::from_utf8_lossy(&out.stderr).into_owned())
}

/// Reads `file` into a string which is passed to the function `f`
/// and the returned string of `f` is written back into `file`.
pub fn modify_file<F>(file: &Path, f: F) -> CrResult<()>
    where F: FnOnce(String) -> String
{
    let mut file = try!(OpenOptions::new()
        .read(true)
        .write(true)
        .open(file));

    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));

    let contents = f(contents);

    // truncate file
    try!(file.set_len(0));

    try!(file.write_all(contents.as_bytes()));
    Ok(())
}

/// Reads `file` into a string which is passed to the function `f`
/// and its return value is returned by `map_file`.
pub fn map_file<R, F>(file: &Path, f: F) -> CrResult<R>
    where F: FnOnce(String) -> CrResult<R>
{
    let mut file = try!(File::open(file));

    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));

    let r = try!(f(contents));
    Ok(r)
}
