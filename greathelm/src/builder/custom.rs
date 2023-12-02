use std::path::Path;

use crate::{manifest::ProjectManifest, script, term::error};

pub fn build(manifest: ProjectManifest) {
    let build_dir = Path::new("build");
    if !match build_dir.try_exists() {
        Ok(o) => o,
        Err(_) => {
            error(format!("Failed to check if build directory exists. Abort."));
            return;
        }
    } {
        match std::fs::create_dir(build_dir) {
            Ok(_) => {}
            Err(_) => {
                error(format!("Failed to create build directory. Abort."));
                return;
            }
        }
    }

    script::run_script("setup", vec![]);

    let outfile = match manifest.properties.get("Output-Name") {
        Some(v) => {
            format!("build/{}", v.clone())
        }
        None => "build/compiled".into(),
    };

    script::run_script("build", vec![outfile]);
    script::run_script("postbuild", vec![]);
}
