use std::env;
use std::path::PathBuf;
use clap::{App, Arg};
use cr_result::{CrResult, err_message};
use version_kind::VersionKind;

/// The configuration used to run `cargo-release`.
#[derive(Debug)]
pub struct Config {
    /// which part of the project version should be increased
    pub version_kind: VersionKind,

    /// start directory for the search of the 'Cargo.toml'
    pub start_dir: PathBuf,
}

impl Config {
   pub fn from_command_args() -> CrResult<Config> {
       let matches = App::new("cargo-release")
           .about("Make a release for a cargo project")
           .version(crate_version!())
           .author("Daniel Trstenjak <daniel.trstenjak@gmail.com>")
           .arg_from_usage("<VERSION_KIND> 'Which version number gets increased (major, minor or patch)'")
           .arg(Arg::with_name("start-dir")
                .short("s")
                .long("start-dir")
                .value_names(&["DIR"])
                .help("Start directory for the search of the Cargo.toml (default: current working directory)")
                .takes_value(true))
           .get_matches();

       let start_dir = matches.value_of("start-dir")
           .map(PathBuf::from)
           .unwrap_or(try!(env::current_dir()));

       if ! start_dir.is_dir() {
           return err_message(format!("Invalid directory given to '--start-dir': '{}'!", start_dir.display()));
       }

       Ok(Config {
           version_kind: value_t_or_exit!(matches.value_of("VERSION_KIND"), VersionKind),
           start_dir: start_dir
       })
   }
}
