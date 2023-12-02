use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

use crate::term::{error, info};

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

        let module_root = PathBuf::from_str(&format!("modules/{}", self.module_name)).unwrap();
        // smaller greathelm, lesserhelm if you will
        let greathelm_subprocess = match Command::new(std::env::current_exe().unwrap())
            .current_dir(module_root)
            .arg("build")
            .spawn()
        {
            Ok(o) => o,
            Err(e) => {
                error(format!("Failed to build module \"{}\"", self.module_name));
                eprintln!("{e}");
                return;
            }
        }
        .wait()
        .unwrap();

        if !greathelm_subprocess.success() {
            error(format!("Module \"{}\" failed to build.", self.module_name));
            return;
        }

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
                        match std::fs::copy(path, Path::new(f)) {
                            Ok(_) => {}
                            Err(_) => {
                                error(format!("Failed getting file \"{f}\" from module \"{}\": Failed to copy", self.module_name));
                            }
                        };
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
    }
}
