use std::path::Path;

use crate::term::*;

/**
 * Copies a directory recursively ignoring all path names ending with strings in `ignore`
 * and logging all failures unless `silent_fail == true`.
 */
pub fn copy_dir(
    from: &Path,
    to: &Path,
    ignore: &Vec<String>,
    silent_fail: bool,
) -> std::io::Result<()> {
    // check if we should ignore this specific directory
    for s in ignore {
        if format!("{}", from.display()).ends_with(s) {
            return Ok(());
        }
    }

    // create location
    std::fs::create_dir_all(&to)?;
    // copy all the things
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir(
                entry.path().as_path(),
                &to.join(entry.file_name()),
                ignore,
                silent_fail,
            )?;
        } else {
            // check if we should ignore this FILE
            for s in ignore {
                if format!("{}", entry.path().display()).ends_with(s) {
                    return Ok(());
                }
            }
            match std::fs::copy(entry.path(), &to.join(entry.file_name())) {
                Ok(_) => {}
                Err(e) => {
                    if !silent_fail {
                        print_error_obj(Some("Failed to copy a file.".into()), Box::new(e));
                    }
                }
            };
        }
    }

    Ok(())
}
