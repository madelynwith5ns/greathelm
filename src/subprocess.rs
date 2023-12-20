use std::path::Path;

use crate::term::*;

/**
 * Gets the number of layers deep in subprocess-nesting we are.
 */
pub fn get_embedding_layers() -> usize {
    match std::env::var("GREATHELM_EMBEDDED_LAYERS") {
        Ok(v) => match v.trim().parse() {
            Ok(v) => v,
            Err(_) => 0,
        },
        Err(_) => 0,
    }
}

/**
 * Builds the project located at `path`.
 */
pub fn build_project(path: &Path) {
    info!("Spawning \x1bcgreathelm build\x1br subprocess...");
    let status = duct::cmd!(std::env::current_exe().unwrap(), "build")
        .dir(path)
        .env(
            "GREATHELM_EMBEDDED_LAYERS",
            format!("{}", get_embedding_layers() + 1),
        )
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
