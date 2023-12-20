use std::path::Path;

use crate::term::*;

/**
 * Builds the project located at `path`.
 */
pub fn build_project(path: &Path) {
    info!("Spawning \x1bcgreathelm build\x1br subprocess...");
    let status = duct::cmd!(std::env::current_exe().unwrap(), "build")
        .dir(path)
        .stderr_to_stdout()
        .run()
        .unwrap()
        .status;
    if status.success() {
        ok!("Build subprocess succeeded.");
    } else {
        error!("Build subprocess failed.");
        std::process::exit(1);
    }
}
