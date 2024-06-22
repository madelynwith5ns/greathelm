use std::path::PathBuf;

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
    fn generate(&self, _cwd: PathBuf) {
        super::helper::create_directory("src");
        super::helper::create_directory("lib");
        super::helper::create_directory("export");
        super::helper::create_directory("lib/include");
        super::helper::create_directory("lib/shared");
        super::helper::create_directory("lib/obj");

        let main_c_contents = "#include <stdio.h>\n\
                               \n\
                               int main(int argc, char **argv) {\n\
                                   \tprintf(\"Hello World!\\n\");\n\
                               }\n";
        super::helper::create_file("src/main.c", main_c_contents);

        let mut project_name = question("Project name?".into());
        let mut project_namespace = question("Project namespace?".into());
        let mut project_author = question("Project author?".into());
        let mut project_emit = question("Emit type (binary / dylib)?".into());

        if project_name == "" {
            project_name = "UnnamedProject".into();
        }
        if project_namespace == "" {
            project_namespace = "com.example".into();
        }
        if project_author == "" {
            project_author = "Example Author".into();
        }
        if project_emit != "binary" && project_emit != "dylib" {
            project_emit = "binary".into();
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
                                       Project-Type=C\n\
                                       Compiler-Opt-Level=2\n\
                                       Executable-Name={project_name}\n\
                                       Emit={project_emit}\n\
                                       \n\
                                       Greathelm-Version={}\n",
                env!("CARGO_PKG_VERSION")
            )
            .as_str(),
        );

        ok!("Succeeded in generating project from template.");
    }
}
