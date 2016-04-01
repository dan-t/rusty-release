use std::process::Command;
use cr_result::{CrResult, err_message};
use utils::check_output;

pub fn build_release() -> CrResult<()> {
    let output = try!(Command::new("cargo")
        .arg("build")
        .arg("--release")
        .output());

    try!(check_output(&output));
    Ok(())
}

pub fn test() -> CrResult<()> {
    let output = try!(Command::new("cargo")
        .arg("test")
        .output());

    try!(check_output(&output));
    Ok(())
}
