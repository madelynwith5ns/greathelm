use std::path::Path;

use crate::{manifest::ProjectManifest, term::error};

pub mod c;

pub fn build(manifest: ProjectManifest) {
    let project_type = manifest.properties.get("Project-Type").unwrap();

    let build_dir = Path::new("build");
    if !build_dir.exists() {
        match std::fs::create_dir(build_dir) {
            Ok(_) => {}
            Err(e) => {
                error(format!(
                    "Failed to create project build directory! Error is below:"
                ));
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }

    match project_type.as_str() {
        "C" => {
            c::build(manifest);
        }
        _ => {
            error(format!(
                "An invalid project type was passed to the builder."
            ));
            return;
        }
    }
}
