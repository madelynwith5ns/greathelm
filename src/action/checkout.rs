use crate::{builder::dependency, identify::NamespacedIdentifier, term::*};

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
    fn get_identifier(&self) -> NamespacedIdentifier {
        NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "Checkout".into(),
        }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {
        let package = match state.cli_args.get(2) {
            Some(v) => v,
            None => {
                error!("Please provide a package.");
                std::process::exit(1);
            }
        };
        // resolve the package
        info!("Attempting to resolve {package}");
        let (id, ver) = dependency::parse_dependency_notation(package.clone());
        let path = dependency::resolve_dependency(id.clone(), ver);
        let path = match path {
            Some(p) => p,
            None => {
                error!("Could not resolve. Abort.");
                std::process::exit(1);
            }
        };
        info!("Checking out to current directory...");
        // current directory
        let dir = match std::env::current_dir() {
            Ok(v) => v,
            Err(_) => {
                error!("Could not get current directory.");
                std::process::exit(1);
            }
        };
        // copy the things
        match crate::util::copy_dir(&path, &dir, &vec![], false) {
            Ok(_) => {
                ok!("Finished checking out \x1bc{id}\x1br");
            }
            Err(e) => {
                error!("Failed to checkout.");
                eprintln!("{e}");
            }
        };
    }
}
