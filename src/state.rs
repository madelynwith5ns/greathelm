use crate::{builder::ProjectBuilder, generator::ProjectGenerator, plugin::GreathelmPlugin, manifest::ProjectManifest};

pub struct GreathelmState {
    pub builders: Vec<Box<dyn ProjectBuilder>>,
    pub generators: Vec<Box<dyn ProjectGenerator>>,
    pub plugins: Vec<GreathelmPlugin>,
    pub manifest: ProjectManifest,
}

impl GreathelmState {
}
