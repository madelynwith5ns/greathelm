use std::path::Path;

use crate::{manifest::ProjectManifest, script, term::error};

use super::ProjectBuilder;

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

        script::run_script("prebuild", vec![]);

        let outfile = match manifest.properties.get("Output-Name") {
            Some(v) => {
                format!("build/{}", v.clone())
            }
            None => "build/compiled".into(),
        };

        script::run_script("build", vec![outfile]);
        script::run_script("postbuild", vec![]);
    }
}
