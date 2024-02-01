use std::{path::PathBuf, str::FromStr};

use crate::{
    identify::NamespacedIdentifier,
    store,
    term::*,
    version::{self, Version},
};

/**
 * Parse a dependency-notation identifier (something like `<identifier>@<version>`, `<identifier>`)
 * into a tuple of the identifier and (if present) version.
 */
pub fn parse_dependency_notation(notation: String) -> (NamespacedIdentifier, Option<Version>) {
    if notation.contains("@") {
        let spl = notation.split_once("@").unwrap();
        return (
            match NamespacedIdentifier::parse_text(&spl.0.into()) {
                Some(v) => v,
                None => {
                    error!("Failed to parse identifier.");
                    std::process::exit(1);
                }
            },
            Some(Version::parse(spl.1.into())),
        );
    } else {
        return (
            match NamespacedIdentifier::parse_text(&notation) {
                Some(v) => v,
                None => {
                    error!("Failed to parse identifier.");
                    std::process::exit(1);
                }
            },
            None,
        );
    }
}

/**
 * Returns a Vec of all versions of the specified package found in the
 * local store.
 */
pub fn get_all_versions(identifier: &NamespacedIdentifier) -> Vec<Version> {
    let path = store::get_path(&identifier);
    if !path.exists() {
        return vec![];
    }

    let mut versions = Vec::new();
    for ent in path.read_dir().unwrap() {
        let ent = ent.unwrap();
        let vtext = format!("{}", ent.file_name().to_string_lossy());
        if !vtext.contains("@") {
            continue;
        }
        let vtext = vtext.split_once("@").unwrap().1;
        let version = version::Version::parse(vtext.into());
        versions.push(version);
    }

    return versions;
}

/**
 * Attempts to resolve the Path (as a PathBuf) to the directory containing the
 * specified package (and version). If no version is passed, the latest version in the
 * store is used.
 * If there are no versions or the package is not found, this returns None.
 */
pub fn resolve_dependency(
    identifier: NamespacedIdentifier,
    version: Option<Version>,
) -> Option<PathBuf> {
    let path = store::get_path(&identifier);
    let path = PathBuf::from_str(
        format!(
            "{}{}",
            path.display(),
            match &version {
                Some(v) => {
                    format!("/@{v}")
                }
                None => {
                    "".into()
                }
            }
        )
        .as_str(),
    )
    .unwrap();

    if path.exists() && version.is_some() {
        return Some(path);
    } else if path.exists() {
        let mut versions = get_all_versions(&identifier);

        if versions.is_empty() {
            error!("Item \x1bc{identifier}\x1br was resolved, but there are no present versions!");
            return None;
        } else {
            versions.sort();
            versions.reverse();

            let v = versions.get(0).unwrap();
            let path = PathBuf::from_str(format!("{}/@{v}", path.display()).as_str()).unwrap();
            if path.exists() {
                return Some(path);
            } else {
                error!(
                    "Item \x1bc{identifier}\x1br was resolved, but the version folder is not present?"
                );
                return None;
            }
        }
    } else {
        error!("Item \x1bc{identifier}\x1br could not be resolved.");
        return None;
    }
}
