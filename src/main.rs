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

fn main() {
    execute().unwrap_or_else(|err| {
        writeln!(&mut io::stderr(), "{}", err).unwrap();
        std::process::exit(1);
    });
}

fn execute() -> CrResult<()> {
    let config = try!(Config::from_command_args());
    println!("{:?}", config);
    let cargo_proj = try!(CargoProj::find(&config.start_dir));
    println!("{:?}", cargo_proj);
    let _ = try!(git::check_clean_state());
    Ok(())
}
