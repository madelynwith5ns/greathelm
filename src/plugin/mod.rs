use std::{path::PathBuf, str::FromStr};

use crate::{builder::ProjectBuilder, generator::ProjectGenerator, config, term::{error, ok}, action::Action};

// this is just here to keep things loaded because libloading automatically
// unloads them when dropped.
static mut FORCEKEEPLOAD: Vec<libloading::Library> = Vec::new();

/**
 * Greathelm is still alpha software.
 * This interface CAN and WILL change before 1.0.
 * After 1.0 you can expect GreathelmPlugin to remain
 * as it is. Future versions of the interface will be new
 * structs.
 */
pub struct GreathelmPlugin {
    /**
     * This is used as the display name of the plugin.
     * Nothing else.
     */
    pub name: String,
    /**
     * All builders and generators within this plugin
     * are expected to reside within this namespace.
     */
    pub namespace: String,
    /**
     * This Vec contains all the plugin's builders.
     * Its contents will be copied into the global
     * Greathelm builders store after plugin initialization.
     * You cannot change these after the plugin initializes.
     */
    pub builders: Vec<Box<dyn ProjectBuilder>>,
    /**
     * This Vec contains all the plugin's generators.
     * Same as with builders it is copied into the global
     * generators store after plugin init.
     */
    pub generators: Vec<Box<dyn ProjectGenerator>>,
    pub actions: Vec<Box<dyn Action>>,
}

pub fn load_plugins() -> Vec<GreathelmPlugin> {
    let mut plugins = Vec::new();

    let plugins_dir = PathBuf::from_str(
        format!("{}/plugins", config::get_config_base_dir()
                .to_str()
                .unwrap())
        .as_str())
        .unwrap();

    for plugin_file in plugins_dir.read_dir().unwrap() {
        match plugin_file {
            Ok(f) => {
                unsafe {
                    let library = libloading::Library::new(format!("{}",f.path().display()));
                    if library.is_err() {
                        error(format!("Failed to load plugin \"{}\". Could not load library.", f.path().display()));
                        continue;
                    }
                    let library = library.unwrap();
                    let init_sym: libloading::Symbol<unsafe fn() -> GreathelmPlugin> = match library.get(b"GHPI_PluginInit") {
                        Ok(s) => { s },
                        Err(_) => {
                            error(format!("Loaded library \"{}\" is not a Greathelm plugin or it is invalid.", f.path().display()));
                            continue;
                        },
                    };
                    plugins.push(init_sym());
                    FORCEKEEPLOAD.push(library);
                }
            },
            Err(_) => {},
        } 
    }

    ok(format!("Successfully loaded {} plugins.", plugins.len()));
    return plugins;
}
