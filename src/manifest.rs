#![allow(dead_code)] // don't complain about convenience methods
                     // (get_<type>_property) that are unused
                     // they are for plugins/later.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{module::Module, term::*};

/**
 * Struct for Project Manifests. This is usually a combined manifest of
 * (CONFIGROOT)/UserManifest.ghm (PROJECTROOT)/Project.ghm and (PROJECTROOT)/Project.local.ghm and
 * any additional manifests they may @Import.
 */
#[derive(Clone)]
pub struct ProjectManifest {
    pub properties: HashMap<String, String>,
    pub directives: HashMap<String, Vec<String>>,
}

impl ProjectManifest {
    /**
     * Creates an empty ProjectManifest
     */
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

    /**
     * Gets the property `key` as a String. Defaulting to `default` if not present.
     */
    pub fn get_string_property(&self, key: &str, default: &str) -> String {
        match self.properties.get(key) {
            Some(k) => k.clone(),
            None => default.into(),
        }
    }

    /**
     * Gets the property `key` as an i32. Defaulting to `default` if not present or not an i32.
     */
    pub fn get_i32_property(&self, key: &str, default: i32) -> i32 {
        match self.get_string_property(key, &format!("{default}")).parse() {
            Ok(v) => v,
            Err(_) => default,
        }
    }

    /**
     * Gets the property `key` as a u32. Defaulting to `default` if not present or not a u32.
     */
    pub fn get_u32_property(&self, key: &str, default: u32) -> u32 {
        match self.get_string_property(key, &format!("{default}")).parse() {
            Ok(v) => v,
            Err(_) => default,
        }
    }

    /**
     * Gets the property `key` as an i64. Defaulting to `default` if not present or not an i64.
     */
    pub fn get_i64_property(&self, key: &str, default: i64) -> i64 {
        match self.get_string_property(key, &format!("{default}")).parse() {
            Ok(v) => v,
            Err(_) => default,
        }
    }

    /**
     * Gets the property `key` as a u64. Defaulting to `default` if not present or not a u64.
     */
    pub fn get_u64_property(&self, key: &str, default: u64) -> u64 {
        match self.get_string_property(key, &format!("{default}")).parse() {
            Ok(v) => v,
            Err(_) => default,
        }
    }

    /**
     * Gets the property `key` as a usize. Defaulting to `default` if not present or not a usize.
     */
    pub fn get_usize_property(&self, key: &str, default: usize) -> usize {
        match self.get_string_property(key, &format!("{default}")).parse() {
            Ok(v) => v,
            Err(_) => default,
        }
    }

    /**
     * Gets the property `key` as a bool. Defaulting to `default` if not present or not a bool.
     */
    pub fn get_bool_property(&self, key: &str, default: bool) -> bool {
        match self.get_string_property(key, &format!("{default}")).parse() {
            Ok(v) => v,
            Err(_) => default,
        }
    }

    /**
     * Reads the manifest at `path` and appends its contents to this manifest.
     */
    pub fn read_and_append(&mut self, path: &Path) {
        let raw_file = match std::fs::read_to_string(path) {
            Ok(data) => data,
            Err(e) => {
                print_error_obj(
                    Some("Failed to read \x1bcProject.ghm\x1br.".into()),
                    Box::new(e),
                );
                std::process::exit(1);
            }
        };

        for l in raw_file.split("\n") {
            if l.starts_with("#") {
                continue;
            }
            if l.starts_with("@Import ") {
                let path =
                    PathBuf::from_str(format!("{}", l.split_once("@Import ").unwrap().1).as_str())
                        .unwrap();
                if path.exists() {
                    self.read_and_append(&path);
                }
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

    /**
     * Gets the @Module directives as Module structs.
     */
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

    /**
     * Gets a map of all the aliases defined with @Alias directives.
     */
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

    /**
     * Appends properties to this manifest instance from CLI arguments.
     */
    pub fn append_from_cli_args(&mut self, args: Vec<String>) {
        for arg in &args {
            if arg.starts_with("--") {
                if arg.contains("=") {
                    let (k, v) = arg[2..].split_once("=").unwrap();
                    self.properties.insert(k.into(), v.into());
                } else {
                    self.properties.insert(arg[2..].into(), "true".into());
                }
            } else if arg.starts_with("@") && arg.contains(":") {
                let (directive, value) = arg[1..].split_at(match arg.find(":") {
                    Some(v) => v,
                    None => {
                        continue;
                    }
                });
                let directive = &directive[0..directive.len() - 1];

                if !self.directives.contains_key(directive.into()) {
                    self.directives.insert(directive.into(), Vec::new());
                }

                self.directives
                    .get_mut(directive.into())
                    .unwrap()
                    .push(value.into());
            }
        }
    }
}
