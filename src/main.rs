#[macro_use]
extern crate clap;
extern crate toml;
extern crate semver;

use std::io::{self, Write};
use cr_result::CrResult;
use config::Config;
use cargo_proj::CargoProj;

mod git;
mod cr_result;
mod version_kind;
mod config;
mod cargo_proj;
mod cargo;

fn main() {
    execute().unwrap_or_else(|err| {
        writeln!(&mut io::stderr(), "{}", err).unwrap();
        std::process::exit(1);
    });
}

fn execute() -> CrResult<()> {
    let config = try!(Config::from_command_args());
    let mut cargo_proj = try!(CargoProj::find(&config.start_dir));

    try!(std::env::set_current_dir(try!(cargo_proj.root_dir())));

    println!("Checking git state ...");
    try!(git::check_clean_state());

    let version = config.version_kind.increment(cargo_proj.version());
    try!(cargo_proj.write_version(&version));

    println!("Building release ...");
    try!(cargo::build_release());

    println!("Testing ...");
    try!(cargo::test());

    Ok(())
}
