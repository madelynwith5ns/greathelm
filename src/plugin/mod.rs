use std::{path::PathBuf, str::FromStr};

use crate::{
    action::Action, builder::ProjectBuilder, config, generator::ProjectGenerator,
    identify::NamespacedIdentifier, term::*,
};

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
#[repr(C)]
pub struct GreathelmPlugin {
    /**
     * This is used as the display name of the plugin.
     * Nothing else.
     */
    pub name: String,
    /**
     * All builders and generators within this plugin are expected to reside underneath this
     * identifier. For example, if the identifier is io.github.greathelm:Greathelm, all plugin
     * components are expected to be under io.github.greathelm.greathelm:<identifier here>.
     */
    pub identifier: NamespacedIdentifier,
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
    /**
     * This Vec contains all the plugin's actions.
     * Same as with builders and generators it is copied into an actual global Vec after plugin
     * init.
     */
    pub actions: Vec<Box<dyn Action>>,
    /**
     * List of all templates this plugin provides. If the template does not exist, the plugin will
     * be asked to create it.
     */
    pub provides_templates: Vec<NamespacedIdentifier>,

    // Plugin Function Pointers
    /**
     * Called when the plugin needs to create a template that does not exist.
     */
    pub ghpi_create_template: &'static dyn Fn(NamespacedIdentifier, PathBuf) -> bool,
    /**
     * Called when the plugin is `greathelm install`ed.
     */
    pub ghpi_first_time_setup: &'static dyn Fn(),
    /**
     * Called when the plugin is `greathelm uninstall`ed.
     * The plugin should remove all templates and clean up any other files it may have created.
     */
    pub ghpi_uninstall: &'static dyn Fn(),
}

/**
 * Loads all plugins in the (CONFIGROOT)/plugins directory. Called at startup. Do not call past
 * then.
 */
pub fn load_plugins() -> Vec<GreathelmPlugin> {
    let mut plugins = Vec::new();

    let plugins_dir = PathBuf::from_str(
        format!(
            "{}/plugins",
            config::get_config_base_dir().to_str().unwrap()
        )
        .as_str(),
    )
    .unwrap();

    for plugin_file in plugins_dir.read_dir().unwrap() {
        match plugin_file {
            Ok(f) => unsafe {
                match load_plugin_rs(f.path()) {
                    Ok(pl) => {
                        for t in &pl.provides_templates {
                            let tp = crate::template::get_template_path(t);
                            if !tp.exists() {
                                let func = pl.ghpi_create_template;
                                info!("Plugin-provided template \x1bc{t}\x1br does not exist. Creating it now...");
                                func(t.to_owned(), tp);
                            }
                        }

                        plugins.push(pl);
                    }
                    Err(_) => {}
                }
            },
            Err(_) => {}
        }
    }
    return plugins;
}

/**
 * Loads a plugin using the pure Rust interface.
 */
pub unsafe fn load_plugin_rs(path: PathBuf) -> Result<GreathelmPlugin, String> {
    let library = libloading::Library::new(format!("{}", path.display()));
    if library.is_err() {
        error!(
            "Failed to load plugin \x1bc{}\x1br. Could not load library.",
            path.display()
        );
        return Err("Failed to load".into());
    }
    let library = library.unwrap();
    let init_sym: libloading::Symbol<unsafe fn() -> GreathelmPlugin> =
        match library.get(b"GHPI_PluginInit") {
            Ok(s) => s,
            Err(_) => {
                error!(
                    "Loaded library \x1bc{}\x1br is not a Greathelm plugin or it is invalid.",
                    path.display()
                );
                return Err("Failed to load".into());
            }
        };
    let pl = init_sym();
    FORCEKEEPLOAD.push(library);
    return Ok(pl);
}
