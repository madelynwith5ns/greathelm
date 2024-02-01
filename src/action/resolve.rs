use crate::{builder::dependency, identify::NamespacedIdentifier, term::*};

use super::Action;

/**
 * Built-in action (io.github.madelynwith5ns.greathelm:Resolve) resolving a dependency
 * and viewing all versions of it in the local store.
 */
pub struct ResolveAction {}
impl ResolveAction {
    pub fn create() -> Self {
        Self {}
    }
}

impl Action for ResolveAction {
    fn get_name(&self) -> String {
        "Resolve".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["resolve".into()]
    }
    fn get_identifier(&self) -> NamespacedIdentifier {
        NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "Resolve".into(),
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
        info!("Parsing notation \x1bc{package}\x1br");
        let (id, _) = dependency::parse_dependency_notation(package.clone());

        info!("Querying versions...");
        let versions = dependency::get_all_versions(&id);
        if versions.is_empty() {
            warning!("There are no versions of this package present!");
            return;
        }
        for v in versions {
            info!("Version: @\x1bc{v}\x1br (include via: \x1bc{id}@{v}\x1br)");
        }
    }
}
