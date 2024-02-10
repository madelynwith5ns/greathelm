use crate::term::*;
use std::path::Path;

use crate::{builder::ProjectBuilder, identify::NamespacedIdentifier};

use super::Action;

/**
 * Built-in action (io.github.madelynwith5ns.greathelm:Clean) for cleaning up after a project.
 */
pub struct CleanAction {}
impl CleanAction {
    pub fn create() -> Self {
        Self {}
    }
}

impl Action for CleanAction {
    fn get_name(&self) -> String {
        "Clean".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["clean".into()]
    }
    fn get_identifier(&self) -> NamespacedIdentifier {
        NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "Clean".into(),
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
        // we cant clean a project if it doesnt have a Project-Type.
        if project_type == "Unknown" {
            error!("This project does not have a set \x1bcProject-Type\x1br property.");
            std::process::exit(1);
        }

        info!("Cleaning up after \x1bc{project_name}\x1br");

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

        // clean!
        match use_builder {
            Some(builder) => {
                let path = Path::new("build");
                if !path.exists() {
                    warning!("No build directory. Nothing to clean.");
                    std::process::exit(1);
                }

                builder.cleanup(&state.manifest);
            }
            None => {
                error!("Could not find the required builder \x1bc{project_type}\x1br.");
                error!("Are you missing a plugin?");
                std::process::exit(1);
            }
        }
    }
}
