use std::{collections::HashMap, fs::ReadDir, path::Path};

use crate::term::*;

/**
 * Hashes all files in the src/ directory and then writes the result to the IBHT.ghd file.
 */
pub fn write_ibht() {
    let hashes = gen_hashtable();

    let mut hashtable_file: String = "".into();

    for pair in hashes {
        hashtable_file.push_str(format!("{}={}\n", pair.0, pair.1).as_str());
    }

    match std::fs::write("IBHT.ghd", hashtable_file) {
        Ok(_) => {}
        Err(e) => {
            print_error_obj(Some("Failed to write IBHT.".into()), Box::new(e));
        }
    }
}

/**
 * Generates a table of files in src/ to their MD5 hashes.
 */
pub fn gen_hashtable() -> HashMap<String, String> {
    let mut hashes: HashMap<String, String> = HashMap::new();

    let srcdir = Path::new("src");
    if !srcdir.exists() {
        error!("There is no source directory. Abort.");
        return hashes;
    }
    if !srcdir.is_dir() {
        error!("src/ is not a directory. Abort.");
        return hashes;
    }

    match srcdir.read_dir() {
        Ok(iter) => {
            recurse_dir(iter, &mut hashes);
        }
        Err(e) => {
            print_error_obj(Some("Failed to read src".into()), Box::new(e));
            return hashes;
        }
    }

    return hashes;
}

fn recurse_dir(dir: ReadDir, hashes: &mut HashMap<String, String>) {
    for f in dir {
        match f {
            Ok(f) => {
                if f.metadata().unwrap().is_dir() {
                    recurse_dir(
                        match std::fs::read_dir(f.path()) {
                            Ok(dir) => dir,
                            Err(e) => {
                                print_error_obj(Some("Failed to read src".into()), Box::new(e));
                                std::process::exit(1);
                            }
                        },
                        hashes,
                    );
                }
                let contents = match std::fs::read_to_string(f.path()) {
                    Ok(contents) => contents,
                    Err(_) => {
                        continue;
                    }
                };
                let hash = md5::compute(contents);
                hashes.insert(f.path().display().to_string(), format!("{:x}", hash));
            }
            Err(_) => {
                continue;
            }
        }
    }
}

/**
 * Reads IBHT.ghd from disk into a HashMap of files within src/ to their MD5 hashes.
 */
pub fn read_ibht() -> HashMap<String, String> {
    let ibht_path = Path::new("IBHT.ghd");
    if !ibht_path.exists() {
        return HashMap::new();
    }

    let ibht_file = match std::fs::read_to_string(ibht_path) {
        Ok(ibht) => ibht,
        Err(e) => {
            print_error_obj(Some("Failed to read IBHT.".into()), Box::new(e));
            std::process::exit(1);
        }
    };
    let mut table: HashMap<String, String> = HashMap::new();
    for ent in ibht_file.split("\n") {
        if !ent.contains("=") {
            continue;
        }
        let (f, h) = ent.split_once("=").unwrap();
        table.insert(f.into(), h.into());
    }

    return table;
}
