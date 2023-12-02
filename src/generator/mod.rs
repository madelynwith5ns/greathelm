use std::path::PathBuf;

use crate::term::{error, info};

pub mod c;
pub mod custom;

pub fn generate(project_type: String, cwd: PathBuf) {
    match project_type.as_str() {
        "C" => {
            info(format!("Using generator \"C\""));
            c::generate(cwd);
        }
        "Custom" => {
            info(format!("Using generator \"Custom\""));
            custom::generate(cwd);
        }

        _ => {
            error(format!("FATAL: Invalid project type passed to generator."));
            std::process::exit(1);
        }
    }
}
