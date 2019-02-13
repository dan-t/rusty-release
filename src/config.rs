use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use clap::{App, Arg};
use toml;
use dirs;
use rr_result::RrResult;
use version_kind::VersionKind;
use utils::map_file;
use cargo_proj::CargoProj;

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
    pub git_push: bool,

    /// string template for the commit message
    commit_message: String,

    /// string template for the tag name
    tag_name: String,

    /// the editor command, used for opening of the changelog
    editor: String
}

/// Helper macro to apply the settings from ConfigFromFile to Config
macro_rules! config {
    ( $file_config:ident, [ $( $field_name:ident ),* ]) => {{
        let mut config = Config::default()?;
        $(
            if let Some(f) = $file_config.$field_name {
                config.$field_name = f;
            }
        )*

        config
    }}
}

impl Config {
   pub fn from_file_and_command_args() -> RrResult<Config> {
       let matches = App::new("rusty-release")
           .about("Make a release for a cargo project")
           .version(crate_version!())
           .author("Daniel Trstenjak <daniel.trstenjak@gmail.com>")
           .arg_from_usage("<VERSION_KIND> 'Which version number gets increased (major, minor, patch or current)'")
           .arg(Arg::with_name("start-dir")
                .short("s")
                .long("start-dir")
                .value_names(&["DIR"])
                .help("Start directory for the search of the Cargo.toml (default: current working directory)")
                .takes_value(true))
           .arg_from_usage("-n --no-cargo-publish 'Do not publish to crates.io'")
           .arg_from_usage("-N --no-git-push 'Do not push to remote git repository'")
           .get_matches();

       let start_dir = matches.value_of("start-dir")
           .map(PathBuf::from)
           .unwrap_or(env::current_dir()?);

       if ! start_dir.is_dir() {
           return Err(format!("Invalid directory given to '--start-dir': '{}'!", start_dir.display()).into());
       }

       let mut config = Config::from_file()?;
       config.version_kind = value_t_or_exit!(matches.value_of("VERSION_KIND"), VersionKind);
       config.start_dir = start_dir;

       if matches.is_present("no-cargo-publish") {
           config.cargo_publish = ! matches.is_present("no-cargo-publish");
       }

       if matches.is_present("no-git-push") {
           config.git_push = ! matches.is_present("no-git-push");
       }

       config.check()?;
       Ok(config)
   }

   pub fn commit_message(&self, proj: &CargoProj) -> String {
       Template(&self.commit_message).render(proj)
   }

   pub fn tag_name(&self, proj: &CargoProj) -> String {
       Template(&self.tag_name).render(proj)
   }

   pub fn editor(&self) -> Command {
       let editor_and_args = self.editor.split(' ').collect::<Vec<&str>>();
       let mut cmd = Command::new(editor_and_args[0]);
       let args = editor_and_args.iter().skip(1);
       for arg in args {
           cmd.arg(arg);
       }

       cmd
   }

   fn from_file() -> RrResult<Config> {
       let curr_file_config = ConfigFromFile::load_from_current_dir()?;
       let home_file_config = ConfigFromFile::load_from_home_dir()?;

       let file_config = match (curr_file_config, home_file_config) {
           (Some(cfc), Some(hfc)) => cfc.combine(&hfc),
           (Some(cfc), None)      => cfc,
           (None     , Some(hfc)) => hfc,
           (None     , None)      => ConfigFromFile::default()
       };

       let config = config!(file_config, [
           cargo_publish,
           git_push,
           commit_message,
           tag_name,
           editor
       ]);

       Ok(config)
   }

   fn default() -> RrResult<Config> {
       Ok(Config {
           version_kind: VersionKind::Patch,
           start_dir: env::current_dir()?,
           cargo_publish: true,
           git_push: true,
           commit_message: "<PROJ_NAME> <NEW_VERSION>".to_string(),
           tag_name: "v<NEW_VERSION>".to_string(),
           editor: {
               if let Ok(editor) = env::var("EDITOR") {
                   editor
               } else if let Ok(visual) = env::var("VISUAL") {
                   visual
               } else {
                   "gvim -o".to_string()
               }
           }
       })
   }

   fn check(&self) -> RrResult<()> {
       if self.commit_message.is_empty() {
           return Err("Invalid, empty commit message!".into());
       }

       if self.tag_name.is_empty() {
           return Err("Invalid empty tag name!".into());
       }

       if self.editor.is_empty() {
           return Err("Invalid, empty editor command!".into());
       }

       Ok(())
   }
}

/// Represents a string template that contains placeholders that can be replaced.
/// Currently only the placeholders '<PROJ_NAME>' - representing the name of the
/// cargo project and '<NEW_VERSION>' - representing the version of the release -
/// are supported.
#[derive(Debug)]
struct Template<'a>(&'a str);

impl<'a> Template<'a> {
    fn render(&self, proj: &CargoProj) -> String {
        self.0.replace("<PROJ_NAME>", &proj.name().to_string())
            .replace("<NEW_VERSION>", &proj.version().to_string())
    }
}

/// Represents the data from a `.rusty-release.toml` configuration file.
#[derive(Deserialize, Debug, Default)]
struct ConfigFromFile {
    cargo_publish: Option<bool>,
    git_push: Option<bool>,
    commit_message: Option<String>,
    tag_name: Option<String>,
    editor: Option<String>
}

impl ConfigFromFile {
    fn load_from_current_dir() -> RrResult<Option<ConfigFromFile>> {
        let path = env::current_dir()?.join(config_file_name());
        if ! path.is_file() {
            return Ok(None);
        }

        Ok(Some(ConfigFromFile::load_from_file(&path)?))
    }

    fn load_from_home_dir() -> RrResult<Option<ConfigFromFile>> {
        if let Some(path) = dirs::home_dir().map(|d| d.join(config_file_name())) {
            if path.is_file() {
                return Ok(Some(ConfigFromFile::load_from_file(&path)?));
            }
        }

        Ok(None)
    }

    fn load_from_file(path: &Path) -> RrResult<ConfigFromFile> {
        map_file(path, |contents| {
            let config = toml::from_str(&contents)?;
            Ok(config)
        })
    }

    fn combine(&self, other: &ConfigFromFile) -> ConfigFromFile {
        ConfigFromFile {
            cargo_publish: self.cargo_publish.or(other.cargo_publish),
            git_push: self.git_push.or(other.git_push),
            commit_message: self.commit_message.as_ref().or(other.commit_message.as_ref()).cloned(),
            tag_name: self.tag_name.as_ref().or(other.tag_name.as_ref()).cloned(),
            editor: self.editor.as_ref().or(other.editor.as_ref()).cloned()
        }
    }
}

fn config_file_name() -> &'static str {
    ".rusty-release.toml"
}
