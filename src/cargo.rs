use std::process::Command;
use cr_result::{CrResult, err_message};

pub fn build_release() -> CrResult<()> {
    let output = try!(Command::new("cargo")
        .arg("build")
        .arg("--release")
        .output());

    if ! output.status.success() {
        return err_message(format!("stdout: {}, stderr: {}\n",
                                   String::from_utf8_lossy(&output.stdout),
                                   String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}

pub fn test() -> CrResult<()> {
    let output = try!(Command::new("cargo")
        .arg("test")
        .output());

    if ! output.status.success() {
        return err_message(format!("stdout: {}, stderr: {}\n",
                                   String::from_utf8_lossy(&output.stdout),
                                   String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}
