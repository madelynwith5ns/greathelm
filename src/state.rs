use crate::{
    action::Action, builder::ProjectBuilder, generator::ProjectGenerator, manifest::ProjectManifest,
};

/**
 * State struct. This is passed to all actions.
 */
pub struct GreathelmState {
    pub builders: Vec<Box<dyn ProjectBuilder>>,
    pub generators: Vec<Box<dyn ProjectGenerator>>,
    pub actions: Vec<Box<dyn Action>>,
    pub manifest: ProjectManifest,
    pub cli_args: Vec<String>,
}
