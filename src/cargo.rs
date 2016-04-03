use std::process::Command;
use rr_result::RrResult;
use utils::check_output;

pub fn build_release() -> RrResult<()> {
    let output = try!(Command::new("cargo")
        .arg("build")
        .arg("--release")
        .output());

    try!(check_output(&output));
    Ok(())
}

pub fn test() -> RrResult<()> {
    let output = try!(Command::new("cargo")
        .arg("test")
        .output());

    try!(check_output(&output));
    Ok(())
}

pub fn publish() -> RrResult<()> {
    let output = try!(Command::new("cargo")
        .arg("publish")
        .output());

    try!(check_output(&output));
    Ok(())
}
