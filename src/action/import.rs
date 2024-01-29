use std::{path::PathBuf, str::FromStr};

use crate::{identify::NamespacedIdentifier, script, store, term::*, version::Version};

use super::Action;

/**
 * Built-in (io.github.madelynwith5ns.greathelm:Import) action for importing a project into the
 * global store.
 * Does NOT call build.
 * Calls the pre-import script.
 */
pub struct ImportAction {}
impl ImportAction {
    pub fn create() -> Self {
        Self {}
    }
}

impl Action for ImportAction {
    fn get_name(&self) -> String {
        "Import".into()
    }

    fn get_aliases(&self) -> Vec<String> {
        vec!["import".into()]
    }

    fn get_identifier(&self) -> NamespacedIdentifier {
        NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "Import".into(),
        }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {
        // get all of our settings
        let namespace = state
            .manifest
            .get_string_property("Project-Namespace", "unnamespaced");
        if namespace == "unnamespaced" {
            error!("Project does not have a Project-Namespace. Cannot be imported.");
            return;
        }
        let name = state
            .manifest
            .get_string_property("Project-Name", "unnamed");
        if name == "unnamed" {
            error!("Project does not have a Project-Name. Cannot be imported.");
            return;
        }
        let version = state
            .manifest
            .get_string_property("Project-Version", "unversioned");
        if version == "unversioned" {
            error!("Project does not have a Project-Version. Cannot be imported");
        }
        // make the version and identifier structs
        let version = Version::parse(version);
        let identifier = NamespacedIdentifier {
            namespace,
            identifier: name,
        };

        // get the final path
        let path = store::get_path(&identifier);
        let path = PathBuf::from_str(&format!("{}/@{version}", path.display())).unwrap();

        // people run scripts or something i dont know
        script::run_script("pre-import", vec![format!("{}", path.display())]);

        // import time
        info!("Importing project to \x1bc{}\x1br", path.display());
        if path.exists() {
            // delete the old stuff
            info!("Clearing old copy...");
            match std::fs::remove_dir_all(&path) {
                Ok(_) => {}
                Err(e) => {
                    print_error_obj(
                        Some("Failed to remove old copy of this project in the store.".into()),
                        Box::new(e),
                    );
                    return;
                }
            };
        }

        // create the destination
        match std::fs::create_dir_all(&path) {
            Ok(_) => {}
            Err(_) => {
                error!("Failed to create path in store.");
                return;
            }
        };

        // current dir
        let cd = match std::env::current_dir() {
            Ok(v) => v,
            Err(_) => {
                error!("Failed to get current directory.");
                return;
            }
        };

        // @StoreIgnore directives
        let mut ignore = Vec::new();
        match state.manifest.directives.get("StoreIgnore") {
            Some(si) => {
                for i in si {
                    ignore.push(i.to_owned());
                }
            }
            None => {}
        }

        // finally actually import the stuff
        match crate::util::copy_dir(&cd, &path, &ignore, false) {
            Ok(_) => {
                ok!("Successfully imported project \x1bc{identifier}@{version}\x1br");
            }
            Err(e) => {
                print_error_obj(Some("Failed to import project.".into()), Box::new(e));
            }
        };
    }
}
