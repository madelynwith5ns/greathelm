use crate::{
    identify::NamespacedIdentifier,
    store,
    term::{error, info, ok},
};

use super::Action;

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
        match state.cli_args.get(2) {
            Some(v) => {
                info(format!("Attempting to resolve {v}"));
                let id = NamespacedIdentifier::parse_text(v);
                let path = store::get_path(&id);
                if path.exists() {
                    match std::fs::remove_dir_all(path) {
                        Ok(_) => {
                            ok(format!("Succeeded in removing from store."));
                        }
                        Err(_) => {
                            error(format!("Failed to remove from store."));
                        }
                    };
                } else {
                    error(format!("{} is not in store.", id.as_text()));
                }
            }
            None => {
                error(format!("Please provide an identifier."));
            }
        }
    }
}
