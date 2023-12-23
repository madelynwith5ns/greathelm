use std::path::PathBuf;

use crate::term::*;

use super::ProjectGenerator;

/**
 * Project generator for C++ projects.
 */
pub struct CPPGenerator {}
impl CPPGenerator {
    pub fn create() -> Self {
        Self {}
    }
}

impl ProjectGenerator for CPPGenerator {
    fn get_name(&self) -> String {
        "C++".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["c++".into(), "cpp".into()]
    }
    fn should_make_ibht_stub(&self) -> bool {
        true
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "C++".into(),
        }
    }
    fn generate(&self, cwd: PathBuf) {
        super::helper::create_directory("src");
        super::helper::create_directory("lib");
        super::helper::create_directory("export");
        super::helper::create_directory("lib/include");
        super::helper::create_directory("lib/shared");
        super::helper::create_directory("lib/obj");

        let main_cpp_contents = "#include <iostream>\n\
                                 \n\
                                 int main(int argc, char **argv) {\n\
                                     \tstd::cout << \"Hello World!\" << std::endl;\n\
                                 }\n";
        super::helper::create_file("src/main.cpp", main_cpp_contents);

        let project_name = match cwd.file_name() {
            Some(s) => s.to_string_lossy().to_string(),
            None => "example".into(),
        };
        super::helper::create_file(
            "Project.ghm",
            format!(
                "# Greathelm Project Manifest\n\
                                       Project-Name={project_name}\n\
                                       Project-Namespace=com.example\n\
                                       Project-Author=Example Author\n\
                                       Project-Version=0.1.0-alpha\n\
                                       Project-Type=C++\n\
                                       C++-Stdlib-Flavor=stdc++\n\
                                       Compiler-Opt-Level=2\n\
                                       Executable-Name={project_name}\n\
                                       Emit=binary\n\
                                       \n\
                                       Greathelm-Version={}\n",
                env!("CARGO_PKG_VERSION")
            )
            .as_str(),
        );

        ok!("Succeeded in generating project from template.");
    }
}
