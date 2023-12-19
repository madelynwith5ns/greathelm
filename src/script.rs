use std::path::Path;

use crate::{
    config,
    term::{error, info},
};

/**
 * This method only checks if the script exists in the
 * current project!!!
 * It does NOT check user scripts!
 */
pub fn has_script(script_name: &str) -> bool {
    let mut path: String = "scripts/".into();
    path.push_str(script_name);
    path.push_str(".sh");
    match Path::new(&path).try_exists() {
        Ok(o) => o,
        Err(_) => {
            error(format!(
                "Cannot check for script {path}. Assuming it is not present."
            ));
            false
        }
    }
}

/**
 * Runs all instances of `script_name` found.
 */
pub fn run_script(script_name: &str, args: Vec<String>) {
    let str = format!(
        "{}/scripts/{script_name}.sh",
        config::get_config_base_dir().to_str().unwrap()
    );
    let userpath = Path::new(&str);
    let mut argstr = String::new();
    for arg in &args {
        argstr.push_str(&format!(" {}", arg));
    }
    let str = format!("{str}{}", argstr);

    if userpath.exists() {
        duct::cmd!("sh", "-c", str).stderr_to_stdout().run().ok();
    }

    if has_script(script_name) {
        info(format!("Running script {script_name}"));
        let mut invoke: String = "./scripts/".into();
        invoke.push_str(script_name);
        invoke.push_str(".sh");
        for arg in &args {
            invoke.push_str(" ");
            invoke.push_str(&arg);
        }

        duct::cmd!("sh", "-c", invoke).stderr_to_stdout().run().ok();
    }
}
