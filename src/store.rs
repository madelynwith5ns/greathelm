use std::{path::PathBuf, str::FromStr};

use crate::{config, identify::NamespacedIdentifier};

pub fn get_store_path() -> PathBuf {
    return PathBuf::from_str(
        format!("{}/store", config::get_data_base_dir().to_str().unwrap()).as_str(),
    )
    .unwrap();
}

pub fn get_path(identifier: &NamespacedIdentifier) -> PathBuf {
    let mut path = get_store_path();
    path.push(identifier.as_text().replace(".", "/").replace(":", "/"));
    return path;
}
