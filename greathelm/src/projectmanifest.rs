use std::{collections::HashMap, path::Path};

use crate::term::{error, ok};

pub struct ProjectManifest {
    pub properties: HashMap<String, String>,
    pub dependencies: Vec<String>,
    pub directives: Vec<String>,
}

pub fn create_manifest(project_name: String, project_type: String) {
    let contents = match project_type.as_str() {
        _ => c_mf_gen(project_name),
    };

    match std::fs::write("Project.ghm", contents) {
        Ok(_) => {
            ok(format!("Succeeded in creating project manifest."));
        }
        Err(e) => {
            error(format!("Failed to write project manifest. Error is below:"));
            eprintln!("{}", e);
        }
    }
}

pub fn read_manifest(path: &Path) -> ProjectManifest {
    let mut properties: HashMap<String, String> = HashMap::new();
    let mut dependencies: Vec<String> = Vec::new();
    let mut directives: Vec<String> = Vec::new();

    let raw_file = match std::fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) => {
            error(format!("Failed to read Project.ghm. Error is below:"));
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    for l in raw_file.split("\n") {
        if l.starts_with("#") {
            continue;
        }
        if l.starts_with("@Dependency ") {
            dependencies.push(l.split_once("@Dependency ").unwrap().1.into());
            continue;
        }
        if l.starts_with("@Directive ") {
            directives.push(l.split_once("@Directive ").unwrap().1.into());
            continue;
        }

        if !l.contains("=") {
            continue;
        }
        let k = l.split_once("=").unwrap().0;
        let v = l.split_once("=").unwrap().1;

        properties.insert(k.into(), v.into());
    }

    return ProjectManifest {
        properties,
        dependencies,
        directives
    };
}

fn c_mf_gen(project_name: String) -> String {
    format!(
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
        std::env!("CARGO_PKG_VERSION")
    )
}
