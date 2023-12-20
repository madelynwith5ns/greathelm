use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{script, subprocess, term::*};

/**
 * Defines a Module specified with the @Module directive.
 */
#[derive(Clone)]
pub struct Module {
    /**
     * Name of the module. This is also its path within modules/ in the project it is included
     * from.
     */
    pub module_name: String,
    /**
     * Map of paths within the parent project to paths within the module project which should be
     * copied.
     */
    pub files: HashMap<String, String>,
}

impl Module {
    /**
     * Builds this module and copies its files to their locations in the parent project.
     */
    pub fn build(&self) {
        info!("Module \x1bc{}\x1br", self.module_name);
        for f in self.files.keys() {
            info!(
                "|-> Provides \x1bc{f}\x1br from \x1bc{}\x1br",
                self.files.get(f).unwrap()
            );
        }

        script::run_script("prebuild-module", vec![self.module_name.clone()]);
        let module_root = PathBuf::from_str(&format!("modules/{}", self.module_name)).unwrap();
        // smaller greathelm, lesserhelm if you will
        subprocess::build_project(&module_root);
        script::run_script("postbuild-module", vec![self.module_name.clone()]);

        for f in self.files.keys() {
            let path = PathBuf::from_str(&format!(
                "modules/{}/{}",
                self.module_name,
                self.files.get(f).unwrap()
            ))
            .unwrap();
            match path.try_exists() {
                Ok(exists) => {
                    if exists {
                        if path.is_dir() {
                            match crate::util::copy_dir(&path, Path::new(f), &vec![], false) {
                                Ok(_) => {}
                                Err(_) => {
                                    error!("Failed getting file \x1bc{f}\x1br from module \x1bc{}\x1br: Failed to copy", self.module_name);
                                }
                            }
                        } else {
                            match std::fs::copy(&path, Path::new(f)) {
                                Ok(_) => {}
                                Err(_) => {
                                    error!("Failed getting file \x1bc{f}\x1br from module \x1bc{}\x1br: Failed to copy", self.module_name);
                                }
                            };
                        }
                    }
                }
                Err(_) => {
                    error!(
                        "Failed getting file \x1bc{f}\x1br from module \x1bc{}\x1br: File not present.",
                        self.module_name
                    );
                }
            }
        }

        script::run_script("postfetch-module", vec![self.module_name.clone()]);
    }
}
