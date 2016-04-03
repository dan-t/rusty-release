use std::env;
use std::path::{Path, PathBuf};
use clap::{App, Arg};
use toml;
use rustc_serialize::Decodable;
use rr_result::{RrResult, err_message, rr_error_message};
use version_kind::VersionKind;
use utils::map_file;

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
   pub fn from_file_and_command_args() -> RrResult<Config> {
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

       let mut config = try!(Config::from_file());
       config.version_kind = value_t_or_exit!(matches.value_of("VERSION_KIND"), VersionKind);
       config.start_dir = start_dir;

       if matches.is_present("no-cargo-publish") {
           config.cargo_publish = ! matches.is_present("no-cargo-publish");
       }

       if matches.is_present("no-git-push") {
           config.git_push = ! matches.is_present("no-git-push");
       }

       Ok(config)
   }

   fn from_file() -> RrResult<Config> {
       let curr_file_config = try!(ConfigFromFile::load_from_current_dir());
       let home_file_config = try!(ConfigFromFile::load_from_home_dir());

       let file_config = match (curr_file_config, home_file_config) {
           (Some(cfc), Some(hfc)) => cfc.combine(&hfc),
           (Some(cfc), None)      => cfc,
           (None     , Some(hfc)) => hfc,
           (None     , None)      => ConfigFromFile::default()
       };

       let mut config = try!(Config::default());
       if let Some(p) = file_config.cargo_publish {
           config.cargo_publish = p;
       }

       if let Some(p) = file_config.git_push {
           config.git_push = p;
       }

       Ok(config)
   }

   fn default() -> RrResult<Config> {
       Ok(Config {
           version_kind: VersionKind::Patch,
           start_dir: try!(env::current_dir()),
           cargo_publish: true,
           git_push: true
       })
   }
}

/// Represents the data from a `.rusty-release.toml` configuration file.
#[derive(RustcDecodable, Debug)]
struct ConfigFromFile {
    pub cargo_publish: Option<bool>,
    pub git_push: Option<bool>
}

impl ConfigFromFile {
    fn load_from_current_dir() -> RrResult<Option<ConfigFromFile>> {
        let path = try!(env::current_dir()).join(config_file_name());
        if ! path.is_file() {
            return Ok(None);
        }

        Ok(Some(try!(ConfigFromFile::load_from_file(&path))))
    }

    fn load_from_home_dir() -> RrResult<Option<ConfigFromFile>> {
        if let Some(path) = env::home_dir().map(|d| d.join(config_file_name())) {
            if path.is_file() {
                return Ok(Some(try!(ConfigFromFile::load_from_file(&path))));
            }
        }

        Ok(None)
    }

    fn load_from_file(path: &Path) -> RrResult<ConfigFromFile> {
        map_file(path, |contents| {
            let mut parser = toml::Parser::new(&contents);
            let value = try!(parser.parse()
                .ok_or_else(|| rr_error_message(format!("Couldn't parse toml file '{}': {:?}",
                                                        path.display(), parser.errors))));

            let mut decoder = toml::Decoder::new(toml::Value::Table(value));
            Ok(try!(ConfigFromFile::decode(&mut decoder)))
        })
    }

    fn combine(&self, other: &ConfigFromFile) -> ConfigFromFile {
        ConfigFromFile {
            cargo_publish: self.cargo_publish.or(other.cargo_publish),
            git_push: self.git_push.or(other.git_push)
        }
    }

    fn default() -> ConfigFromFile {
        ConfigFromFile {
            cargo_publish: None,
            git_push: None
        }
    }
}

fn config_file_name() -> &'static str {
    ".rusty-release.toml"
}
