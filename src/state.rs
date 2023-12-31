use crate::{
    action::Action, builder::ProjectBuilder, generator::ProjectGenerator,
    manifest::ProjectManifest, plugin::PluginInfo,
};

/**
 * State struct. This is passed to all actions.
 */
pub struct GreathelmState {
    pub builders: Vec<Box<dyn ProjectBuilder>>,
    pub generators: Vec<Box<dyn ProjectGenerator>>,
    pub actions: Vec<Box<dyn Action>>,
    pub manifest: ProjectManifest,
    pub plugins: Vec<PluginInfo>,
    pub cli_args: Vec<String>,
}
