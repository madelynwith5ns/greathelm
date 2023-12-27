use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    config, error,
    identify::NamespacedIdentifier,
    manifest::ProjectManifest,
    term::*,
    util::{self, copy_dir},
};

/**
 * Gets the path of the templates store..
*/
pub fn get_templates_path() -> PathBuf {
    return PathBuf::from_str(
        format!(
            "{}/templates",
            config::get_data_base_dir().to_str().unwrap()
        )
        .as_str(),
    )
    .unwrap();
}

/**
 * Gets the path of a specific template.
 * This does NOT check if the template exists or not.
 */
pub fn get_template_path(template: &NamespacedIdentifier) -> PathBuf {
    return PathBuf::from_str(
        format!(
            "{}/{}/{}",
            get_templates_path().display(),
            template.namespace.replace(".", "/"),
            template.identifier
        )
        .as_str(),
    )
    .unwrap();
}

pub fn generate_from_template(manifest: &ProjectManifest, template: String) {
    let dir = match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => {
            error!("Could not get current directory. Abort.");
            std::process::exit(1);
        }
    };

    let mut identifier: Option<NamespacedIdentifier> = None;
    if template.contains(".") && template.contains(":") {
        // probably a namespaced identifier
        identifier = Some(match NamespacedIdentifier::parse_text(&template) {
            Some(i) => i,
            None => {
                error!("Failed to parse provided namespaced identifier.");
                std::process::exit(1);
            }
        });
    } else {
        // probably not a namespaced identifier
        let aliases = manifest.get_aliases_map();
        for a in aliases.keys() {
            if &template == a {
                let target = aliases.get(a).unwrap();
                match NamespacedIdentifier::parse_text(target) {
                    Some(i) => {
                        identifier = Some(i);
                    }
                    None => {
                        warning!("Alias \x1bc{a}\x1br=\x1bc{target}\x1br did not point to a valid NamespacedIdentifier. Skipping.");
                    }
                }
            }
        }
    }

    let path: PathBuf;
    match identifier {
        Some(id) => {
            path = get_template_path(&id);
        }
        None => {
            error!("Could not resolve a valid identifier from your input \x1bc{template}\x1br.");
            error!("Please provide a namespaced identifier or a valid alias to one.");
            std::process::exit(1);
        }
    }
    if path.exists() {
        info!("Generating a project from template \x1bc{template}\x1br...");
    } else {
        error!("The requested template \x1bc{template}\x1br could not be found.");
        std::process::exit(1);
    }

    match copy_dir(&path, &dir, &vec![], false) {
        Ok(_) => {}
        Err(e) => {
            print_error_obj(Some(format!("Failed to copy directory.")), Box::new(e));
            std::process::exit(1);
        }
    };

    let mut tdef = ProjectManifest::new();
    let tdef_path = format!("{}/TemplateDef.ghm", path.display());
    let tdef_path = Path::new(tdef_path.as_str());
    if tdef_path.exists() {
        tdef.read_and_append(tdef_path);
    }

    let prompts = match tdef.directives.get("Prompt") {
        Some(s) => s.to_owned(),
        None => {
            vec![]
        }
    };

    let mut replace_in_files: HashMap<String, String> = HashMap::new();

    for p in prompts {
        if !p.contains(" ") {
            continue;
        }
        let mut segments = p.split(" ");
        let key = segments.nth(0).unwrap();

        let mut prompt: String = String::new();
        for s in segments {
            prompt.push_str(s);
            prompt.push_str(" ");
        }

        replace_in_files.insert(key.into(), question(prompt).replace("\n", ""));
        // remove the
        // newlines from
        // the response
    }

    util::run_on_all(&dir, &|p| {
        let mut str = match std::fs::read_to_string(&p) {
            Ok(s) => s,
            Err(_) => {
                return;
            }
        };

        for (k, v) in &replace_in_files {
            str = str.replace(k, v);
        }

        match std::fs::write(&p, str) {
            Ok(_) => {}
            Err(e) => {
                print_error_obj(Some("Failed to write a modified file.".into()), Box::new(e));
            }
        };
    })
    .unwrap();

    ok!("Finished generating project from template");
}
