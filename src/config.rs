use std::env;
use std::path::PathBuf;
use clap::{App, Arg};
use rr_result::{RrResult, err_message};
use version_kind::VersionKind;

/// The configuration used to run `rusty-release`.
#[derive(Debug)]
pub struct Config {
    /// which part of the project version should be incremented
    pub version_kind: VersionKind,

    /// start directory for the search of the 'Cargo.toml'
    pub start_dir: PathBuf,

    /// publish to crates.io
    pub cargo_publish: bool,

    /// push to git remote repository
    pub git_push: bool
}

impl Config {
   pub fn from_command_args() -> RrResult<Config> {
       let matches = try!(App::new("rusty-release")
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
           .arg_from_usage("-n --no-cargo-publish 'Do not publish to crates.io'")
           .arg_from_usage("-N --no-git-push 'Do not push to remote git repository'")
           .get_matches_safe());

       let start_dir = matches.value_of("start-dir")
           .map(PathBuf::from)
           .unwrap_or(try!(env::current_dir()));

       if ! start_dir.is_dir() {
           return err_message(format!("Invalid directory given to '--start-dir': '{}'!", start_dir.display()));
       }

       Ok(Config {
           version_kind: value_t_or_exit!(matches.value_of("VERSION_KIND"), VersionKind),
           start_dir: start_dir,
           cargo_publish: ! matches.is_present("no-cargo-publish"),
           git_push: ! matches.is_present("no-git-push")
       })
   }
}
