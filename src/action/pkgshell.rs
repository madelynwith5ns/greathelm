use crate::{
    builder::dependency,
    term::{error, info},
};

use super::Action;

pub struct PackageShell {}
impl PackageShell {
    pub fn create() -> Self {
        Self {}
    }
}

impl Action for PackageShell {
    fn get_name(&self) -> String {
        "PackageShell".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["pkgshell".into(), "pkgsh".into()]
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "PackageShell".into(),
        }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {
        match state.cli_args.get(2) {
            Some(v) => {
                info(format!("Attempting to resolve {v}"));
                let (id, ver) = dependency::parse_dependency_notation(v.clone());
                let path = dependency::resolve_dependency(id.clone(), ver);
                match path {
                    Some(p) => {
                        duct::cmd!("sh").stderr_to_stdout().dir(p).run().ok();
                    }
                    None => {
                        error(format!("Could not resolve. Abort."));
                    }
                }
            }
            None => {
                error(format!("Please provide an identifier."));
            }
        }
    }
}
