use std::path::{Path, PathBuf};

use crate::term::*;

use super::ProjectGenerator;

/**
 * Project generator for C projects.
 */
pub struct CGenerator {}
impl CGenerator {
    pub fn create() -> Self {
        Self {}
    }
}

impl ProjectGenerator for CGenerator {
    fn get_name(&self) -> String {
        "C".into()
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "C".into(),
        }
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["c".into()]
    }
    fn should_make_ibht_stub(&self) -> bool {
        true
    }
    fn generate(&self, cwd: PathBuf) {
        match std::fs::create_dir("src") {
            Ok(_) => {}
            Err(e) => {
                print_error_obj(
                    Some("Failed to create project! Error is below:".into()),
                    Box::new(e),
                );
                std::process::exit(1);
            }
        };

        match std::fs::create_dir("lib") {
            Ok(_) => {}
            Err(e) => {
                print_error_obj(
                    Some("Failed to create project! Error is below:".into()),
                    Box::new(e),
                );
                std::process::exit(1);
            }
        }

        match std::fs::create_dir("export") {
            Ok(_) => {}
            Err(e) => {
                print_error_obj(
                    Some("Failed to create project! Error is below:".into()),
                    Box::new(e),
                );
                std::process::exit(1);
            }
        }

        match std::fs::create_dir("lib/include") {
            Ok(_) => {}
            Err(e) => {
                print_error_obj(
                    Some("Failed to create project! Error is below:".into()),
                    Box::new(e),
                );
                std::process::exit(1);
            }
        }
        match std::fs::create_dir("lib/shared") {
            Ok(_) => {}
            Err(e) => {
                print_error_obj(
                    Some("Failed to create project! Error is below:".into()),
                    Box::new(e),
                );
                std::process::exit(1);
            }
        }
        match std::fs::create_dir("lib/obj") {
            Ok(_) => {}
            Err(e) => {
                print_error_obj(
                    Some("Failed to create project! Error is below:".into()),
                    Box::new(e),
                );
                std::process::exit(1);
            }
        }

        let main_c_contents = "#include <stdio.h>\n\
                               \n\
                               int main(int argc, char **argv) {\n\
                                   \tprintf(\"Hello World!\\n\");\n\
                               }\n";

        match std::fs::write(Path::new("src/main.c"), main_c_contents) {
            Ok(_) => {}
            Err(e) => {
                print_error_obj(
                    Some("Failed to create project! Error is below:".into()),
                    Box::new(e),
                );
                std::process::exit(1);
            }
        };

        let project_name = match cwd.file_name() {
            Some(s) => s.to_string_lossy().to_string(),
            None => "example".into(),
        };

        match std::fs::write(
            Path::new("Project.ghm"),
            format!(
                "# Greathelm Project Manifest\n\
                Project-Name={project_name}\n\
                Project-Namespace=com.example\n\
                Project-Author=Example Author\n\
                Project-Version=0.1.0-alpha\n\
                Project-Type=C\n\
                Compiler-Opt-Level=2\n\
                Executable-Name={project_name}\n\
                Emit=binary\n\
                \n\
                Greathelm-Version={}\n",
                env!("CARGO_PKG_VERSION")
            ),
        ) {
            Ok(_) => {}
            Err(e) => {
                print_error_obj(
                    Some("Failed to create project! Error is below:".into()),
                    Box::new(e),
                );
                std::process::exit(1);
            }
        };
        ok!("Succeeded in generating project from template.");
    }
}
