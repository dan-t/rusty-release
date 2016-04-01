use std::process::Command;
use cr_result::{CrResult, err_message};
use utils::check_output;

/// Checks if git has a clean state, a non dirty working directory,
/// an empty stage area and a non diverging local and remote git repository.
pub fn check_clean_state() -> CrResult<()> {
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

/// If the working directory has uncommited changes.
fn has_dirty_working_dir() -> CrResult<bool> {
    let output = try!(Command::new("git")
        .arg("diff-files")
        .arg("--quiet")
        .arg("--exit-code")
        .output());

    Ok(output.status.code() == Some(1))
}

/// If the stage area contains uncommited changes.
fn has_staged_changes() -> CrResult<bool> {
    let output = try!(Command::new("git")
        .arg("diff-index")
        .arg("--quiet")
        .arg("--exit-code")
        .arg("--cached HEAD")
        .output());

    Ok(output.status.code() == Some(1))
}

/// Update the local refs to the remote repository.
fn remote_update() -> CrResult<()> {
    let output = try!(Command::new("git")
        .arg("remote")
        .arg("update")
        .output());

    try!(check_output(&output));
    Ok(())
}

type CommitHash = String;

fn local_head() -> CrResult<CommitHash> {
    commit_hash("@")
}

fn remote_head() -> CrResult<CommitHash> {
    commit_hash("@{u}")
}

fn commit_hash(refname: &str) -> CrResult<CommitHash> {
    let output = try!(Command::new("git")
        .arg("rev-parse")
        .arg(refname)
        .output());

    try!(check_output(&output));
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
