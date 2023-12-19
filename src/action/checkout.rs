use crate::{
    builder::dependency,
    term::{error, info, ok},
};

use super::Action;

/**
 * Built-in action (io.github.madelynwith5ns.greathelm:Checkout) for copying a project out of the
 * global store once it has been imported.
 * Requires a dependency-notation-form identifier at state.cli_args[2].
 */
pub struct CheckoutAction {}
impl CheckoutAction {
    pub fn create() -> Self {
        Self {}
    }
}

impl Action for CheckoutAction {
    fn get_name(&self) -> String {
        "Checkout".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["checkout".into()]
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "Checkout".into(),
        }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {
        match state.cli_args.get(2) {
            Some(v) => {
                info(format!("Attempting to resolve {v}"));
                let (id, ver) = dependency::parse_dependency_notation(v.clone());
                let path = dependency::resolve_dependency(id.clone(), ver);
                match path {
                    Some(p) => {
                        info(format!("Checking out to current directory..."));
                        let dir = match std::env::current_dir() {
                            Ok(v) => v,
                            Err(_) => {
                                error(format!("Could not get current directory."));
                                std::process::exit(1);
                            }
                        };
                        match crate::util::copy_dir(&p, &dir, &vec![], false) {
                            Ok(_) => {
                                ok(format!("Finished checking out {id}"));
                            }
                            Err(e) => {
                                error(format!("Failed to checkout."));
                                eprintln!("{e}");
                            }
                        };
                    }
                    None => {
                        error(format!("Could not resolve. Abort."));
                    }
                }
            }
            None => {
                error(format!("Please provide an identifier."));
            }
        }
    }
}
