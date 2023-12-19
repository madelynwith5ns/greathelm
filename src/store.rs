use std::{path::PathBuf, str::FromStr};

use crate::{config, identify::NamespacedIdentifier};

/**
 * Gets the path of the local store.
 */
pub fn get_store_path() -> PathBuf {
    return PathBuf::from_str(
        format!("{}/store", config::get_data_base_dir().to_str().unwrap()).as_str(),
    )
    .unwrap();
}

/**
 * Gets the path at which `identifier` would be located in the store.
 */
pub fn get_path(identifier: &NamespacedIdentifier) -> PathBuf {
    let mut path = get_store_path();
    path.push(
        format!("{}", identifier)
            .replace(".", "/")
            .replace(":", "/"),
    );
    return path;
}
