use std::io::Read;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use toml;
use semver::Version;
use cr_result::{CrResult, err_message, cr_err_message};

#[derive(Debug)]
pub struct CargoToml {
    /// the path to the `Cargo.toml`
    path: PathBuf,

    /// the parsed toml value from `path`
    pub value: toml::Value
}

impl CargoToml {
    /// Searches for a `Cargo.toml` file starting at `start_dir` and continuing the search upwards the
    /// directory tree until the file is found.
    pub fn find(start_dir: &Path) -> CrResult<CargoToml> {
        let path = try!(find_cargo_toml(start_dir));
        Ok(try!(CargoToml::new(path)))
    }

    /// Creates a `CargoToml` from the `Cargo.toml` located at `cargo_toml`.
    pub fn new<P: Into<PathBuf>>(cargo_toml: P) -> CrResult<CargoToml> {
        let path = cargo_toml.into();
        let table = try!(parse_toml(&path));
        Ok(CargoToml { path: path, value: toml::Value::Table(table) })
    }

    /// Returns the name of the cargo project.
    pub fn project_name(&self) -> CrResult<&str> {
        self.value.lookup("package.name")
            .and_then(toml::Value::as_str)
            .ok_or_else(|| cr_err_message(format!("Couldn't get 'package.name' string from: {:?}", self.value)))
    }

    /// Returns the version of the cargo project.
    pub fn project_version(&self) -> CrResult<Version> {
        let vers_str = try!(self.value.lookup("package.version")
            .and_then(toml::Value::as_str)
            .ok_or_else(|| cr_err_message(format!("Couldn't get 'package.version' string from: {:?}", self.value))));

        let vers = try!(Version::parse(vers_str));
        Ok(vers)
    }

    /// Sets the version of the cargo version. This only changes
    /// the version in the internal representation of `CargoToml`,
    /// the `Cargo.toml` isn't modified until `write` is called.
    pub fn set_project_version(&mut self, version: &Version) {
        set_value_at_path(&mut self.value,
                          &toml::Value::String(format!("{}", version)),
                          &["package", "version"]);
    }
}

/// Searches for a `Cargo.toml` file starting at `start_dir` and continuing the search upwards the
/// directory tree until the file is found.
fn find_cargo_toml(start_dir: &Path) -> CrResult<PathBuf> {
    let mut dir = start_dir.to_path_buf();
    loop {
        if let Ok(files) = fs::read_dir(&dir) {
            for file in files {
                if let Ok(file) = file {
                    if file.path().is_file() {
                        if let Some("Cargo.toml") = file.path().file_name().and_then(|s| s.to_str()) {
                            return Ok(file.path().to_path_buf());
                        }
                    }
                }
            }
        }

        if ! dir.pop() {
            return err_message(format!("Couldn't find 'Cargo.toml' starting at directory '{}'!", start_dir.display()));
        }
    }
}

fn parse_toml(path: &Path) -> CrResult<toml::Table> {
    let mut file = try!(File::open(path));
    let mut string = String::new();
    try!(file.read_to_string(&mut string));
    let mut parser = toml::Parser::new(&string);
    parser.parse().ok_or_else(|| cr_err_message(format!("Couldn't parse toml file '{}': {:?}", path.display(), parser.errors)))
}

fn set_value_at_path(toml: &mut toml::Value, value: &toml::Value, path: &[&str]) {
    if path.len() == 0 {
        return
    }

    match *toml {
        toml::Value::Table(ref mut t) => {
            match t.get_mut(path[0]) {
                Some(v) => {
                    if path.len() == 1 {
                        *v = value.clone();
                    } else {
                        set_value_at_path(v, value, path.split_at(1).1);
                    }
                },

                _ => return
            }
        },

        toml::Value::Array(ref mut a) => {
            match path[0].parse::<usize>().ok() {
                Some(idx) if idx < a.len() => {
                    match a.get_mut(idx) {
                        Some(v) => {
                            if path.len() == 1 {
                                *v = value.clone();
                            } else {
                                set_value_at_path(v, value, path.split_at(1).1);
                            }
                        },

                        _ => return
                    }
                },

                _ => return
            }
        },

        _ => return
    }
}
