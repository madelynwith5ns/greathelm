use std::{path::PathBuf, str::FromStr};

use crate::{config, plugin, term::*};

use super::Action;

/**
 * Built-in (com.mw5ns.greathelm:PluginInstall) action to install a plugin.
 */
pub struct PluginInstallAction {}
impl PluginInstallAction {
    pub fn create() -> Self {
        Self {}
    }
}

impl Action for PluginInstallAction {
    fn get_name(&self) -> String {
        "PluginInstall".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["plugininstall".into(), "pluginstall".into()]
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "com.mw5ns.greathelm".into(),
            identifier: "PluginInstall".into(),
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

                if plugin_file.exists() {
                    info!("You already have this plugin installed.");
                    let r = question("Would you like to update it? (y/N)".into()).to_lowercase();
                    if !r.starts_with("y") {
                        ok!("Exitting.");
                        return;
                    }
                    match std::fs::remove_file(&plugin_file) {
                        Ok(_) => {
                            ok!("Old copy removed.");
                        }
                        Err(_) => {
                            error!("There was an error removing the old copy of the plugin!");
                            return;
                        }
                    }
                }

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
                error!("Please provide a plugin file.");
            }
        }
    }
}
