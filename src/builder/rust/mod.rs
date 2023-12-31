use std::{io::Write, path::Path};

use crate::{manifest::ProjectManifest, script, term::*};

use super::ProjectBuilder;

/**
 * Built-in builder for Rust projects.
 */
pub struct RustBuilder {}
impl RustBuilder {
    pub fn create() -> Self {
        Self {}
    }
}

impl ProjectBuilder for RustBuilder {
    fn get_name(&self) -> String {
        "Rust".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["rust".into(), "rs".into()]
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.greathelm.greathelm".into(),
            identifier: "Rust".into(),
        }
    }
    fn validate(&self, _manifest: &ProjectManifest) -> bool {
        return true;
    }
    fn cleanup(&self, _manifest: &ProjectManifest) {
        warning!("Rust builder does not currently have a cleanup step.");
    }
    fn build(&self, manifest: &ProjectManifest) {
        let build_dir = Path::new("build");
        if !build_dir.exists() {
            match std::fs::create_dir(build_dir) {
                Ok(_) => {}
                Err(_) => {
                    error!("Failed to create build directory. Abort.");
                    return;
                }
            }
        }

        let crate_type = manifest.get_string_property("Crate-Type", "bin");
        let project_name = manifest.get_string_property("Project-Name", "Unnamed-Project");
        let executable_name = manifest.get_string_property("Executable-Name", &project_name);
        let opt_level = manifest.get_string_property("Compiler-Opt-Level", "2");

        script::run_script("prebuild", vec![]);

        let comp_file = if crate_type == "bin" {
            "src/main.rs"
        } else {
            "src/lib.rs"
        };

        info!(
            "RUSTC \x1bc{}\x1br (\x1bc{}\x1br)",
            project_name, crate_type
        );

        let rustc = duct::cmd!(
            "rustc",
            "--crate-type",
            &crate_type,
            "--emit",
            "link",
            "-o",
            format!("build/{executable_name}"),
            "--edition",
            "2021",
            "-C",
            format!("opt-level={opt_level}"),
            comp_file
        );

        let rustc = match rustc.stderr_to_stdout().run() {
            Ok(v) => v,
            Err(_) => {
                error!("Failed to invoke compiler.");
                std::process::exit(1);
            }
        };

        if rustc.status.success() {
            print!("\x1b[1A");
            std::io::stdout().flush().ok();
            ok!(
                "RUSTC \x1bc{}\x1br (\x1bc{}\x1br)",
                project_name,
                crate_type
            );
        } else {
            print!("\x1b[1A");
            std::io::stdout().flush().ok();
            error!(
                "RUSTC \x1bc{}\x1br (\x1bc{}\x1br)",
                project_name, crate_type
            );
        }

        script::run_script("postbuild", vec![]);
    }
}
