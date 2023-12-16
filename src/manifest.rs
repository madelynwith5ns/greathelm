#![allow(dead_code)] // don't complain about convenience methods
                     // (get_<type>_property) that are unused
                     // they are for plugins/later.

use std::{collections::HashMap, path::Path};

use crate::{module::Module, term::error};

#[derive(Clone)]
pub struct ProjectManifest {
    pub properties: HashMap<String, String>,
    pub directives: HashMap<String, Vec<String>>,
}

impl ProjectManifest {
    pub fn new() -> Self {
        let mut s = Self {
            properties: HashMap::new(),
            directives: HashMap::new(),
        };

        s.directives.insert("Dependency".into(), Vec::new());
        s.directives.insert("Alias".into(), Vec::new());
        s.directives.insert("Module".into(), Vec::new());
        s.directives.insert("Directive".into(), Vec::new());

        return s;
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
            if l.starts_with("@") && l.contains(" ") {
                let directive = l.split_once("@").unwrap().1.split_once(" ").unwrap().0;
                let mut directivecontent = String::new();
                for c in l.split(" ").skip(1) {
                    if !directivecontent.is_empty() {
                        directivecontent.push_str(" ");
                    }
                    directivecontent.push_str(c);
                }
                if !self.directives.contains_key(directive) {
                    self.directives.insert(directive.into(), Vec::new());
                }
                self.directives
                    .get_mut(directive)
                    .unwrap()
                    .push(directivecontent);
            }

            if !l.contains("=") {
                continue;
            }
            let k = l.split_once("=").unwrap().0;
            let v = l.split_once("=").unwrap().1;

            self.properties.insert(k.into(), v.into());
        }
    }

    pub fn get_modules(&self) -> Vec<Module> {
        let mut modules = Vec::new();

        for m in self.directives.get("Module").unwrap() {
            let module_name = m.split_once(" ").unwrap().0;
            let mut files: HashMap<String, String> = HashMap::new();
            for path in m.split(" ").skip(2) {
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

        return modules;
    }

    pub fn get_aliases_map(&self) -> HashMap<String, String> {
        let mut m = HashMap::new();

        for a in self.directives.get("Alias").unwrap() {
            if !a.contains(" ") {
                continue;
            }
            let (k, v) = a.split_once(" ").unwrap();
            m.insert(k.into(), v.into());
        }

        return m;
    }
}
