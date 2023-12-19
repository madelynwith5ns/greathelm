use crate::{
    builder::dependency,
    term::{error, info},
};

use super::Action;

/**
 * Built-in (io.github.madelynwith5ns.greathelm:PackageScript) action for running a script within an
 * imported package. Requires a dependency-notation identifier at state.cli_args[2].
 */
pub struct PackageScript {}
impl PackageScript {
    pub fn create() -> Self {
        Self {}
    }
}

impl Action for PackageScript {
    fn get_name(&self) -> String {
        "PackageScript".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["pkgscript".into()]
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "PackageScript".into(),
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
                        let mut args: Vec<String> = state
                            .cli_args
                            .iter()
                            .skip(3)
                            .map(|f| f.to_owned())
                            .collect();
                        args.insert(0, "script".into());

                        let mut display_command: String = "".into();
                        display_command.push_str(
                            format!("{} script", std::env::current_exe().unwrap().display())
                                .as_str(),
                        );
                        for a in &args {
                            display_command.push_str(format!(" {a}").as_str());
                        }
                        info(format!("Resolved. Invoking \"{display_command}\""));

                        duct::cmd(std::env::current_exe().unwrap(), args)
                            .stderr_to_stdout()
                            .dir(p)
                            .run()
                            .ok();
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
