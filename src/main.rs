#[macro_use]
extern crate clap;
extern crate toml;
extern crate semver;
extern crate term;

use std::io::{Read, Write};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::process::Command;
use semver::Version;
use cr_result::CrResult;
use config::Config;
use cargo_proj::CargoProj;
use utils::check_output;

mod git;
mod cr_result;
mod version_kind;
mod config;
mod cargo_proj;
mod cargo;
mod utils;

fn main() {
    set_stdout_stderr_colors();

    let mut exit_code = 0;
    execute().unwrap_or_else(|err| {
        let mut stderr = term::stderr().unwrap();
        writeln!(stderr, "{}", err).unwrap();
        exit_code = 1;
    });

    reset_stdout_stderr();
    std::process::exit(exit_code);
}

fn execute() -> CrResult<()> {
    let mut stdout = term::stdout().unwrap();

    let config = try!(Config::from_command_args());
    let mut cargo_proj = try!(CargoProj::find(&config.start_dir));

    try!(std::env::set_current_dir(try!(cargo_proj.root_dir())));

    try!(writeln!(stdout, "Checking git state ..."));
    try!(git::check_clean_state());

    try!(writeln!(stdout, "Testing ..."));
    try!(cargo::test());

    let curr_version = cargo_proj.version().clone();
    let new_version = config.version_kind.increment(&curr_version);
    try!(cargo_proj.write_version(&new_version));

    try!(writeln!(stdout, "Building release ..."));
    try!(cargo::build_release());

    if let Some(changelog) = cargo_proj.changelog() {
        try!(writeln!(stdout, "Updating changelog ..."));
        try!(update_changelog(changelog, &new_version));
    }

    try!(writeln!(stdout, "Creating git commit ..."));
    try!(git::add_update());
    try!(git::commit(&commit_message(&cargo_proj)));
    try!(git::tag(&tag_name(&cargo_proj)));

    try!(writeln!(stdout, "Pushing git changes ..."));
    try!(git::push());

    try!(writeln!(stdout, "Publishing to crates.io ..."));
    try!(cargo::publish());

    Ok(())
}

/// Adds `new_version` at the top of the `changelog` and opens
/// `changelog` in the editor specified by `CARGO_RELEASE_EDITOR` (default: gvim).
fn update_changelog(changelog: &Path, new_version: &Version) -> CrResult<()> {
    let contents = {
        let mut file = try!(File::open(changelog));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents));
        contents
    };

    let contents = format!("{}\n\n{}", new_version, contents);
    {
        let mut file = try!(OpenOptions::new()
            .truncate(true)
            .read(true)
            .write(true)
            .open(changelog));

        try!(file.write_all(contents.as_bytes()));
    }

    let editor = std::env::var("CARGO_RELEASE_EDITOR").unwrap_or("gvim".to_string());
    let output = try!(Command::new(editor)
        .arg(changelog)
        .output());

    try!(check_output(&output));
    Ok(())
}

fn commit_message(proj: &CargoProj) -> String {
    format!("{} {}", proj.name(), proj.version())
}

fn tag_name(proj: &CargoProj) -> String {
    format!("{}-{}", proj.name(), proj.version())
}

fn set_stdout_stderr_colors() {
    let mut stdout = term::stdout().unwrap();
    stdout.fg(term::color::GREEN).unwrap();

    let mut stderr = term::stderr().unwrap();
    stderr.fg(term::color::RED).unwrap();
}

fn reset_stdout_stderr() {
    let mut stdout = term::stdout().unwrap();
    stdout.reset().unwrap();

    let mut stderr = term::stderr().unwrap();
    stderr.reset().unwrap();
}
