use std::{path::PathBuf, str::FromStr};

use crate::{config, plugin, term::*};

use super::Action;

/**
 * Built-in (io.github.madelynwith5ns.greathelm:Install) action to install a plugin.
 */
pub struct InstallAction {}
impl InstallAction {
    pub fn create() -> Self {
        Self {}
    }
}

impl Action for InstallAction {
    fn get_name(&self) -> String {
        "Install".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["install".into()]
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "Install".into(),
        }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {
        match state.cli_args.get(2) {
            Some(v) => {
                let path = PathBuf::from_str(v).unwrap();
                if !path.exists() {
                    error!("Could not access provided file.");
                }
                let pl = match unsafe { plugin::load_plugin_rs(path.clone()) } {
                    Ok(p) => p,
                    Err(_) => {
                        error!("Failed installing plugin.");
                        std::process::exit(1);
                    }
                };

                let fts = pl.ghpi_first_time_setup;

                info!("Running plugin setup...");
                fts();

                info!("Installing...");
                let config_dir = config::get_config_base_dir();
                let plugin_file = format!(
                    "{}/plugins/{}",
                    config_dir.display(),
                    path.file_name().unwrap().to_str().unwrap()
                );
                let plugin_file = PathBuf::from_str(plugin_file.as_str()).unwrap();
                match std::fs::copy(path, plugin_file) {
                    Ok(_) => {
                        ok!("Successfully installed \x1bc{}\x1br", pl.identifier);
                    }
                    Err(_) => {
                        error!("Failed to install plugin.");
                    }
                };
            }
            None => {
                error!("Please provide a script name.");
            }
        }
    }
}
