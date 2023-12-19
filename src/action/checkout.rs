use std::path::Path;

use crate::{
    builder::dependency,
    term::{error, info, ok},
};

use super::Action;

pub struct CheckoutAction {}
impl CheckoutAction {
    pub fn create() -> Self {
        Self {}
    }
}

impl Action for CheckoutAction {
    fn get_name(&self) -> String {
        "Checkout".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["checkout".into()]
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "Script".into(),
        }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {
        match state.cli_args.get(2) {
            Some(v) => {
                info(format!("Attempting to resolve {v}"));
                let (id, ver) = dependency::parse_dependency_notation(v.clone());
                let path = dependency::resolve_dependency(id.clone(), ver);
                match path {
                    Some(p) => {
                        info(format!("Checking out to current directory..."));
                        let dir = match std::env::current_dir() {
                            Ok(v) => v,
                            Err(_) => {
                                error(format!("Could not get current directory."));
                                std::process::exit(1);
                            }
                        };
                        match copy_dir(&p, &dir) {
                            Ok(_) => {
                                ok(format!("Finished checking out {}", id.as_text()));
                            }
                            Err(e) => {
                                error(format!("Failed to checkout."));
                                eprintln!("{e}");
                            }
                        };
                    }
                    None => {
                        error(format!("Could not resolve. Abort."));
                    }
                }
            }
            None => {
                error(format!("Please provide an identifier."));
            }
        }
    }
}

fn copy_dir(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(&to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir(entry.path().as_path(), &to.join(entry.file_name()))?;
        } else {
            match std::fs::copy(entry.path(), &to.join(entry.file_name())) {
                Ok(_) => {}
                Err(e) => {
                    error(format!(
                        "Failed to copy {}",
                        entry.file_name().to_string_lossy()
                    ));
                    eprintln!("{e}");
                }
            };
        }
    }
    Ok(())
}
