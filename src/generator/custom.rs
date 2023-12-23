use std::{
    fs::Permissions,
    os::unix::prelude::PermissionsExt,
    path::{Path, PathBuf},
};

use crate::term::*;

use super::ProjectGenerator;

/**
 * Project generator for Custom projects.
 */
pub struct CustomGenerator {}
impl CustomGenerator {
    pub fn create() -> Self {
        Self {}
    }
}

impl ProjectGenerator for CustomGenerator {
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
    fn should_make_ibht_stub(&self) -> bool {
        false
    }
    fn generate(&self, cwd: PathBuf) {
        super::helper::create_directory("src");
        super::helper::create_directory("scripts");
        super::helper::create_file("scripts/prebuild.sh", "#!/usr/bin/bash\necho !! prebuild.sh has not been written yet !!\n");
        super::helper::create_file("scripts/build.sh", "#!/usr/bin/bash\necho !! build.sh has not been written yet !!\n");
        super::helper::create_file("scripts/postbuild.sh", "#!/usr/bin/bash\necho !! postbuild.sh has not been written yet !!\n");

        // set permissions on UNIX systems.
        #[cfg(target_family = "unix")]
        {
            std::fs::set_permissions(
                Path::new("scripts/prebuild.sh"),
                Permissions::from_mode(0o777),
                )
                .ok();
            std::fs::set_permissions(Path::new("scripts/build.sh"), Permissions::from_mode(0o777)).ok();
            std::fs::set_permissions(
                Path::new("scripts/postbuild.sh"),
                Permissions::from_mode(0o777),
                )
                .ok();
        }

        let project_name = match cwd.file_name() {
            Some(s) => s.to_string_lossy().to_string(),
            None => "example".into(),
        };
        super::helper::create_file("Project.ghm", format!(
                "# Greathelm Project Manifest\n\
                Project-Name={project_name}\n\
                Project-Namespace=com.example\n\
                Project-Author=Example Author\n\
                Project-Version=0.1.0-alpha\n\
                Project-Type=Custom\n\
                Output-Name={project_name}\n\
                \n\
                Greathelm-Version={}\n",
                env!("CARGO_PKG_VERSION")
                ).as_str());

        ok!("Succeeded in generating project from template.");
    }
}
