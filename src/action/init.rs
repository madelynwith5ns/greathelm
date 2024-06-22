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

    fn get_identifier(&self) -> NamespacedIdentifier {
        NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "Initialize".into(),
        }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {
        // cwd
        let cdir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(_) => {
                error!("Current directory is invalid.");
                return;
            }
        };

        // project name, why does this default to "current-dir" if we cant get the current
        // directory? i don't remember but I'm just gonna roll with it.
        // fuck it, we ball.
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

        // I *do* remember why this defaults to Custom if not set. Scripts are cool.
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
            } else {
                match namespaced {
                    Some(ref n) => {
                        if n == &g.get_identifier() {
                            use_generator = Some(g);
                        }
                    }
                    None => {}
                }
            }
        }

        let generator = match use_generator {
            Some(generator) => generator,
            None => {
                error!("Could not find requested generator \x1bc{project_type}\x1br");
                error!("Are you missing a plugin?");
                std::process::exit(1);
            }
        };
        info!("Initializing current directory as Greathelm project \x1bc{project_name}\x1br");

        generator.generate(std::env::current_dir().unwrap());
    }
}
