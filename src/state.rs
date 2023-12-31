use crate::{
    action::Action, builder::ProjectBuilder, generator::ProjectGenerator,
    identify::NamespacedIdentifier, manifest::ProjectManifest, plugin::PluginInfo,
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

impl GreathelmState {
    pub fn get_builder_by_id(&self, id: &NamespacedIdentifier) -> Option<&Box<dyn ProjectBuilder>> {
        for b in &self.builders {
            if &b.get_identifier() == id {
                return Some(b);
            }
        }
        None
    }
    pub fn get_generator_by_id(
        &self,
        id: &NamespacedIdentifier,
    ) -> Option<&Box<dyn ProjectGenerator>> {
        for g in &self.generators {
            if &g.get_identifier() == id {
                return Some(g);
            }
        }
        None
    }
    pub fn get_action_by_id(&self, id: &NamespacedIdentifier) -> Option<&Box<dyn Action>> {
        for a in &self.actions {
            if &a.get_identifier() == id {
                return Some(a);
            }
        }
        None
    }
}
