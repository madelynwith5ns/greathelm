use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    script, subprocess,
    term::{error, info},
};

#[derive(Clone)]
pub struct Module {
    pub module_name: String, // name of the module
    // this is also the subfolder
    // within modules/ in which this
    // module resides.
    pub files: HashMap<String, String>, // map of paths within the parent project
                                        // paths within the module which should be
                                        // grabbed.
}

impl Module {
    pub fn build(&self) {
        info(format!("Module \"{}\"", self.module_name));
        for f in self.files.keys() {
            info(format!(
                "|-> Provides \"{f}\" from \"{}\"",
                self.files.get(f).unwrap()
            ));
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
                                    error(format!("Failed getting file \"{f}\" from module \"{}\": Failed to copy", self.module_name));
                                }
                            }
                        } else {
                            match std::fs::copy(&path, Path::new(f)) {
                                Ok(_) => {}
                                Err(_) => {
                                    error(format!("Failed getting file \"{f}\" from module \"{}\": Failed to copy", self.module_name));
                                }
                            };
                        }
                    }
                }
                Err(_) => {
                    error(format!(
                        "Failed getting file \"{f}\" from module \"{}\": File not present.",
                        self.module_name
                    ));
                }
            }
        }

        script::run_script("postfetch-module", vec![self.module_name.clone()]);
    }
}
