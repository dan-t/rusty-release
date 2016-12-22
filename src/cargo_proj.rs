use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use toml;
use semver::Version;
use rr_result::RrResult;
use utils::{modify_file, map_file};

#[derive(Debug)]
pub struct CargoProj {
    /// the name of the cargo project
    name: String,

    /// the version of the cargo project
    version: Version,

    /// the path to the `Cargo.toml`
    cargo_toml: PathBuf,

    /// the path to an optinal changelog file
    changelog: Option<PathBuf>
}

impl CargoProj {
    /// Searches for the root directory (containing a `Cargo.toml`) of the cargo project starting
    /// at `start_dir` and continuing the search upwards the directory tree until it's found.
    pub fn find(start_dir: &Path) -> RrResult<CargoProj> {
        let cargo_dir = try!(find_cargo_toml_dir(start_dir));

        let cargo_toml = cargo_dir.join("Cargo.toml");
        let toml = try!(parse_toml(&cargo_toml));

        let name = try!(toml.lookup("package.name")
            .and_then(toml::Value::as_str)
            .ok_or(format!("Couldn't get 'package.name' string from: {:?}", toml)));

        let version = {
            let version_str = try!(toml.lookup("package.version")
                .and_then(toml::Value::as_str)
                .ok_or(format!("Couldn't get 'package.version' string from: {:?}", toml)));

            try!(Version::parse(version_str))
        };

        let changelog = try!(find_changelog(&cargo_dir));

        Ok(CargoProj {
            name: name.to_string(),
            version: version,
            cargo_toml: cargo_toml,
            changelog: changelog
        })
    }

    /// The name of the cargo project.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The version of the cargo project.
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Write the new `version` into the `Cargo.toml`.
    pub fn write_version(&mut self, version: &Version) -> RrResult<()> {
        if *version != self.version {
            try!(modify_file(&self.cargo_toml, |contents| {
                contents.replace(&format!("version = \"{}\"", self.version),
                                 &format!("version = \"{}\"", version))
            }));

            self.version = version.clone();
        }

        Ok(())
    }

    /// The root directory of the cargo project.
    pub fn root_dir(&self) -> RrResult<&Path> {
        self.cargo_toml.parent()
            .ok_or(format!("Couldn't get directory of path: {:?}", self.cargo_toml).into())
    }

    pub fn changelog(&self) -> Option<&Path> {
        self.changelog.as_ref().map(|p| p.as_path())
    }
}

/// Convenience macro to read all files from a directory.
///
/// Should be possible to be written as a function, but for
/// now I didn't want to wrangle with rustc to find the
/// correct return type for ther FilterMap iterator adpater.
macro_rules! read_files {
    ($dir:expr) => (
        {
            let dir_entries = try!(fs::read_dir($dir));

            let files = dir_entries.filter_map(|f| {
                match f {
                    Ok(f) => { let p = f.path(); if p.is_file() { Some(p) } else { None } },
                    _ => None
                }
            });

            files
        }
    )
}

/// Searches for a directory containing a `Cargo.toml` file starting at
/// `start_dir` and continuing the search upwards the directory tree
/// until a directory is found.
fn find_cargo_toml_dir(start_dir: &Path) -> RrResult<PathBuf> {
    let mut dir = start_dir.to_path_buf();
    loop {
        for file in read_files!(&dir) {
            if let Some("Cargo.toml") = file.file_name().and_then(OsStr::to_str) {
                return Ok(dir);
            }
        }

        if ! dir.pop() {
            return Err(format!("Couldn't find 'Cargo.toml' starting at directory '{}'!", start_dir.display()).into());
        }
    }
}

/// Searches for an optional changelog file in `dir`.
fn find_changelog(dir: &Path) -> RrResult<Option<PathBuf>> {
    for file in read_files!(dir) {
        if let Some(base_name) = file.file_stem().and_then(OsStr::to_str).map(str::to_lowercase) {
            if base_name == "changelog" {
                return Ok(Some(file));
            }
        }
    }

    Ok(None)
}

fn parse_toml(path: &Path) -> RrResult<toml::Value> {
    map_file(path, |contents| {
        let mut parser = toml::Parser::new(&contents);
        parser.parse()
            .map(toml::Value::Table)
            .ok_or(format!("Couldn't parse toml file '{}': {:?}", path.display(), parser.errors).into())
    })
}
