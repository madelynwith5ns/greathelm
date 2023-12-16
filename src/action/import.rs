use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    identify::NamespacedIdentifier,
    manifest::ProjectManifest,
    store,
    term::{error, info, ok, warn},
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

        match copy_dir(&cd, &path, &state.manifest) {
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

fn copy_dir(from: &Path, to: &Path, manifest: &ProjectManifest) -> std::io::Result<()> {
    if manifest.directives.contains_key("StoreIgnore") {
        if manifest
            .directives
            .get("StoreIgnore")
            .unwrap()
            .contains(&format!("{}", from.file_name().unwrap().to_string_lossy()))
        {
            info(format!("Ignoring \"{}\"", from.display()));
            info(format!(
                "Due to rule \"@StoreIgnore {}\"",
                from.file_name().unwrap().to_string_lossy()
            ));
            return Ok(());
        }
    }

    std::fs::create_dir_all(&to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir(
                entry.path().as_path(),
                &to.join(entry.file_name()),
                manifest,
            )?;
        } else {
            match std::fs::copy(entry.path(), &to.join(entry.file_name())) {
                Ok(_) => {}
                Err(e) => {
                    if !manifest.get_bool_property("silent-fail", false) {
                        warn(format!("Skipping file \"{}\":", entry.path().display()));
                        warn(format!("{e}"));
                    }
                }
            };
        }
    }

    Ok(())
}
