use crate::{script, term::*};

use super::Action;

/**
 * Built-in (io.github.madelynwith5ns.greathelm:Script) action to run a script in the current
 * project.
 */
pub struct ScriptAction {}
impl ScriptAction {
    pub fn create() -> Self {
        Self {}
    }
}

impl Action for ScriptAction {
    fn get_name(&self) -> String {
        "Script".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["script".into()]
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "Script".into(),
        }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {
        match state.cli_args.get(2) {
            Some(v) => {
                script::run_script(
                    v,
                    state.cli_args.iter().skip(3).map(|f| f.clone()).collect(),
                );
            }
            None => {
                error!("Please provide a script name.");
            }
        }
    }
}
