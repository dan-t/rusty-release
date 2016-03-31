use std::io::Read;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use toml;
use cr_result::{CrResult, err_message, cr_err_message};

#[derive(Debug)]
pub struct CargoToml {
    /// the path to the `Cargo.toml`
    path: PathBuf,

    /// the parsed toml table from `path`
    table: toml::Table
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
        Ok(CargoToml { path: path, table: table })
    }

    /// Returns the name of the cargo project.
    pub fn project_name(&self) -> CrResult<&str> {
        let package = try!(self.package_table());
        package.get("name")
            .and_then(toml::Value::as_str)
            .ok_or_else(|| cr_err_message(format!("Couldn't get 'name' string from: {:?}", package)))
    }

    fn package_table(&self) -> CrResult<&toml::Table> {
        self.table.get("package")
            .and_then(toml::Value::as_table)
            .ok_or_else(|| cr_err_message(format!("Couldn't get 'package' table from: {:?}", self.table)))
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
