#[macro_use]
extern crate clap;
extern crate toml;
extern crate semver;

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
    let mut cargo_toml = try!(CargoToml::find(&config.start_dir));
    println!("{:?}", cargo_toml);
    println!("project_name: {}", cargo_toml.project_name().unwrap());
    println!("project_version: {}", cargo_toml.project_version().unwrap());
    let new_vers = config.version_kind.increment(cargo_toml.project_version().unwrap());
    cargo_toml.set_project_version(&new_vers);
    println!("{}", cargo_toml.value);
    let _ = try!(git::check_clear_state());
    Ok(())
}
