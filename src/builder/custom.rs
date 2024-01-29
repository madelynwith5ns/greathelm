use std::path::Path;

use crate::{manifest::ProjectManifest, script, term::*};

use super::ProjectBuilder;

/**
 * Built-in builder for Custom projects. These projects use scripts to either use a completely
 * custom build process or wrap another build system's project (which might be done in the case of
 * using Greathelm to build a system iso image).
 */
pub struct CustomBuilder {}
impl CustomBuilder {
    pub fn create() -> Self {
        Self {}
    }
}

impl ProjectBuilder for CustomBuilder {
    fn get_name(&self) -> String {
        "Custom".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["custom".into()]
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "Custom".into(),
        }
    }
    fn validate(&self, _manifest: &ProjectManifest) -> bool {
        return true;
    }
    fn cleanup(&self, _manifest: &ProjectManifest) {
        script::run_script("cleanup", vec![]);
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

        script::run_script("prebuild", vec![]);

        let outfile = manifest.get_string_property("Output-Name", "build/compiled".into());
        script::run_script("build", vec![outfile]);

        script::run_script("postbuild", vec![]);
    }
}
