use crate::{identify::NamespacedIdentifier, store, term::*};

use super::Action;

/**
 * Built-in (io.github.madelynwith5ns.greathelm:Remove) action to remove all versions of a package
 * from the local store.
 */
pub struct RemoveAction {}
impl RemoveAction {
    pub fn create() -> Self {
        Self {}
    }
}

impl Action for RemoveAction {
    fn get_name(&self) -> String {
        "Remove".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["remove".into()]
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "Remove".into(),
        }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {
        let identifier = match state.cli_args.get(2) {
            Some(v) => v,
            None => {
                error!("Please provide an identifier.");
                std::process::exit(1);
            }
        };
        info!("Attempting to resolve \x1bc{identifier}\x1br");
        let id = match NamespacedIdentifier::parse_text(identifier) {
            Some(v) => v,
            None => {
                error!("Could not resolve \x1bc{identifier}\x1br");
                std::process::exit(1);
            }
        };
        let path = store::get_path(&id);
        if path.exists() {
            match std::fs::remove_dir_all(path) {
                Ok(_) => {
                    ok!("Succeeded in removing from store.");
                }
                Err(_) => {
                    error!("Failed to remove from store.");
                }
            };
        } else {
            error!("{identifier} is not in store.");
        }
    }
}
