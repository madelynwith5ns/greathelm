use std::path::Path;

use crate::term::print_error_obj;

pub fn create_directory(name: &str) {
    let path = Path::new(name);
    if !path.exists() {
        match std::fs::create_dir_all(path) {
            Ok(_) => {}
            Err(e) => {
                print_error_obj(Some("Failed to create directory.".into()), Box::new(e));
                std::process::exit(1);
            }
        }
    }
}

pub fn create_file(name: &str, contents: &str) {
    let path = Path::new(name);
    match std::fs::write(path, contents) {
        Ok(_) => {}
        Err(e) => {
            print_error_obj(Some("Failed to write file.".into()), Box::new(e));
            std::process::exit(1);
        }
    };
}
