use std::io::Write;
use std::process::Command;
use tempfile::{NamedTempFile, NamedTempFileOptions};
use rr_result::{RrResult, err_message};
use utils::check_output;

/// Checks if git has a clean state, a non dirty working directory,
/// an empty stage area and a non diverging local and remote git repository.
pub fn check_clean_state() -> RrResult<()> {
    if try!(has_dirty_working_dir()) {
        return err_message("Can't operate with dirty git working directory! Clear or commit changes!");
    }

    if try!(has_staged_changes()) {
        return err_message("Can't operate with non empty git staging area! Clear or commit staged changes!");
    }

    let local_head = try!(local_head());
    try!(remote_update());
    let remote_head = try!(remote_head());
    if local_head != remote_head {
        return err_message("Can't operate with diverging local and remote git repository! Synchronize them!")
    }

    Ok(())
}

pub fn add_update() -> RrResult<()> {
    let output = try!(Command::new("git")
        .arg("add")
        .arg("--update")
        .output());

    try!(check_output(&output));
    Ok(())
}

pub fn commit(msg: &str) -> RrResult<()> {
    let output = try!(Command::new("git")
        .arg("commit")
        .arg(format!("--message={}", msg))
        .output());

    try!(check_output(&output));
    Ok(())
}

pub fn tag(name: &str) -> RrResult<()> {
    let output = try!(Command::new("git")
        .arg("tag")
        .arg(name)
        .output());

    try!(check_output(&output));
    Ok(())
}

pub fn push() -> RrResult<()> {
    let output = try!(Command::new("git")
        .arg("push")
        .output());

    try!(check_output(&output));

    let output = try!(Command::new("git")
        .arg("push")
        .arg("--tags")
        .output());

    try!(check_output(&output));
    Ok(())
}

pub fn log_file(from: &str, to: Option<&str>) -> RrResult<NamedTempFile> {
    let output = try!(log(from, to));
    let mut log_file = try!(NamedTempFileOptions::new()
        .prefix(&format!("{}...{}___", from, to.unwrap_or("")))
        .create());

    try!(log_file.write_all(output.as_bytes()));
    Ok(log_file)
}

pub fn log(from: &str, to: Option<&str>) -> RrResult<String> {
    let output = try!(Command::new("git")
        .arg("--no-pager")
        .arg("log")
        .arg("--decorate=short")
        .arg("--pretty=oneline")
        .arg("--abbrev-commit")
        .arg(if let Some(to) = to { format!("{}...{}", from, to) } else { from.to_owned() })
        .output());

    try!(check_output(&output));
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn has_tag(name: &str) -> RrResult<bool> {
    let output = try!(Command::new("git")
        .arg("rev-parse")
        .arg("--verify")
        .arg("--quiet")
        .arg(name)
        .output());

    Ok(output.status.success())
}

/// If the working directory has uncommited changes.
fn has_dirty_working_dir() -> RrResult<bool> {
    let output = try!(Command::new("git")
        .arg("diff-files")
        .arg("--quiet")
        .arg("--exit-code")
        .output());

    Ok(output.status.code() == Some(1))
}

/// If the stage area contains uncommited changes.
fn has_staged_changes() -> RrResult<bool> {
    let output = try!(Command::new("git")
        .arg("diff-index")
        .arg("--quiet")
        .arg("--exit-code")
        .arg("--cached HEAD")
        .output());

    Ok(output.status.code() == Some(1))
}

/// Update the local refs to the remote repository.
fn remote_update() -> RrResult<()> {
    let output = try!(Command::new("git")
        .arg("remote")
        .arg("update")
        .output());

    try!(check_output(&output));
    Ok(())
}

type CommitHash = String;

fn local_head() -> RrResult<CommitHash> {
    commit_hash("@")
}

fn remote_head() -> RrResult<CommitHash> {
    commit_hash("@{u}")
}

fn commit_hash(refname: &str) -> RrResult<CommitHash> {
    let output = try!(Command::new("git")
        .arg("rev-parse")
        .arg(refname)
        .output());

    try!(check_output(&output));
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
