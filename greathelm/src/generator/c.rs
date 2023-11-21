use std::path::{Path, PathBuf};

use crate::term::{error, ok};

pub fn generate(_cwd: PathBuf) {
    match std::fs::create_dir("src") {
        Ok(_) => {}
        Err(e) => {
            error(format!("Failed to create project! Error is below:"));
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    match std::fs::create_dir("lib") {
        Ok(_) => {}
        Err(e) => {
            error(format!("Failed to create project! Error is below:"));
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
    match std::fs::create_dir("lib/include") {
        Ok(_) => {}
        Err(e) => {
            error(format!("Failed to create project! Error is below:"));
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
    match std::fs::create_dir("lib/shared") {
        Ok(_) => {}
        Err(e) => {
            error(format!("Failed to create project! Error is below:"));
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
    match std::fs::create_dir("lib/obj") {
        Ok(_) => {}
        Err(e) => {
            error(format!("Failed to create project! Error is below:"));
            eprintln!("{}", e);
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
            error(format!("Failed to create project! Error is below:"));
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    match std::fs::write(
        Path::new("Project.ghm"),
        "# Greathelm Project Manifest\n\
                         Project-Name={project_name}\n\
                         Project-Author=Example Author\n\
                         Project-Version=0.1.0-alpha\n\
                         Project-Type=C\n\
                         Compiler-Opt-Level=2\n\
                         Executable-Name={project_name}\n\
                         Emit=binary\n\
                         \n\
                         Greathelm-Version={}\n",
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
