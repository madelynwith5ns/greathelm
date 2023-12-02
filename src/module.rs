use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

use crate::{
    script,
    term::{error, info},
};

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
                            match copy_dir(&path, Path::new(f)) {
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

fn copy_dir(from: impl AsRef<Path>, to: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir(entry.path(), to.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), to.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}
