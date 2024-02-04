use crate::term::*;

use super::Action;

pub struct LSAction {}

impl LSAction {
    pub fn create() -> LSAction {
        LSAction {}
    }
}

impl Action for LSAction {
    fn get_name(&self) -> String {
        "ls".into()
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "ls".into(),
        }
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["ls".into()]
    }
    fn execute(&self, state: &crate::state::GreathelmState) {
        if state.cli_args.len() < 3 {
            error!("Please input a type of component to list.");
            error!("Example: greathelm ls action");
            return;
        }

        let lstype = state.cli_args.get(2).unwrap();

        match lstype.as_str() {
            "action" => {
                for a in &state.actions {
                    // we print with raw stdio to make it easier to
                    // pipe the results of greathelm ls into other
                    // programs (like fzf or dmenu).
                    println!(
                        "{};{};{}",
                        a.get_identifier(),
                        a.get_name(),
                        a.get_aliases().join(",")
                    );
                }
            }
            "builder" => {
                for b in &state.builders {
                    println!(
                        "{};{};{}",
                        b.get_identifier(),
                        b.get_name(),
                        b.get_aliases().join(",")
                    );
                }
            }
            "generator" => {
                for g in &state.generators {
                    println!(
                        "{};{};{}",
                        g.get_identifier(),
                        g.get_name(),
                        g.get_aliases().join(",")
                    );
                }
            }
            "plugin" => {
                for p in &state.plugins {
                    println!("{};{};{};{}", p.identifier, p.name, p.vendor, p.version);
                }
            }

            _ => {
                error!("Unrecognized ls type.");
            }
        }
    }
}
