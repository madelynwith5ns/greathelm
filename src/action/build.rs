use crate::term::*;
use std::path::{Path, PathBuf};

use crate::{builder::ProjectBuilder, identify::NamespacedIdentifier, script};

use super::Action;

/**
 * Built-in action (io.github.madelynwith5ns.greathelm:Build) for building a project.
 * This is where modules are built.
 * Calls validate() and then build() if that succeeds.
 * Also handles @Export directives.
 */
pub struct BuildAction {}
impl BuildAction {
    pub fn create() -> Self {
        Self {}
    }
}

impl Action for BuildAction {
    fn get_name(&self) -> String {
        "Build".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["build".into()]
    }
    fn get_identifier(&self) -> NamespacedIdentifier {
        NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "Build".into(),
        }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {
        // make sure we have working settings
        let project_name = state
            .manifest
            .get_string_property("Project-Name", "Unnamed Project");
        let project_type = state
            .manifest
            .get_string_property("Project-Type", "Unknown");
        // we cant build a project if it doesnt have a Project-Type.
        if project_type == "Unknown" {
            error!("This project does not have a set \x1bcProject-Type\x1br property.");
            std::process::exit(1);
        }

        // modules time
        info!("Building modules...");
        script::run_script("pre-modules", vec![]);
        for module in &state.manifest.get_modules() {
            module.build();
        }
        script::run_script("post-modules", vec![]);

        info!("Building project \x1bc{project_name}\x1br");

        // find the builder, fail out if ambiguous.
        let mut use_builder: Option<&Box<dyn ProjectBuilder>> = None;
        let namespaced = NamespacedIdentifier::parse_text(&project_type);
        for b in &state.builders {
            // check short names.
            if b.get_aliases().contains(&project_type.to_lowercase()) {
                // fail out if we already have a builder found
                if use_builder.is_some() {
                    error!(
                        "Builder name \x1bc{project_type}\x1br is ambiguous in your configuration."
                    );
                    error!("Please specify which one you would like to use either on the command line,");
                    error!("like so \x1bc--Project-Type=<full.namespaced:Identifier>\x1br");
                    error!("or in your project manifest.");
                    std::process::exit(1);
                } else {
                    use_builder = Some(b);
                }
            } else {
                // check NamespacedIdentifier.
                match namespaced {
                    Some(ref n) => {
                        if n == &b.get_identifier() {
                            use_builder = Some(b);
                        }
                    }
                    None => {} // if we dont have a NamespacedIdentifier we don't care.
                }
            }
        }

        // build!
        match use_builder {
            Some(builder) => {
                // create build dir if absent
                let path = Path::new("build");
                if !path.exists() {
                    if std::fs::create_dir(path).is_err() {
                        error!("Failed to create build directory. Abort.");
                        std::process::exit(1);
                    }
                }

                info!("Validating...");
                if builder.validate(&state.manifest) {
                    // run the validator
                    info!("Building...");
                    builder.build(&state.manifest);
                } else {
                    error!("Validating project failed.");
                }
            }
            None => {
                error!("Could not find the required builder \x1bc{project_type}\x1br.");
                error!("Are you missing a plugin?");
                std::process::exit(1);
            }
        }

        let export_dir = Path::new("export/");
        if !export_dir.exists() {
            match std::fs::create_dir_all(export_dir) {
                Ok(_) => {}
                Err(e) => {
                    print_error_obj(
                        Some("Failed to create export directory. Abort.".into()),
                        Box::new(e),
                    );
                    std::process::exit(1);
                }
            }
        }

        // exports
        let exports = match state.manifest.directives.get("Export") {
            Some(s) => s.to_owned(),
            None => {
                vec![]
            }
        };

        for export in exports {
            // @Export build/greathelm bin/greathelm
            // for example
            // exports build/greathelm to the export/bin/greathelm directory
            // this is to make packaging easier.
            let (export, mut to) = export.split_once(" ").unwrap_or((&export, ""));
            let export_name = match export.split("/").last() {
                Some(v) => v,
                None => "unnamed_export",
            };
            if to == "" {
                to = export_name;
            }
            // parent directories
            // cuz we might export to
            // etc/program/some/more/config/folders/for/some/reason/config.cfg
            let dest = PathBuf::from(format!("export/{to}"));
            match std::fs::create_dir_all(dest.parent().unwrap()) {
                Ok(_) => {}
                Err(_) => {
                    warning!("Failed exporting \x1bc{export}\x1br");
                    return;
                }
            };
            match std::fs::copy(export, dest) {
                Ok(_) => {
                    ok!("Successfully exported \x1bc{export}\x1br");
                }
                Err(_) => {
                    warning!("Failed exporting \x1bc{export}\x1br");
                }
            };
        }
    }
}
