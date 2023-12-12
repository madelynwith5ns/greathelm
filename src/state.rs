use crate::{
    action::Action, builder::ProjectBuilder, generator::ProjectGenerator, manifest::ProjectManifest,
};

pub struct GreathelmState {
    pub builders: Vec<Box<dyn ProjectBuilder>>,
    pub generators: Vec<Box<dyn ProjectGenerator>>,
    pub actions: Vec<Box<dyn Action>>,
    pub manifest: ProjectManifest,
}
