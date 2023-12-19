use std::{path::PathBuf, str::FromStr};

use crate::{
    identify::NamespacedIdentifier,
    store,
    term::{error, info, ok},
    version::Version,
};

use super::Action;

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

    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "Import".into(),
        }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {
        let namespace = state
            .manifest
            .get_string_property("Project-Namespace", "unnamespaced");
        if namespace == "unnamespaced" {
            error(format!(
                "Project does not have a Project-Namespace. Cannot be imported."
            ));
            return;
        }
        let name = state
            .manifest
            .get_string_property("Project-Name", "unnamed");
        if name == "unnamed" {
            error(format!(
                "Project does not have a Project-Name. Cannot be imported."
            ));
            return;
        }
        let version = state
            .manifest
            .get_string_property("Project-Version", "unversioned");
        if version == "unversioned" {
            error(format!(
                "Project does not have a Project-Version. Cannot be imported"
            ));
        }
        let version = Version::parse(version);
        let identifier = NamespacedIdentifier {
            namespace,
            identifier: name,
        };
        let path = store::get_path(&identifier);
        let path =
            PathBuf::from_str(&format!("{}/@{}", path.display(), version.as_text())).unwrap();

        info(format!("Importing project to {}", path.display()));
        if path.exists() {
            info(format!("Clearing old copy..."));
            match std::fs::remove_dir_all(&path) {
                Ok(_) => {}
                Err(e) => {
                    error(format!(
                        "Failed to remove old copy of this project in the store."
                    ));
                    error(format!("{e}"));
                    return;
                }
            };
        }
        match std::fs::create_dir_all(&path) {
            Ok(_) => {}
            Err(_) => {
                error(format!("Failed to create path in store."));
                return;
            }
        };

        let cd = match std::env::current_dir() {
            Ok(v) => v,
            Err(_) => {
                error(format!("Failed to get current directory."));
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

        match crate::util::copy_dir(&cd, &path, &ignore, false) {
            Ok(_) => {
                ok(format!(
                    "Successfully imported project \"{}@{}\"",
                    identifier.as_text(),
                    version.as_text()
                ));
            }
            Err(e) => {
                error(format!("Failed to import project."));
                eprintln!("{}", e);
            }
        };
    }
}
