use std::path::PathBuf;

use crate::term::*;

use super::ProjectGenerator;

/**
 * Project generator for Rust projects.
 */
pub struct RustGenerator {}
impl RustGenerator {
    pub fn create() -> Self {
        Self {}
    }
}

impl ProjectGenerator for RustGenerator {
    fn get_name(&self) -> String {
        "Rust".into()
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.greathelm.greathelm".into(),
            identifier: "Rust".into(),
        }
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["rust".into(), "rs".into()]
    }
    fn should_make_ibht_stub(&self) -> bool {
        true
    }
    fn generate(&self, _cwd: PathBuf) {
        super::helper::create_directory("src");
        super::helper::create_directory("lib");
        super::helper::create_directory("export");

        let mut project_name = question("Project name?".into());
        let mut project_namespace = question("Project namespace?".into());
        let mut project_author = question("Project author?".into());
        let mut project_crate_type = question("Crate type (bin, lib, dylib, cdylib, etc.)?".into());

        if project_name == "" {
            project_name = "UnnamedProject".into();
        }
        if project_namespace == "" {
            project_namespace = "com.example".into();
        }
        if project_author == "" {
            project_author = "Example Author".into();
        }
        if project_crate_type == "" {
            project_crate_type = "bin".into();
        }

        if project_crate_type == "bin" {
            let main_rs_contents = "fn main() {\n\
                                    \tprintln!(\"Hello World!\");\n\
        }\n";
            super::helper::create_file("src/main.rs", main_rs_contents);
        } else if project_crate_type.contains("lib") {
            let lib_rs_contents = "pub fn sayhello() {\n\
                                    \tprintln!(\"Hello World!\");\n\
        }\n";
            super::helper::create_file("src/lib.rs", lib_rs_contents);
        }

        // i have no idea why rustfmt is indenting this in such an ugly fashion but it is and we
        // shall deal with it.
        super::helper::create_file(
            "Project.ghm",
            format!(
                "# Greathelm Project Manifest\n\
                Project-Name={project_name}\n\
                Project-Namespace={project_namespace}\n\
                Project-Author={project_author}\n\
                Project-Version=0.1.0-alpha\n\
                Project-Type=Rust\n\
                Executable-Name={project_name}\n\
                Crate-Type={project_crate_type}\n\
                Compiler-Opt-Level=2\n\
                \n\
                Greathelm-Version={}\n",
                env!("CARGO_PKG_VERSION")
            )
            .as_str(),
        );

        ok!("Succeeded in generating project from template.");
    }
}
