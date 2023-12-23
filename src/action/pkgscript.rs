use crate::{builder::dependency, identify::NamespacedIdentifier, term::*};

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
    fn get_identifier(&self) -> NamespacedIdentifier {
        NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "PackageScript".into(),
        }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {
        let package = match state.cli_args.get(2) {
            Some(v) => v,
            None => {
                error!("Please provide an identifier.");
                std::process::exit(1);
            }
        };
        info!("Attempting to resolve {package}");
        let (id, ver) = dependency::parse_dependency_notation(package.clone());
        let path = dependency::resolve_dependency(id.clone(), ver);
        let path = match path {
            Some(p) => p,
            None => {
                error!("Could not resolve. Abort.");
                std::process::exit(1);
            }
        };
        let exec = match std::env::current_exe() {
            Ok(e) => e,
            Err(_) => {
                error!("Could not resolve the current executable path. Abort.");
                std::process::exit(1);
            }
        };
        let mut args: Vec<String> = state
            .cli_args
            .iter()
            .skip(3)
            .map(|f| f.to_owned())
            .collect();
        args.insert(0, "script".into());

        let mut display_command: String = "".into();
        display_command.push_str(format!("{} script", exec.display()).as_str());
        for a in &args {
            display_command.push_str(format!(" {a}").as_str());
        }
        info!("Resolved. Invoking \x1bc{display_command}\x1br");

        duct::cmd(exec, args)
            .stderr_to_stdout()
            .dir(path)
            .run()
            .ok();
    }
}
