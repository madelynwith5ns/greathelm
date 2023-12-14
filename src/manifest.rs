#![allow(dead_code)] // don't complain about convenience methods
                     // (get_<type>_property) that are unused
                     // they are for plugins/later.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{module::Module, term::error};

#[derive(Clone)]
pub struct ProjectManifest {
    pub properties: HashMap<String, String>,
    pub dependencies: Vec<String>,
    pub directives: Vec<String>,
    pub modules: Vec<Module>,
    pub aliases: HashMap<String, String>,
}

impl ProjectManifest {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
            dependencies: Vec::new(),
            directives: Vec::new(),
            modules: Vec::new(),
            aliases: HashMap::new(),
        }
    }

    pub fn get_string_property(&self, key: &str, default: &str) -> String {
        match self.properties.get(key) {
            Some(k) => k.clone(),
            None => default.into(),
        }
    }

    pub fn get_i32_property(&self, key: &str, default: i32) -> i32 {
        match self.get_string_property(key, &format!("{default}")).parse() {
            Ok(v) => v,
            Err(_) => default,
        }
    }

    pub fn get_u32_property(&self, key: &str, default: u32) -> u32 {
        match self.get_string_property(key, &format!("{default}")).parse() {
            Ok(v) => v,
            Err(_) => default,
        }
    }

    pub fn get_i64_property(&self, key: &str, default: i64) -> i64 {
        match self.get_string_property(key, &format!("{default}")).parse() {
            Ok(v) => v,
            Err(_) => default,
        }
    }

    pub fn get_u64_property(&self, key: &str, default: u64) -> u64 {
        match self.get_string_property(key, &format!("{default}")).parse() {
            Ok(v) => v,
            Err(_) => default,
        }
    }

    pub fn get_usize_property(&self, key: &str, default: usize) -> usize {
        match self.get_string_property(key, &format!("{default}")).parse() {
            Ok(v) => v,
            Err(_) => default,
        }
    }

    pub fn get_bool_property(&self, key: &str, default: bool) -> bool {
        match self.get_string_property(key, &format!("{default}")).parse() {
            Ok(v) => v,
            Err(_) => default,
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
                self.dependencies
                    .push(l.split_once("@Dependency ").unwrap().1.into());
                continue;
            }
            if l.starts_with("@Directive ") {
                self.directives
                    .push(l.split_once("@Directive ").unwrap().1.into());
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
                let path =
                    PathBuf::from_str(format!("{}", l.split_once("@Import ").unwrap().1).as_str())
                        .unwrap();
                if path.exists() {
                    self.read_and_append(&path);
                }
            }
            if l.starts_with("@Alias ") {
                let (alias, target) = l.split_once("@Alias ").unwrap().1.split_once("=").unwrap();
                self.aliases.insert(alias.into(), target.into());
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
