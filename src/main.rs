use std::{collections::HashMap, path::Path};

use builder::ProjectBuilder;
use generator::ProjectGenerator;
use identify::NamespacedIdentifier;
use term::ok;

use crate::term::{error, info};

mod builder;
mod generator;
mod ibht;
mod identify;
mod manifest;
mod module;
mod plugin;
mod script;
mod term;
mod config;

fn main() { 
    config::ensure_config_dirs();

    let mut builders: Vec<Box<dyn ProjectBuilder>> = Vec::new();
    let mut generators: Vec<Box<dyn ProjectGenerator>> = Vec::new();

    // builtins
    builders.push(Box::new(builder::c::CBuilder::create()));
    builders.push(Box::new(builder::cpp::CPPBuilder::create()));
    builders.push(Box::new(builder::custom::CustomBuilder::create()));

    generators.push(Box::new(generator::c::CGenerator::create()));
    generators.push(Box::new(generator::cpp::CPPGenerator::create()));
    generators.push(Box::new(generator::custom::CustomGenerator::create()));

    // load plugins here..
    let plugins = plugin::load_plugins();
    for plugin in plugins {
        for b in plugin.builders {
            builders.push(b);
        }
        for g in plugin.generators {
            generators.push(g);
        }
    }

    if std::env::args().len() <= 1 {
        println!("Usage: greathelm <action> [args]");
        std::process::exit(0);
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

            let project_type = match flags.get("project-type") {
                Some(t) => t.clone(),
                None => "io.github.madelynwith5ns.greathelm:Custom".into(),
            };

            let mut use_generator: Option<&Box<dyn ProjectGenerator>> = None;
            let namespaced = NamespacedIdentifier::parse_text(&project_type);
            for g in &generators {
                if g.get_aliases().contains(&project_type.to_lowercase()) {
                    if use_generator.is_some() {
                        error(format!(
                            "Builder name \"{}\" is ambiguous in your configuration.",
                            project_type
                        ));
                        error(format!("Please specify which one you would like to use either on the command line"));
                        error(format!(
                            "like so: --Project-Type=<full namespaced identifier>"
                        ));
                        error(format!("or in your project manifest."));
                        std::process::exit(1);
                    } else {
                        use_generator = Some(g);
                    }
                } else if namespaced.namespace != "unnamespaced" && g.get_identifier() == namespaced
                {
                    use_generator = Some(g);
                }
            }

            match use_generator {
                Some(generator) => {
                    info(format!(
                        "Initializing current directory as Greathelm project \"{}\"",
                        project_name
                    ));

                    generator.generate(std::env::current_dir().unwrap());

                    if generator.should_make_ibht_stub() {
                        info(format!(
                            "Generator requested an IBHT stub. Writing IBHT stub..."
                        ));
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
                }
                None => {
                    error(format!(
                        "Could not find requested generator \"{project_type}\""
                    ));
                    error(format!("Are you missing a plugin?"));
                }
            }
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

            info(format!("Building modules..."));
            script::run_script("pre-modules", vec![]);
            for module in &manifest.modules {
                module.build();
            }
            script::run_script("post-modules", vec![]);

            info(format!("Building project \"{project_name}\""));

            // find the builder, fail out if ambiguous.
            let mut use_builder: Option<&Box<dyn ProjectBuilder>> = None;
            let namespaced = NamespacedIdentifier::parse_text(project_type);
            for b in &builders {
                if b.get_aliases().contains(&project_type.to_lowercase()) {
                    if use_builder.is_some() {
                        error(format!(
                            "Builder name \"{}\" is ambiguous in your configuration.",
                            project_type
                        ));
                        error(format!("Please specify which one you would like to use either on the command line"));
                        error(format!(
                            "like so: --Project-Type=<full namespaced identifier>"
                        ));
                        error(format!("or in your project manifest."));
                        std::process::exit(1);
                    } else {
                        use_builder = Some(b);
                    }
                } else if namespaced.namespace != "unnamespaced" && b.get_identifier() == namespaced
                {
                    use_builder = Some(b);
                }
            }

            // build!
            match use_builder {
                Some(builder) => {
                    builder.build(&manifest);
                }
                None => {
                    error(format!(
                        "Could not find the required builder \"{project_type}\"."
                    ));
                    error(format!("Are you missing a plugin?"));
                    std::process::exit(1);
                }
            }
        }

        _ => {}
    }
}
