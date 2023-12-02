use std::{io::Write, path::Path, process::Command};

use crate::term::{error, info};

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

pub fn run_script(script_name: &str, args: Vec<String>) {
    if has_script(script_name) {
        info(format!("Running script {script_name}"));
        let mut invoke: String = "./scripts/".into();
        invoke.push_str(script_name);
        invoke.push_str(".sh");
        for arg in args {
            invoke.push_str(" ");
            invoke.push_str(&arg);
        }
        match Command::new("sh").arg("-c").arg(invoke).output() {
            Ok(output) => {
                let out = String::from_utf8(output.stdout);
                match out {
                    Ok(ok) => {
                        print!("{}", ok);
                        std::io::stdout().flush().ok();
                    }
                    Err(_) => {}
                }
                let out = String::from_utf8(output.stderr);
                match out {
                    Ok(ok) => {
                        eprint!("{}", ok);
                        std::io::stderr().flush().ok();
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        };
    }
}
