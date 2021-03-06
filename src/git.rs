use std::io::Write;
use std::process::Command;
use tempfile::{NamedTempFile, Builder};
use rr_result::RrResult;
use utils::check_output;

/// Checks if git has a clean state, a non dirty working directory,
/// an empty stage area and that the remote repository isn't ahead of
/// the local one.
pub fn check_state() -> RrResult<()> {
    if has_dirty_working_dir()? {
        return Err("Can't operate with dirty git working directory! Clear or commit changes!".into());
    }

    if has_staged_changes()? {
        return Err("Can't operate with non empty git staging area! Clear or commit staged changes!".into());
    }

    let local_head = local_head()?;

    remote_update()?;
    let remote_head = remote_head()?;

    let merge_base = merge_base(&local_head, &remote_head)?;
    if remote_head != merge_base {
        return Err("Can't operate with diverging local and remote git repository! Synchronize them!".into())
    }

    Ok(())
}

pub fn add_update() -> RrResult<()> {
    let output = Command::new("git")
        .arg("add")
        .arg("--update")
        .output()?;

    check_output(&output)?;
    Ok(())
}

pub fn commit(msg: &str) -> RrResult<()> {
    let output = Command::new("git")
        .arg("commit")
        .arg(format!("--message={}", msg))
        .output()?;

    check_output(&output)?;
    Ok(())
}

pub fn tag(name: &str) -> RrResult<()> {
    let output = Command::new("git")
        .arg("tag")
        .arg(name)
        .output()?;

    check_output(&output)?;
    Ok(())
}

pub fn push() -> RrResult<()> {
    let output = Command::new("git")
        .arg("push")
        .output()?;

    check_output(&output)?;

    let output = Command::new("git")
        .arg("push")
        .arg("--tags")
        .output()?;

    check_output(&output)?;
    Ok(())
}

pub fn log_file(from: &str, to: Option<&str>) -> RrResult<NamedTempFile> {
    let output = log(from, to)?;

    let prefix = if let Some(to) = to {
        format!("{}...{}___", from, to)
    } else {
        from.to_owned()
    };

    let mut log_file = Builder::new()
        .prefix(&prefix)
        .tempfile()?;

    log_file.write_all(output.as_bytes())?;
    Ok(log_file)
}

pub fn log(from: &str, to: Option<&str>) -> RrResult<String> {
    let output = Command::new("git")
        .arg("--no-pager")
        .arg("log")
        .arg("--decorate=short")
        .arg("--pretty=oneline")
        .arg("--abbrev-commit")
        .arg(if let Some(to) = to { format!("{}...{}", from, to) } else { from.to_owned() })
        .output()?;

    check_output(&output)?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn has_tag(name: &str) -> RrResult<bool> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--verify")
        .arg("--quiet")
        .arg(name)
        .output()?;

    Ok(output.status.success())
}

/// If the working directory has uncommited changes.
pub fn has_dirty_working_dir() -> RrResult<bool> {
    let output = Command::new("git")
        .arg("diff-files")
        .arg("--quiet")
        .arg("--exit-code")
        .output()?;

    Ok(output.status.code() == Some(1))
}

/// If the stage area contains uncommited changes.
fn has_staged_changes() -> RrResult<bool> {
    let output = Command::new("git")
        .arg("diff-index")
        .arg("--quiet")
        .arg("--exit-code")
        .arg("--cached HEAD")
        .output()?;

    Ok(output.status.code() == Some(1))
}

/// Update the local refs to the remote repository.
fn remote_update() -> RrResult<()> {
    let output = Command::new("git")
        .arg("remote")
        .arg("update")
        .output()?;

    check_output(&output)?;
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
    let output = Command::new("git")
        .arg("rev-parse")
        .arg(refname)
        .output()?;

    check_output(&output)?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn merge_base(refname1: &str, refname2: &str) -> RrResult<CommitHash> {
    let output = Command::new("git")
        .arg("merge-base")
        .arg(refname1)
        .arg(refname2)
        .output()?;

    check_output(&output)?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

