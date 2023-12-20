use crate::term::*;
use std::path::Path;

use crate::{builder::ProjectBuilder, identify::NamespacedIdentifier, script};

use super::Action;

/**
 * Built-in action (io.github.madelynwith5ns.greathelm:Build) for building a project.
 * This is where modules are built.
 * Calls validate() and then build() if that succeeds.
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
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "Build".into(),
        }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {
        let project_name = state
            .manifest
            .get_string_property("Project-Name", "UnnamedProject");
        let project_type = state
            .manifest
            .get_string_property("Project-Type", "Unknown");

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
            if b.get_aliases().contains(&project_type.to_lowercase()) {
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
            } else if namespaced.namespace != "unnamespaced" && b.get_identifier() == namespaced {
                use_builder = Some(b);
            }
        }

        // build!
        match use_builder {
            Some(builder) => {
                // create build dir if absent
                let path = Path::new("build");
                if !path.exists() {
                    match std::fs::create_dir(path) {
                        Ok(_) => {}
                        Err(_) => {
                            error!("Failed to create build directory. Abort.");
                            std::process::exit(1);
                        }
                    };
                }

                info!("Validating...");
                if builder.validate(&state.manifest) {
                    info!("Building...");
                    builder.build(&state.manifest);
                } else {
                    info!("Validating project failed.");
                }
            }
            None => {
                info!("Could not find the required builder \"{project_type}\".");
                info!("Are you missing a plugin?");
                std::process::exit(1);
            }
        }
    }
}
