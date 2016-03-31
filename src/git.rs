use cr_result::{CrResult, CrErr};
use std::process::Command;

/// Checks if git has a clear state, a non dirty working directory
/// and an empty stage area.
pub fn check_clear_state() -> CrResult<()> {
    if try!(has_dirty_working_dir()) {
        return Err(CrErr::Message("Can't operate with dirty git working directory!".to_string()));
    }

    if try!(has_staged_changes()) {
        return Err(CrErr::Message("Can't operate with non empty git staging area!".to_string()));
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
