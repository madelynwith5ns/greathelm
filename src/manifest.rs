use std::{collections::HashMap, path::{Path, PathBuf}, str::FromStr};

use crate::{module::Module, term::error};

pub struct ProjectManifest {
    pub properties: HashMap<String, String>,
    pub dependencies: Vec<String>,
    pub directives: Vec<String>,
    pub modules: Vec<Module>,
}

impl ProjectManifest {
    pub fn new() -> Self {
        Self { 
            properties: HashMap::new(),
            dependencies: Vec::new(),
            directives: Vec::new(),
            modules: Vec::new()
        }
    }

    pub fn read_and_append(&mut self, path: &Path) { 
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
                self.dependencies.push(l.split_once("@Dependency ").unwrap().1.into());
                continue;
            }
            if l.starts_with("@Directive ") {
                self.directives.push(l.split_once("@Directive ").unwrap().1.into());
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
                self.modules.push(Module {
                    module_name: module_name.into(),
                    files,
                });
            }
            if l.starts_with("@Import ") {
                let path = PathBuf::from_str(format!("{}", l.split_once("@Import ").unwrap().1).as_str()).unwrap();
                if path.exists() {
                    self.read_and_append(&path);
                }
            }

            if !l.contains("=") {
                continue;
            }
            let k = l.split_once("=").unwrap().0;
            let v = l.split_once("=").unwrap().1;

            self.properties.insert(k.into(), v.into());
        }
    }
}
