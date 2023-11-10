use std::{path::Path, collections::HashMap, fmt::format};

use crate::term::{error, warn, info, ok};

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
        },
        Err(e) => {
            error(format!("Failed to write IBHT. Error is below:"));
            eprintln!("{}",e);
        },
    }
}

pub fn gen_hashtable() -> HashMap<String,String> {
    let mut hashes: HashMap<String,String> = HashMap::new();

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
            for f in iter {
                match f {
                    Ok(f) => {
                        if f.metadata().unwrap().is_dir() {
                            continue;
                        }

                        let contents = match std::fs::read_to_string(f.path()) {
                            Ok(contents) => { contents },
                            Err(_) => {
                                warn(format!("Failed to read a file in src! Attempting to continue without."));
                                continue;
                            },
                        };
                        let hash = md5::compute(contents);
                        info(format!("Hashed file {} as {:x}", f.path().display(), &hash));
                        hashes.insert(f.path().display().to_string(), format!("{:x}",hash));
                    },
                    Err(_) => {
                        warn(format!("Failed to read a file in src! Attempting to continue without."));
                        continue;
                    },
                }
            }
        },
        Err(_) => {
            error(format!("Failed to read src/. Abort."));
            return hashes;
        },
    }

    return hashes;
}

pub fn read_ibht() -> HashMap<String,String> {
    let ibht_file = match std::fs::read_to_string(Path::new("IBHT.ghd")) {
        Ok(ibht) => { ibht },
        Err(e) => {
            error(format!("Failed to read IBHT. Error is below:"));
            eprintln!("{}",e);
            std::process::exit(1);
        },
    };
    let mut table: HashMap<String,String> = HashMap::new();
    for ent in ibht_file.split("\n") {
        if !ent.contains("=") { continue; }
        let (f, h) = ent.split_once("=").unwrap();
        table.insert(f.into(),h.into());
    }

    return table;
}
