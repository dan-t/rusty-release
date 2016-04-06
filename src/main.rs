#[macro_use]
extern crate clap;
extern crate toml;
extern crate semver;
extern crate term;
extern crate tempfile;
extern crate rustc_serialize;

use std::io::Write;
use std::path::Path;
use std::process::Command;
use semver::Version;
use rr_result::{RrResult, RrError};
use config::Config;
use cargo_proj::CargoProj;
use utils::{check_output, modify_file};

mod git;
mod rr_result;
mod version_kind;
mod config;
mod cargo_proj;
mod cargo;
mod utils;

fn main() {
    let mut exit_code = 0;
    execute().unwrap_or_else(|err| {
        match err {
            RrError::Message(_) => {
                let mut stderr = term::stderr().unwrap();
                stderr.fg(term::color::RED).unwrap();

                writeln!(stderr, "{}", err).unwrap();
            }

            RrError::ClapDisplaysInfo(_) => {}
        }

        exit_code = 1;
    });

    let mut stdout = term::stdout().unwrap();
    stdout.reset().unwrap();

    let mut stderr = term::stderr().unwrap();
    stderr.reset().unwrap();

    std::process::exit(exit_code);
}

fn execute() -> RrResult<()> {
    let config = try!(Config::from_file_and_command_args());
    let mut cargo_proj = try!(CargoProj::find(&config.start_dir));
    try!(std::env::set_current_dir(try!(cargo_proj.root_dir())));

    let mut stdout = term::stdout().unwrap();
    try!(stdout.fg(term::color::GREEN));

    try!(writeln!(stdout, "Checking git state ..."));
    try!(git::check_clean_state());

    try!(writeln!(stdout, "Testing ..."));
    try!(cargo::test());

    let curr_version = cargo_proj.version().clone();
    let tag_name_curr_version = config.tag_name(&cargo_proj);

    let new_version = config.version_kind.increment(&curr_version);
    try!(cargo_proj.write_version(&new_version));

    try!(writeln!(stdout, "Building release ..."));
    try!(cargo::build_release());

    if let Some(changelog) = cargo_proj.changelog() {
        try!(writeln!(stdout, "Updating changelog ..."));
        try!(update_changelog(config.editor(), changelog, &tag_name_curr_version, &new_version));
    }

    try!(writeln!(stdout, "Creating git commit ..."));
    try!(git::add_update());
    try!(git::commit(&config.commit_message(&cargo_proj)));
    try!(git::tag(&config.tag_name(&cargo_proj)));

    if config.git_push {
        try!(writeln!(stdout, "Pushing git changes ..."));
        try!(git::push());
    }

    if config.cargo_publish {
        try!(writeln!(stdout, "Publishing to crates.io ..."));
        try!(cargo::publish());
    }

    Ok(())
}

/// Adds `new_version` at the top of the `changelog` and opens
/// `changelog` and a temporary file containing the commits from
/// HEAD till the last release.
fn update_changelog(mut editor_cmd: Command,
                    changelog: &Path,
                    tag_name_curr_version: &str,
                    new_version: &Version)
                    -> RrResult<()> {
    try!(modify_file(changelog, |contents| { format!("{}\n\n{}", new_version, contents) }));

    let log_file = try!(git::log_file("HEAD", tag_name_curr_version));

    let output = try!(editor_cmd.arg(changelog)
        .arg(log_file.path())
        .output());

    try!(check_output(&output));
    Ok(())
}
