use std::path::{PathBuf, Path};

use crate::term::{info, error, ok};

pub fn generate(project_type: String, cwd: PathBuf) {
    match project_type.as_str() {
        "C" => {
            info(format!("Using generator \"C\""));
            c_gen(cwd);
        },

        _=>{
            error(format!("FATAL: Invalid project type passed to generator."));
            std::process::exit(1);
        }
    }
}


fn c_gen(_cwd: PathBuf) {
    match std::fs::create_dir("src") {
        Ok(_) => {},
        Err(e) => {
            error(format!("Failed to create project! Error is below:"));
            eprintln!("{}",e);
            std::process::exit(1);
        },
    }; 

    let main_c_contents = "#include <stdio.h>\n\
                           \n\
                           int main(int argc, char **argv) {\n\
                                \tprintf(\"Hello World!\\n\");\n\
                           }\n";

    match std::fs::write(Path::new("src/main.c"),main_c_contents) {
        Ok(_) => {},
        Err(e) => {
            error(format!("Failed to create project! Error is below:"));
            eprintln!("{}",e);
            std::process::exit(1);
        }
    };

    ok(format!("Succeeded in generating project from template."));
}
