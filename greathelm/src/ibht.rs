use std::{collections::HashMap, fmt::format, fs::ReadDir, path::Path};

use crate::term::{error, info, ok, warn};

pub fn write_ibht() {
    let hashes = gen_hashtable();

    info(format!("Writing IBHT to disk..."));

    let mut hashtable_file: String = "".into();

    for pair in hashes {
        hashtable_file.push_str(format!("{}={}\n", pair.0, pair.1).as_str());
    }

    match std::fs::write("IBHT.ghd", hashtable_file) {
        Ok(_) => {
            ok(format!("Finished writing IBHT!"));
        }
        Err(e) => {
            error(format!("Failed to write IBHT. Error is below:"));
            eprintln!("{}", e);
        }
    }
}

pub fn gen_hashtable() -> HashMap<String, String> {
    let mut hashes: HashMap<String, String> = HashMap::new();

    let srcdir = Path::new("src");
    if !srcdir.exists() {
        error(format!("There is no source directory. Abort."));
        return hashes;
    }
    if !srcdir.is_dir() {
        error(format!("src/ is not a directory. Abort."));
        return hashes;
    }

    match srcdir.read_dir() {
        Ok(iter) => {
            recurse_dir(iter, &mut hashes);
        }
        Err(_) => {
            error(format!("Failed to read src/. Abort."));
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
                            Err(_) => {
                                error(format!("Failed reading source tree."));
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
                info(format!("Hashed file {} as {:x}", f.path().display(), &hash));
                hashes.insert(f.path().display().to_string(), format!("{:x}", hash));
            }
            Err(_) => {
                continue;
            }
        }
    }
}

pub fn read_ibht() -> HashMap<String, String> {
    let ibht_path = Path::new("IBHT.ghd");
    if !ibht_path.exists() {
        return HashMap::new();
    }

    let ibht_file = match std::fs::read_to_string(ibht_path) {
        Ok(ibht) => ibht,
        Err(e) => {
            error(format!("Failed to read IBHT. Error is below:"));
            eprintln!("{}", e);
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
