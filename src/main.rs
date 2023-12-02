use std::{collections::HashMap, path::Path};

use term::ok;

use crate::term::{error, info, warn};

mod builder;
mod generator;
mod ibht;
mod manifest;
mod module;
mod script;
mod term;

fn main() {
    if std::env::args().len() <= 1 {
        println!("Usage: greathelm <action> [args]");
        return;
    }
    let args: Vec<String> = std::env::args().collect();

    let action = match args.get(1) {
        Some(arg) => arg,
        None => {
            panic!("Insufficient arguments but check was bypassed. How?")
        }
    };

    let mut flags = HashMap::new();

    if args.len() >= 3 {
        for arg in &args {
            if arg.starts_with("--") && arg.contains("=") {
                let flag: String = arg.split_once("--").unwrap().1.into();
                let flag: String = flag.split_once("=").unwrap().0.into();
                let value: String = arg.split_once("=").unwrap().1.into();
                flags.insert(flag, value);
            }
        }
    }

    match action.as_str() {
        "init" => {
            let cdir = match std::env::current_dir() {
                Ok(dir) => dir,
                Err(_) => {
                    error(format!("Current directory is invalid."));
                    return;
                }
            };

            let project_name: String;

            if flags.contains_key("project-name") {
                project_name = flags.get("project-name").unwrap().clone();
            } else {
                let fname = match cdir.file_name() {
                    Some(fname) => fname.to_string_lossy(),
                    None => {
                        error(format!("Current directory is invalid."));
                        return;
                    }
                };
                project_name = fname.to_string();
            }

            let mut project_type: String = "C".into();

            if flags.contains_key("project-type") {
                project_type =
                    get_project_type_from_aliases(flags.get("project-type").unwrap().clone());
            }

            if project_type == "Unknown" {
                warn(format!("Unknown project type!"));
                warn(format!("Defaulting to \"C\""));
                project_type = "C".into();
            } else {
                info(format!("Project type is \"{}\"", project_type));
            }

            info(format!(
                "Initializing current directory as Greathelm project \"{}\"",
                project_name
            ));

            generator::generate(project_type.clone(), cdir);

            info(format!("Writing IBHT stub..."));
            match std::fs::write("IBHT.ghd", "\n") {
                Ok(_) => {
                    ok(format!("Blank IBHT has been written successfully."));
                }
                Err(e) => {
                    error(format!("Failed to write a blank IBHT. Error is below:"));
                    eprintln!("{}", e);
                }
            };
        }

        "build" => {
            let manifest_path = Path::new("Project.ghm");
            if !manifest_path.exists() {
                error(format!(
                    "Could not find Project.ghm in the current directory. Try 'greathelm init'"
                ));
                return;
            }
            let mut manifest = manifest::read_manifest(manifest_path);

            if !manifest.properties.contains_key("Project-Name") {
                error(format!("Project does not have a name!"));
                return;
            }
            if !manifest.properties.contains_key("Project-Type") {
                error(format!("Project does not have a type!"));
                return;
            }

            for f in flags.keys() {
                manifest
                    .properties
                    .insert(f.clone(), flags.get(f).unwrap().clone());
            }

            let project_name = manifest.properties.get("Project-Name").unwrap();
            let project_type = manifest.properties.get("Project-Type").unwrap();

            if get_project_type_from_aliases(project_type.clone()) == "Unknown" {
                error(format!("Project is of an unknown type."));
                return;
            }

            info(format!("Building modules..."));
            script::run_script("pre-modules", vec![]);
            for module in &manifest.modules {
                module.build();
            }
            script::run_script("post-modules", vec![]);

            info(format!("Building project \"{project_name}\""));
            builder::build(manifest);
        }

        _ => {}
    }
}

pub fn get_project_type_from_aliases(text: String) -> String {
    match text.to_lowercase().as_str() {
        "c" => "C".into(),
        "custom" | "non-greathelm" | "buildscripts" => "Custom".into(),
        _ => return "Unknown".into(),
    }
}
