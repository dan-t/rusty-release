use std::process::Output;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::Path;
use rr_result::RrResult;

pub fn check_output(out: &Output) -> RrResult<()> {
    if out.status.success() {
        return Ok(());
    }

    let mut msg = String::from_utf8_lossy(&out.stderr).into_owned();
    if msg.is_empty() {
        msg = String::from_utf8_lossy(&out.stdout).into_owned();
    }

    Err(msg.into())
}

/// Reads `file` into a string which is passed to the function `f`
/// and the returned string of `f` is written back into `file`.
pub fn modify_file<F>(file: &Path, f: F) -> RrResult<()>
    where F: FnOnce(String) -> String
{
    let mut file = try!(OpenOptions::new()
        .read(true)
        .write(true)
        .open(file));

    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));

    let contents = f(contents);

    try!(file.set_len(contents.as_bytes().len() as u64));
    try!(file.seek(SeekFrom::Start(0)));
    try!(file.write_all(contents.as_bytes()));
    Ok(())
}

/// Reads `file` into a string which is passed to the function `f`
/// and its return value is returned by `map_file`.
pub fn map_file<R, F>(file: &Path, f: F) -> RrResult<R>
    where F: FnOnce(String) -> RrResult<R>
{
    let mut file = try!(File::open(file));

    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));

    let r = try!(f(contents));
    Ok(r)
}

macro_rules! stdoutln {
    ($fmt:expr) => {{
        use term;

        let mut stdout = term::stdout().unwrap();
        stdout.fg(term::color::GREEN).unwrap();

        writeln!(stdout, $fmt).unwrap();

        stdout.reset().unwrap();
    }};

    ($fmt:expr, $($arg:tt)*) => {{
        use term;

        let mut stdout = term::stdout().unwrap();
        stdout.fg(term::color::GREEN).unwrap();

        writeln!(stdout, $fmt, $($arg)*).unwrap();

        stdout.reset().unwrap();
    }};
}

macro_rules! stderrln {
    ($fmt:expr) => {{
        use term;

        let mut stderr = term::stderr().unwrap();
        stderr.fg(term::color::RED).unwrap();

        writeln!(stderr, $fmt).unwrap();

        stderr.reset().unwrap();
    }};

    ($fmt:expr, $($arg:tt)*) => {{
        use term;

        let mut stderr = term::stderr().unwrap();
        stderr.fg(term::color::RED).unwrap();

        writeln!(stderr, $fmt, $($arg)*).unwrap();

        stderr.reset().unwrap();
    }};
}
