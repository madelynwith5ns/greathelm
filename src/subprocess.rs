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
    spawn_with_args(path, vec!["build".into()]);
}

/**
 * Spawns a subprocess with the specified arguments.
 */
pub fn spawn_with_args(cwd: &Path, args: Vec<String>) {
    info!(
        "Spawning \x1bcgreathelm {}\x1br subprocess.",
        match args.get(0) {
            Some(v) => {
                v
            }
            None => {
                error!("No action provided to subprocess::spawn_with_args. Abort.");
                return;
            }
        }
    );
    let status = duct::cmd(std::env::current_exe().unwrap(), args)
        .dir(cwd)
        .env(
            "GREATHELM_EMBEDDED_LAYERS",
            format!("{}", get_embedding_layers() + 1),
        )
        .stderr_to_stdout()
        .run()
        .unwrap()
        .status;
    if status.success() {
        ok!("Subprocess succeeded.");
    } else {
        error!("Subprocess failed.");
        std::process::exit(1);
    }
}
