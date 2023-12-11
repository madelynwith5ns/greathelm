use crate::{builder::ProjectBuilder, generator::ProjectGenerator};

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
}
