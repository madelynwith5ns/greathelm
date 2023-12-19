use std::{path::PathBuf, str::FromStr};

use crate::{
    identify::NamespacedIdentifier,
    store,
    term::error,
    version::{self, Version},
};

pub fn parse_dependency_notation(notation: String) -> (NamespacedIdentifier, Option<Version>) {
    if notation.contains("@") {
        let spl = notation.split_once("@").unwrap();
        return (
            NamespacedIdentifier::parse_text(&spl.0.into()),
            Some(Version::parse(spl.1.into())),
        );
    } else {
        return (NamespacedIdentifier::parse_text(&notation), None);
    }
}

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
                Some(s) => {
                    format!("/@{}", s.as_text())
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

        if versions.is_empty() {
            error(format!(
                "Item {} was resolved, but there are no present versions!",
                identifier.as_text()
            ));
            return None;
        } else {
            versions.sort();
            versions.reverse();

            let v = versions.get(0).unwrap();
            let path =
                PathBuf::from_str(format!("{}/@{}", path.display(), v.as_text()).as_str()).unwrap();
            if path.exists() {
                return Some(path);
            } else {
                error(format!(
                    "Item {} was resolved, but the version folder is not present?",
                    identifier.as_text()
                ));
                return None;
            }
        }
    } else {
        error(format!(
            "Item {} could not be resolved.",
            identifier.as_text()
        ));
        return None;
    }
}
