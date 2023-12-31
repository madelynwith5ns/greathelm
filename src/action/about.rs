use crate::{script, term::*};

use super::Action;

/**
 * Built-in (io.github.greathelm.greathelm:About) action to get information about the current
 * Greathelm install.
 */
pub struct AboutAction {}
impl AboutAction {
    pub fn create() -> Self {
        Self {}
    }
}

impl Action for AboutAction {
    fn get_name(&self) -> String {
        "About".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["about".into()]
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.greathelm.greathelm".into(),
            identifier: "About".into(),
        }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {}
}
