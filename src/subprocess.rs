use std::path::Path;

use crate::term::{error, info, ok};

pub fn build_project(path: &Path) {
    info(format!("Spawning `greathelm build` subprocess..."));
    let status = duct::cmd!(std::env::current_exe().unwrap(), "build")
        .dir(path)
        .stderr_to_stdout()
        .run()
        .unwrap()
        .status;
    if status.success() {
        ok(format!("Build subprocess succeeded."));
    } else {
        error(format!("Build subprocess failed."));
        std::process::exit(1);
    }
}
