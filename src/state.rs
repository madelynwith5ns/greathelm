use crate::{builder::ProjectBuilder, generator::ProjectGenerator, manifest::ProjectManifest, action::Action};

pub struct GreathelmState {
    pub builders: Vec<Box<dyn ProjectBuilder>>,
    pub generators: Vec<Box<dyn ProjectGenerator>>,
    pub actions: Vec<Box<dyn Action>>,
    pub manifest: ProjectManifest,
}
