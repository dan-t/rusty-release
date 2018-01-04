#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate toml;
extern crate semver;
extern crate term;
extern crate tempfile;

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

#[macro_use]
mod utils;

fn main() {
    let mut exit_code = 0;
    execute().unwrap_or_else(|err| {
        match err {
            RrError::Message(_) => {
                stderrln!("{}", err);
                exit_code = 1;
            }

            RrError::ClapDisplaysInfo(_) => {
                exit_code = 2;
            }
        }
    });

    std::process::exit(exit_code);
}

fn execute() -> RrResult<()> {
    let config = Config::from_file_and_command_args()?;
    let mut cargo_proj = CargoProj::find(&config.start_dir)?;
    std::env::set_current_dir(cargo_proj.root_dir()?)?;

    stdoutln!("Checking git state ...");
    git::check_state()?;

    stdoutln!("Testing ...");
    cargo::test()?;

    let curr_version = cargo_proj.version().clone();
    let tag_name_curr_version = config.tag_name(&cargo_proj);

    let new_version = config.version_kind.increment(&curr_version);
    cargo_proj.write_version(&new_version)?;

    stdoutln!("Building release ...");
    cargo::build_release()?;

    if let Some(changelog) = cargo_proj.changelog() {
        stdoutln!("Updating changelog ...");
        update_changelog(config.editor(), changelog, &tag_name_curr_version, &new_version)?;
    }

    if git::has_dirty_working_dir()? {
        stdoutln!("Creating git commit ...");
        git::add_update()?;
        git::commit(&config.commit_message(&cargo_proj))?;
    }

    stdoutln!("Creating git tag ...");
    git::tag(&config.tag_name(&cargo_proj))?;

    if config.git_push {
        stdoutln!("Pushing git changes ...");
        git::push()?;
    }

    if config.cargo_publish {
        stdoutln!("Publishing to crates.io ...");
        cargo::publish()?;
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
    modify_file(changelog, |contents| { format!("{}\n\n{}", new_version, contents) })?;

    let log_to = if git::has_tag(tag_name_curr_version)? {
        Some(tag_name_curr_version)
    } else {
        None
    };

    let log_file = git::log_file("HEAD", log_to)?;

    let output = editor_cmd.arg(changelog)
        .arg(log_file.path())
        .output()?;

    check_output(&output)?;
    Ok(())
}
