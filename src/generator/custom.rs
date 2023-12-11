use std::{
    fs::Permissions,
    os::unix::prelude::PermissionsExt,
    path::{Path, PathBuf},
};

use crate::term::{error, ok};

use super::ProjectGenerator;

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
        match std::fs::create_dir("src") {
            Ok(_) => {}
            Err(e) => {
                error(format!("Failed to create project! Error is below:"));
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }

        match std::fs::create_dir("scripts") {
            Ok(_) => {}
            Err(e) => {
                error(format!("Failed to create project! Error is below:"));
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }

        match std::fs::write(
            Path::new("scripts/prebuild.sh"),
            format!(
                "#!/usr/bin/bash\n\
                echo !! prebuild.sh has not been written yet !!\n"
            ),
        ) {
            Ok(_) => {}
            Err(e) => {
                error(format!("Failed to create project! Error is below:"));
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
        std::fs::set_permissions(
            Path::new("scripts/prebuild.sh"),
            Permissions::from_mode(0o777),
        )
        .ok();

        match std::fs::write(
            Path::new("scripts/build.sh"),
            format!(
                "#!/usr/bin/bash\n\
                echo !! build.sh has not been written yet !!\n"
            ),
        ) {
            Ok(_) => {}
            Err(e) => {
                error(format!("Failed to create project! Error is below:"));
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
        std::fs::set_permissions(Path::new("scripts/build.sh"), Permissions::from_mode(0o777)).ok();

        match std::fs::write(
            Path::new("scripts/postbuild.sh"),
            format!(
                "#!/usr/bin/bash\n\
                echo !! postbuild.sh has not been written yet !!\n"
            ),
        ) {
            Ok(_) => {}
            Err(e) => {
                error(format!("Failed to create project! Error is below:"));
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };
        std::fs::set_permissions(
            Path::new("scripts/postbuild.sh"),
            Permissions::from_mode(0o777),
        )
        .ok();

        let project_name = match cwd.file_name() {
            Some(s) => s.to_string_lossy().to_string(),
            None => "example".into(),
        };
        match std::fs::write(
            Path::new("Project.ghm"),
            format!(
                "# Greathelm Project Manifest\n\
                Project-Name={project_name}\n\
                Project-Author=Example Author\n\
                Project-Version=0.1.0-alpha\n\
                Project-Type=Custom\n\
                Output-Name={project_name}\n\
                \n\
                Greathelm-Version={}\n",
                env!("CARGO_PKG_VERSION")
            ),
        ) {
            Ok(_) => {}
            Err(e) => {
                error(format!("Failed to create project! Error is below:"));
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };

        ok(format!("Succeeded in generating project from template."));
    }
}
