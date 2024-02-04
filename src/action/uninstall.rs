use crate::{config, identify::NamespacedIdentifier, plugin, term::*};

use super::Action;

/**
 * Built-in (io.github.madelynwith5ns.greathelm:Uninstall) action to uninstall an installed plugin.
 */
pub struct UninstallAction {}
impl UninstallAction {
    pub fn create() -> Self {
        Self {}
    }
}

impl Action for UninstallAction {
    fn get_name(&self) -> String {
        "Uninstall".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["uninstall".into()]
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "Uninstall".into(),
        }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {
        'outer: {
            match state.cli_args.get(2) {
                Some(v) => {
                    let id = match NamespacedIdentifier::parse_text(v) {
                        Some(v) => v,
                        None => {
                            error!("Could not parse provided identifier.");
                            std::process::exit(1);
                        }
                    };

                    let plugins_dir = config::get_config_base_dir();
                    let plugins_dir = format!("{}/plugins", plugins_dir.display());

                    for f in std::fs::read_dir(plugins_dir).unwrap() {
                        let f = match f {
                            Ok(f) => f,
                            Err(_) => {
                                continue;
                            }
                        };
                        let pl = match unsafe { plugin::load_plugin_rs(f.path()) } {
                            Ok(p) => p,
                            Err(_) => {
                                continue;
                            }
                        };
                        if pl.identifier == id {
                            match std::fs::remove_file(f.path()) {
                                Ok(_) => {
                                    let func = pl.ghpi_uninstall;
                                    func();
                                    ok!("Successfully removed plugin.");
                                    break 'outer;
                                }
                                Err(_) => {
                                    error!("Failed to delete plugin file");
                                }
                            };
                        }
                    }

                    error!("Failed to remove plugin: No plugin installed with the provided identifier.");
                }
                None => {
                    error!("Please provide a plugin identifier.");
                }
            }
        }
    }
}
