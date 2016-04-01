#[macro_use]
extern crate clap;
extern crate toml;
extern crate semver;
extern crate term;

use std::io::{self, Write};
use cr_result::{CrResult, cr_err_message};
use config::Config;
use cargo_proj::CargoProj;

mod git;
mod cr_result;
mod version_kind;
mod config;
mod cargo_proj;
mod cargo;
mod utils;

fn main() {
    execute().unwrap_or_else(|err| {
        let mut stderr = term::stderr().unwrap();
        stderr.fg(term::color::RED).unwrap();

        writeln!(stderr, "{}", err).unwrap();
        std::process::exit(1);
    });
}

fn execute() -> CrResult<()> {
    let mut stdout = try!(term::stdout()
        .ok_or_else(|| cr_err_message("Couldn't get stdout of terminal!")));

    try!(stdout.fg(term::color::GREEN));

    let config = try!(Config::from_command_args());
    let mut cargo_proj = try!(CargoProj::find(&config.start_dir));

    try!(std::env::set_current_dir(try!(cargo_proj.root_dir())));

    try!(writeln!(stdout, "Checking git state ..."));
    try!(git::check_clean_state());

    let version = config.version_kind.increment(cargo_proj.version());
    try!(cargo_proj.write_version(&version));

    try!(writeln!(stdout, "Building release ..."));
    try!(cargo::build_release());

    try!(writeln!(stdout, "Testing ..."));
    try!(cargo::test());

    Ok(())
}
