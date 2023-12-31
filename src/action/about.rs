use crate::{config, store, template, term::*};

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

    fn execute(&self, state: &crate::state::GreathelmState) {
        info!("== Build Information ==");
        info!("Name: \x1bc{}\x1br", env!("CARGO_PKG_NAME"));
        info!("Version: \x1bc{}\x1br", env!("CARGO_PKG_VERSION"));
        info!("== Install Information ==");
        match std::env::current_exe() {
            Ok(p) => {
                info!("Executable Path: \x1bc{}\x1br", p.display());
            }
            Err(_) => {
                error!("Executable Path: \x1bcUnknown\x1br");
            }
        };
        info!(
            "Config Directory: \x1bc{}/\x1br",
            config::get_config_base_dir().display()
        );
        info!(
            "Data Directory: \x1bc{}/\x1br",
            config::get_data_base_dir().display()
        );
        info!(
            "Local Store Path: \x1bc{}/\x1br",
            store::get_store_path().display()
        );
        info!(
            "Templates Path: \x1bc{}/\x1br",
            template::get_templates_path().display()
        );

        info!(
            "== Installed Plugins (\x1bc{}\x1br) ==",
            state.plugins.len()
        );
        for p in &state.plugins {
            info!("- \x1bc{}\x1br (\x1bc{}\x1br)", p.name, p.identifier);
            info!("  |- Version: \x1bcv{}\x1br", p.version);
            info!("  |- Vendor: \x1bc{}\x1br", p.vendor);
            info!("  |- Description: {}", p.description);
        }
        info!(
            "== Available Actions (\x1bc{}\x1br) ==",
            state.actions.len()
        );
        for a in &state.actions {
            info!(
                "- \x1bc{}\x1br (\x1bc{}\x1br)",
                a.get_name(),
                a.get_identifier()
            );
        }
        info!(
            "== Available Builders (\x1bc{}\x1br) ==",
            state.builders.len()
        );
        for b in &state.builders {
            info!(
                "- \x1bc{}\x1br (\x1bc{}\x1br)",
                b.get_name(),
                b.get_identifier()
            );
        }
        info!(
            "== Available Generators (\x1bc{}\x1br) ==",
            state.generators.len()
        );
        for g in &state.generators {
            info!(
                "- \x1bc{}\x1br (\x1bc{}\x1br)",
                g.get_name(),
                g.get_identifier()
            );
        }
    }
}
