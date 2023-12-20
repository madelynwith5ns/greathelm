use crate::{generator::ProjectGenerator, identify::NamespacedIdentifier, term::*};

use super::Action;

/**
 * Built-in (io.github.madelynwith5ns.greathelm:Initialize) action for creating a project in the
 * current directory.
 */
pub struct InitAction {}
impl InitAction {
    pub fn create() -> Self {
        Self {}
    }
}

impl Action for InitAction {
    fn get_name(&self) -> String {
        "Initialize".into()
    }

    fn get_aliases(&self) -> Vec<String> {
        vec!["init".into()]
    }

    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "Initialize".into(),
        }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {
        let cdir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(_) => {
                error!("Current directory is invalid.");
                return;
            }
        };

        let project_name: String = state.manifest.get_string_property(
            "project-name",
            match cdir.file_name() {
                Some(v) => match v.to_str() {
                    Some(v) => v,
                    None => "current-dir",
                },
                None => "current-dir",
            },
        );

        let project_type = state
            .manifest
            .get_string_property("project-type", "io.github.madelynwith5ns.greathelm:Custom");

        let mut use_generator: Option<&Box<dyn ProjectGenerator>> = None;
        let namespaced = NamespacedIdentifier::parse_text(&project_type);
        for g in &state.generators {
            if g.get_aliases().contains(&project_type.to_lowercase()) {
                if use_generator.is_some() {
                    error!("Generator name \x1bc{project_type}\x1br is ambiguous in your configuration.");
                    error!("Please specify which one you would like to use on the command line");
                    error!("like so: \x1bc--project-type=<full.namespaced:Identifier>\x1br");
                    std::process::exit(1);
                } else {
                    use_generator = Some(g);
                }
            } else if namespaced.namespace != "unnamespaced" && g.get_identifier() == namespaced {
                use_generator = Some(g);
            }
        }

        match use_generator {
            Some(generator) => {
                info!(
                    "Initializing current directory as Greathelm project \x1bc{project_name}\x1br"
                );

                generator.generate(std::env::current_dir().unwrap());

                if generator.should_make_ibht_stub() {
                    info!("Generator requested an IBHT stub. Writing IBHT stub...");
                    match std::fs::write("IBHT.ghd", "\n") {
                        Ok(_) => {
                            ok!("Blank IBHT has been written successfully.");
                        }
                        Err(e) => {
                            print_error_obj(Some("Failed to write IBHT.".into()), Box::new(e));
                        }
                    };
                }
            }
            None => {
                error!("Could not find requested generator \x1bc{project_type}\x1br");
                error!("Are you missing a plugin?");
            }
        }
    }
}
