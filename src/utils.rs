use std::process::Output;
use cr_result::{CrResult, err_message};

pub fn check_output(out: &Output) -> CrResult<()> {
    if out.status.success() {
        return Ok(());
    }

    err_message(format!("{}\n{}\n",
                        String::from_utf8_lossy(&out.stdout),
                        String::from_utf8_lossy(&out.stderr)))
}
