use std::path::Path;

use crate::{term::{error, info}, script, builder::ProjectBuilder, identify::NamespacedIdentifier};

use super::Action;

pub struct BuildAction { }
impl BuildAction {
    pub fn create() -> Self {
        Self {  }
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
        crate::identify::NamespacedIdentifier { namespace: "io.github.madelynwith5ns.greathelm".into(), identifier: "Build".into() }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {
        if !state.manifest.properties.contains_key("Project-Name") {
            error(format!("Project does not have a name!"));
            return;
        }
        if !state.manifest.properties.contains_key("Project-Type") {
            error(format!("Project does not have a type!"));
            return;
        }

        let project_name = state.manifest.properties.get("Project-Name").unwrap();
        let project_type = state.manifest.properties.get("Project-Type").unwrap();

        info(format!("Building modules..."));
        script::run_script("pre-modules", vec![]);
        for module in &state.manifest.modules {
            module.build();
        }
        script::run_script("post-modules", vec![]);

        info(format!("Building project \"{project_name}\""));

        // find the builder, fail out if ambiguous.
        let mut use_builder: Option<&Box<dyn ProjectBuilder>> = None;
        let namespaced = NamespacedIdentifier::parse_text(project_type);
        for b in &state.builders {
            if b.get_aliases().contains(&project_type.to_lowercase()) {
                if use_builder.is_some() {
                    error(format!(
                            "Builder name \"{}\" is ambiguous in your configuration.",
                            project_type
                            ));
                    error(format!("Please specify which one you would like to use either on the command line"));
                    error(format!(
                            "like so: --Project-Type=<full.namespaced:Identifier>"
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
                // create build dir if absent
                let path = Path::new("build");
                if !path.exists() {
                    match std::fs::create_dir(path) {
                        Ok(_) => {},
                        Err(_) => {
                            error(format!("Failed to create build directory. Abort."));
                            std::process::exit(1);
                        },
                    };
                }

                builder.build(&state.manifest);
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
}
