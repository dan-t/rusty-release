#[macro_use]
extern crate clap;
extern crate toml;

use std::io::{self, Write};
use cr_result::CrResult;
use config::Config;
use cargo_toml::CargoToml;

mod git;
mod cr_result;
mod version_kind;
mod config;
mod cargo_toml;

fn main() {
    execute().unwrap_or_else(|err| {
        writeln!(&mut io::stderr(), "{}", err).unwrap();
        std::process::exit(1);
    });
}

fn execute() -> CrResult<()> {
    let config = try!(Config::from_command_args());
    println!("{:?}", config);
    let cargo_toml = try!(CargoToml::find(&config.start_dir));
    println!("{:?}", cargo_toml);
    println!("{:?}", cargo_toml.project_name());
    let _ = try!(git::check_clear_state());
    Ok(())
}
