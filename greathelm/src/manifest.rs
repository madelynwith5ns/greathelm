use std::{collections::HashMap, path::Path};

use crate::{module::Module, term::error};

pub struct ProjectManifest {
    pub properties: HashMap<String, String>,
    pub dependencies: Vec<String>,
    pub directives: Vec<String>,
    pub modules: Vec<Module>,
}

pub fn read_manifest(path: &Path) -> ProjectManifest {
    let mut properties: HashMap<String, String> = HashMap::new();
    let mut dependencies: Vec<String> = Vec::new();
    let mut directives: Vec<String> = Vec::new();
    let mut modules: Vec<Module> = Vec::new();

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
        if l.starts_with("@Module ") {
            let module_name = l.split(" ").nth(1).unwrap();
            let mut files: HashMap<String, String> = HashMap::new();
            for path in l.split(" ").skip(2) {
                if !path.contains(":") {
                    continue;
                }
                let (homepath, modpath) = path.split_once(":").unwrap();
                files.insert(homepath.into(), modpath.into());
            }
            modules.push(Module {
                module_name: module_name.into(),
                files,
            });
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
        directives,
        modules,
    };
}
